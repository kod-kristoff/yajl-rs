use std::{
    env::args,
    io::{self, Read},
};

use ::libc;
use yajlir::{Parser, ParserOptions};

fn usage(progname: Option<&str>) {
    eprintln!(
        "{}: validate json from stdin",
        progname.unwrap_or("json-verify")
    );
    eprintln!(
        "\nusage: json_verify [options]\n    -c allow comments\n    -q quiet mode\n    -s verify a stream of multiple json entities\n    -u allow invalid utf8 inside strings\n"
    );
    std::process::exit(1);
}
unsafe fn main_0(_argc: libc::c_int, _argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut stat;
    let mut rd: usize;

    let mut filedata: [u8; 65536] = [0; 65536];
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int;
    let options = ParserOptions::default();
    let argv: Vec<String> = std::env::args().collect();
    for a in argv.iter().skip(1) {
        match a.as_str() {
            "-q" => quiet = 1,
            "-c" => {
                options.allow_comments = true;
            }
            "-u" => {
                options.dont_validate_strings = true;
            }
            "-s" => {
                options.allow_multiple_values = true;
            }
            c => {
                eprintln!("unrecognized option: '{c}'\n");
                usage(args().next().as_deref());
            }
        }
    }

    let mut parser = Parser::with_options(options);
    let mut stdin = io::stdin();
    loop {
        rd = match stdin.read(&mut filedata) {
            Ok(rd) => rd,
            Err(err) => {
                if quiet == 0 {
                    eprintln!("error encountered on file read: {err:?}");
                }
                return 1;
            }
        };
        retval = 0;
        if rd == 0 {
            break;
        } else {
            filedata[rd] = 0;
            stat = parser.parse(&filedata);
            // stat = parser.parse(filedata.as_mut_ptr(), rd);
            if stat != Status::Ok {
                break;
            }
        }
    }
    stat = parser.complete_parse();
    if stat != Status::Ok {
        if quiet == 0 {
            let str: *mut libc::c_uchar = parser.get_error(true, filedata.as_mut_ptr(), rd);

            libc::write(
                libc::STDERR_FILENO,
                str as *mut libc::c_void,
                libc::strlen(str as *const libc::c_char),
            );
            // eprintln!(
            //     "{}",
            //     String::from_utf8_lossy(unsafe { &*(str as *const [u8]) })
            // );
            parser.free_error(str);
        }
        retval = 1 as libc::c_int;
    }
    // Parser::free(hand);
    if quiet == 0 {
        println!("JSON is {}", if retval != 0 { "invalid" } else { "valid" },);
    }
    retval
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr()) as i32)
    }
}
