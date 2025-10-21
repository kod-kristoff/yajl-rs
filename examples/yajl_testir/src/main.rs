use std::{env, fs::File, io};

use yajlir::{ParseError, Parser, ParserOptions};

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
    let mut parser = Parser::new(options);

    let mut file_data = vec![0; buf_size];
    let mut file: Box<dyn io::BufRead> = if let Some(file_path) = file_name {
        let file = File::open(file_path).expect("an existing file");
        let reader = io::BufReader::new(file);
        Box::new(reader)
    } else {
        Box::new(io::stdin().lock())
    };
    let mut rd;
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
            parser.parse(&file_data[..rd])?;
        }
    }
    parser.complete_parse()?;
    println!("memory leaks: 0");
    Ok(())
}

fn usage(progname: &str) -> ! {
    eprintln!("usage:  {} [options]\nParse input from stdin as JSON and output parsing details to stdout\n   -b  set the read buffer size\n   -c  allow comments\n   -g  allow *g*arbage after valid JSON text\n   -m  allows the parser to consume multiple JSON values\n       from a single string separated by whitespace\n   -p  partial JSON documents should not cause errors\n",
        progname);

    std::process::exit(1)
}
