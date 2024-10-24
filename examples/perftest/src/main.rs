#![allow(non_camel_case_types)]
use ::libc;
use libc::STDERR_FILENO;
use yajl::{
    yajl::{yajl_complete_parse, yajl_config, yajl_free_error, yajl_get_error},
    yajl_alloc::yajl_alloc_funcs,
    yajl_option::yajl_dont_validate_strings,
    yajl_parse,
    yajl_parser::{yajl_callbacks, yajl_handle_t},
    yajl_status::{yajl_status, yajl_status_ok},
};

use self::documents::{doc_size, get_doc, num_docs};

mod documents;

pub type yajl_handle = *mut yajl_handle_t;

unsafe extern "C" fn mygettime() -> libc::c_double {
    let mut now: libc::timeval = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    libc::gettimeofday(&mut now, 0 as *mut libc::timezone);
    return now.tv_sec as libc::c_double + now.tv_usec as libc::c_double / 1000000.0f64;
}
unsafe extern "C" fn run(validate_utf8: libc::c_int) -> libc::c_int {
    let mut times: usize = 0;
    let starttime = mygettime();
    loop {
        let now: libc::c_double = mygettime();
        if now - starttime >= 3 as libc::c_int as libc::c_double {
            break;
        }
        let mut i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            let hand: yajl_handle = yajl_handle_t::alloc(
                0 as *const yajl_callbacks,
                0 as *mut yajl_alloc_funcs,
                0 as *mut libc::c_void,
            );
            let mut stat: yajl_status;
            yajl_config(
                hand,
                yajl_dont_validate_strings,
                if validate_utf8 != 0 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                },
            );
            let mut d = get_doc(times % num_docs());
            while !(*d).is_null() {
                stat = yajl_parse(hand, *d as *mut libc::c_uchar, libc::strlen(*d));
                if stat as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
                    break;
                }
                d = d.offset(1);
            }
            stat = yajl_complete_parse(hand);
            if stat != yajl_status_ok {
                let str: *mut libc::c_uchar = yajl_get_error(
                    hand,
                    1 as libc::c_int,
                    *d as *mut libc::c_uchar,
                    if !(*d).is_null() { libc::strlen(*d) } else { 0 },
                );
                libc::write(
                    STDERR_FILENO,
                    // b"%s\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    str as *const libc::c_char as *const libc::c_void,
                    libc::strlen(str as *const libc::c_char),
                );
                yajl_free_error(hand, str);
                return 1 as libc::c_int;
            }
            yajl_handle_t::free(hand);
            times += 1;
            i += 1;
        }
    }
    let mut all_units: [*const libc::c_char; 4] = [
        b"B/s\0" as *const u8 as *const libc::c_char,
        b"KB/s\0" as *const u8 as *const libc::c_char,
        b"MB/s\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_char as *const libc::c_char,
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
    return 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    libc::printf(
        b"-- speed tests determine parsing throughput given %d different sample documents --\n\0"
            as *const u8 as *const libc::c_char,
        num_docs(),
    );
    libc::printf(b"With UTF8 validation:\n\0" as *const u8 as *const libc::c_char);
    let mut rv = run(1 as libc::c_int);
    if rv != 0 as libc::c_int {
        return rv;
    }
    libc::printf(b"Without UTF8 validation:\n\0" as *const u8 as *const libc::c_char);
    rv = run(0 as libc::c_int);
    return rv;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
