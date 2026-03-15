use std::{any::Any, env, fs::File, io};

use yajlir::{CallbackStatus, ParseError, Parser, ParserCallbacks, ParserOptions};

fn main() -> Result<(), ParseError> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let argc = args.len();
    let mut options = ParserOptions::default();
    let mut buf_size: usize = 2048;
    let mut file_name: Option<&str> = None;
    while i < argc {
        if args[i] == "-c" {
            options.allow_comments(true);
        } else if args[i] == "-b" {
            i += 1;
            if i >= argc {
                usage(&args[0]);
            }
            buf_size = match args[i].parse() {
                Ok(x) => x,
                Err(err) => {
                    eprintln!(
                        "-b requires an integer argument. '{}' is invalid: {}",
                        args[i], err
                    );
                    usage(&args[0]);
                }
            }
        } else if args[i] == "-g" {
            options.allow_trailing_garbage(true);
        } else if args[i] == "-m" {
            options.allow_multiple_values(true);
        } else if args[i] == "-p" {
            options.allow_partial_values(true);
        } else {
            file_name = Some(&args[i]);
            break;
        }
        i += 1;
    }
    let callbacks = ParserCallbacks {
        null: Some(test_null),
        boolean: Some(test_bool),
        start_map: Some(test_start_map),
        end_map: Some(test_end_map),
        start_array: Some(test_start_array),
        end_array: Some(test_end_array),
        ..Default::default()
    };
    let mut ctx = ();
    let mut parser = Parser::new(Some(&callbacks), &mut ctx, options);

    let mut file_data = vec![0; buf_size];
    let mut file: Box<dyn io::BufRead> = if let Some(file_path) = file_name {
        let file = File::open(file_path).expect("an existing file");
        let reader = io::BufReader::new(file);
        Box::new(reader)
    } else {
        Box::new(io::stdin().lock())
    };
    let mut rd;
    let mut res;
    loop {
        rd = match file.read(&mut file_data) {
            Ok(rd) => rd,
            Err(err) => {
                eprintln!(
                    "error reading from '{}': {}",
                    file_name.unwrap_or("stdin"),
                    err
                );
                std::process::exit(2);
            }
        };
        if rd == 0 {
            break;
        } else {
            res = parser.parse(&file_data[..rd]);
            if res.is_err() {
                break;
            }
        }
    }
    res = parser.complete_parse();
    if let Err(error) = res {
        eprintln!("{}", error);
    }
    println!("memory leaks: 0");
    Ok(())
}

fn usage(progname: &str) -> ! {
    eprintln!("usage:  {} [options]\nParse input from stdin as JSON and output parsing details to stdout\n   -b  set the read buffer size\n   -c  allow comments\n   -g  allow *g*arbage after valid JSON text\n   -m  allows the parser to consume multiple JSON values\n       from a single string separated by whitespace\n   -p  partial JSON documents should not cause errors\n",
        progname);

    std::process::exit(1)
}

fn test_null(_ctx: &mut dyn Any) -> CallbackStatus {
    println!("null");
    CallbackStatus::Continue
}

fn test_bool(_ctx: &mut dyn Any, val: bool) -> CallbackStatus {
    println!("bool: {}", if val { "true" } else { "false" });
    CallbackStatus::Continue
}

fn test_start_map(_ctx: &mut dyn Any) -> CallbackStatus {
    println!("map open '{{'");
    CallbackStatus::Continue
}

fn test_end_map(_ctx: &mut dyn Any) -> CallbackStatus {
    println!("map close '}}'");
    CallbackStatus::Continue
}

fn test_start_array(_ctx: &mut dyn Any) -> CallbackStatus {
    println!("array open '['");
    CallbackStatus::Continue
}

fn test_end_array(_ctx: &mut dyn Any) -> CallbackStatus {
    println!("array close ']'");
    CallbackStatus::Continue
}
