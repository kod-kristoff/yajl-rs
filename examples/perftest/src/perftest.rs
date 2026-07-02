#![allow(non_camel_case_types)]
use ::libc;
use libc::STDERR_FILENO;
use yajl::{
    parser::{yajl_callbacks, Parser},
    yajl_alloc::yajl_alloc_funcs,
    ParserOption, Status,
};

use self::documents::{doc_size, get_doc, num_docs};

mod documents;

pub type yajl_handle = *mut Parser;

unsafe extern "C" fn mygettime() -> libc::c_double {
    let mut now: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    #[cfg(not(target_os = "macos"))]
    libc::gettimeofday(&mut now, std::ptr::null_mut::<libc::timezone>());
    #[cfg(target_os = "macos")]
    libc::gettimeofday(&mut now, std::ptr::null_mut::<libc::c_void>());
    now.tv_sec as libc::c_double + now.tv_usec as libc::c_double / 1000000.0f64
}
unsafe extern "C" fn run(validate_utf8: bool) -> libc::c_int {
    let mut times: usize = 0;
    let starttime = mygettime();
    loop {
        let now: libc::c_double = mygettime();
        if now - starttime >= 3 as libc::c_int as libc::c_double {
            break;
        }
        let mut i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            let hand: yajl_handle = Parser::alloc(
                std::ptr::null::<yajl_callbacks>(),
                std::ptr::null_mut::<yajl_alloc_funcs>(),
                std::ptr::null_mut::<libc::c_void>(),
            );
            let mut stat;
            let parser = unsafe { &mut *hand };
            parser.config(ParserOption::DontValidateStrings, !validate_utf8);
            let mut d = get_doc(times % num_docs());
            while !d.is_empty() {
                stat = parser.parse(d[0]);
                if stat != Status::Ok {
                    break;
                }
                d = &d[1..];
            }
            stat = parser.complete_parse();
            if stat != Status::Ok {
                let str: *mut libc::c_uchar = parser.get_error(true, d[0]);
                libc::write(
                    STDERR_FILENO,
                    // b"%s\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    str as *const libc::c_char as *const libc::c_void,
                    libc::strlen(str as *const libc::c_char),
                );
                parser.free_error(str);
                return 1 as libc::c_int;
            }
            Parser::free(hand);
            times += 1;
            i += 1;
        }
    }
    let mut all_units: [*const libc::c_char; 4] = [
        b"B/s\0" as *const u8 as *const libc::c_char,
        b"KB/s\0" as *const u8 as *const libc::c_char,
        b"MB/s\0" as *const u8 as *const libc::c_char,
        std::ptr::null_mut::<libc::c_char>() as *const libc::c_char,
    ];
    let mut units: *mut *const libc::c_char = all_units.as_mut_ptr();
    let mut avg_doc_size: usize = 0;
    let now_0 = mygettime();
    let mut i_0 = 0;
    while i_0 < num_docs() {
        avg_doc_size = (avg_doc_size).wrapping_add(doc_size(i_0));
        i_0 += 1;
    }
    avg_doc_size /= num_docs();
    let mut throughput = (times * avg_doc_size) as libc::c_double / (now_0 - starttime);
    while !(*units.offset(1 as libc::c_int as isize)).is_null()
        && throughput > 1024 as libc::c_int as libc::c_double
    {
        throughput /= 1024 as libc::c_int as libc::c_double;
        units = units.offset(1);
    }
    libc::printf(
        b"Parsing speed: %g %s\n\0" as *const u8 as *const libc::c_char,
        throughput,
        *units,
    );
    0 as libc::c_int
}
unsafe fn main_0() -> libc::c_int {
    libc::printf(
        b"-- speed tests determine parsing throughput given %d different sample documents --\n\0"
            as *const u8 as *const libc::c_char,
        num_docs(),
    );
    libc::printf(b"With UTF8 validation:\n\0" as *const u8 as *const libc::c_char);
    let mut rv = run(true);
    if rv != 0 as libc::c_int {
        return rv;
    }
    libc::printf(b"Without UTF8 validation:\n\0" as *const u8 as *const libc::c_char);
    rv = run(false);
    rv
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
