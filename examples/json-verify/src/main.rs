use std::{
    env::args,
    io::{self, Read},
};

use ::libc;
use yajl::{
    yajl::{yajl_complete_parse, yajl_config, yajl_free_error, yajl_get_error, yajl_handle},
    yajl_alloc::yajl_alloc_funcs,
    yajl_parse,
    yajl_parser::{yajl_callbacks, yajl_handle_t},
    yajl_status::{yajl_status, yajl_status_ok},
    yajl_tree::{yajl_allow_comments, yajl_allow_multiple_values, yajl_dont_validate_strings},
};

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
    let mut stat: yajl_status;
    let mut rd: usize;
    let hand: yajl_handle;
    let mut filedata: [libc::c_uchar; 65536] = [0; 65536];
    let mut quiet: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int;
    hand = yajl_handle_t::alloc(
        0 as *const yajl_callbacks,
        0 as *mut yajl_alloc_funcs,
        0 as *mut libc::c_void,
    );
    let argv: Vec<String> = std::env::args().collect();
    for a in argv.iter().skip(1) {
        match a.as_str() {
            "-q" => quiet = 1,
            "-c" => {
                yajl_config(hand, yajl_allow_comments, 1 as libc::c_int);
            }
            "-u" => {
                yajl_config(hand, yajl_dont_validate_strings, 1 as libc::c_int);
            }
            "-s" => {
                yajl_config(hand, yajl_allow_multiple_values, 1 as libc::c_int);
            }
            c => {
                eprintln!("unrecognized option: '{c}'\n");
                usage(args().nth(0).as_deref());
            }
        }
    }
    // while a < argc
    //     && *(*argv.offset(a as isize)).offset(0 as libc::c_int as isize) as libc::c_int
    //         == '-' as i32
    //     && libc::strlen(*argv.offset(a as isize)) > 1 as libc::c_int as libc::c_ulong
    // {
    //     let mut i: libc::c_uint = 0;
    //     i = 1 as libc::c_int as libc::c_uint;
    //     while (i as libc::c_ulong) < libc::strlen(*argv.offset(a as isize)) {
    //         match *(*argv.offset(a as isize)).offset(i as isize) as libc::c_int {
    //             113 => {
    //                 quiet = 1 as libc::c_int;
    //             }
    //             99 => {}
    //             117 => {}
    //             115 => {}
    //             _ => {
    //                 fprintf(
    //                     stderr,
    //                     b"unrecognized option: '%c'\n\n\0" as *const u8 as *const libc::c_char,
    //                     *(*argv.offset(a as isize)).offset(i as isize) as libc::c_int,
    //                 );
    //                 // usage(*argv.offset(0 as libc::c_int as isize));
    //                 usage(args().nth(0).as_deref());
    //             }
    //         }
    //         i = i.wrapping_add(1);
    //     }
    //     a += 1;
    // }
    // if a < argc {
    //     // usage(*argv.offset(0 as libc::c_int as isize));
    //     usage(args().nth(0).as_deref());
    // }
    let mut stdin = io::stdin();
    loop {
        // rd = libc::fread(
        //     fileData.as_mut_ptr() as *mut libc::c_void,
        //     1 as libc::c_int as libc::c_ulong,
        //     (::core::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
        //         .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        //     stdin,
        // );
        rd = match stdin.read(&mut filedata) {
            Ok(rd) => rd,
            Err(err) => {
                if quiet == 0 {
                    eprintln!("error encountered on file read: {err:?}");
                }
                return 1;
            }
        };
        retval = 0 as libc::c_int;
        if rd == 0 {
            //     if feof(stdin) == 0 {
            //         if quiet == 0 {
            //             libc::fprintf(
            //                 stderr,
            //                 b"error encountered on file read\n\0" as *const u8 as *const libc::c_char,
            //             );
            //         }
            //         retval = 1 as libc::c_int;
            //     }
            break;
        } else {
            filedata[rd as usize] = 0 as libc::c_int as libc::c_uchar;
            stat = yajl_parse(hand, filedata.as_mut_ptr(), rd);
            if stat != yajl_status_ok {
                break;
            }
        }
    }
    stat = yajl_complete_parse(hand);
    if stat != yajl_status_ok {
        if quiet == 0 {
            let str: *mut libc::c_uchar =
                yajl_get_error(hand, 1 as libc::c_int, filedata.as_mut_ptr(), rd);
            // libc::fprintf(
            //     stderr,
            //     b"%s\0" as *const u8 as *const libc::c_char,
            //     str as *const libc::c_char,
            // );
            libc::write(
                libc::STDERR_FILENO,
                str as *mut libc::c_void,
                libc::strlen(str as *const libc::c_char),
            );
            // eprintln!(
            //     "{}",
            //     String::from_utf8_lossy(unsafe { &*(str as *const [u8]) })
            // );
            yajl_free_error(hand, str);
        }
        retval = 1 as libc::c_int;
    }
    yajl_handle_t::free(hand);
    if quiet == 0 {
        println!("JSON is {}", if retval != 0 { "invalid" } else { "valid" },);
    }
    return retval;
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
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
