#![allow(clippy::missing_safety_doc)]
use core::{ptr, slice};

use yajl::{
    parser::{yajl_callbacks, Parser},
    yajl_alloc::yajl_alloc_funcs,
    Status,
};

use crate::yajl_handle_t;
#[allow(non_camel_case_types)]
pub type yajl_option = u32;
#[allow(non_camel_case_types)]
pub type yajl_status = u32;

/// allocate a parser handle
///
/// # Arguments
///
/// * `callbacks` - a yajl callbacks structure specifying the
///                    functions to call when different JSON entities
///                    are encountered in the input text.  May be NULL,
///                    which is only useful for validation.
/// * `afs` - memory allocation functions, may be NULL for to use
///                    C runtime library routines (malloc and friends)
/// * `ctx` - a context pointer that will be passed to callbacks.
///
/// # Safety
///
/// The caller is responsible for free the handle by calling `yajl_free`
#[no_mangle]
pub unsafe extern "C" fn yajl_alloc(
    mut callbacks: *const yajl_callbacks,
    mut afs: *mut yajl_alloc_funcs,
    mut ctx: *mut libc::c_void,
) -> *mut yajl_handle_t {
    Parser::alloc(callbacks, afs, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_free(mut handle: *mut yajl_handle_t) {
    Parser::free(handle)
}

#[cfg(feature = "nightly")]
#[no_mangle]
pub unsafe extern "C" fn yajl_config(
    mut h: *mut yajl_handle_t,
    mut opt: yajl_option,
    mut args: ...
) -> libc::c_int {
    let mut rv: libc::c_int = 1 as libc::c_int;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    match opt as libc::c_uint {
        1 | 2 | 4 | 8 | 16 => {
            if ap.arg::<libc::c_int>() != 0 {
                (*h).flags |= opt as libc::c_uint;
            } else {
                (*h).flags &= !(opt as libc::c_uint);
            }
        }
        _ => {
            rv = 0 as libc::c_int;
        }
    }
    rv
}
#[cfg(not(feature = "nightly"))]
#[no_mangle]
pub unsafe extern "C" fn yajl_config(
    mut h: *mut yajl_handle_t,
    mut opt: yajl_option,
    mut arg: libc::c_int,
) -> libc::c_int {
    use yajl::ParserOption;

    if h.is_null() {
        return 0;
    }
    let parser = unsafe { &mut *h };
    if let Some(opt) = ParserOption::from_repr(opt) {
        parser.config(opt, arg != 0) as i32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn yajl_status_to_string(mut stat: yajl_status) -> *const libc::c_char {
    let mut statStr: *const libc::c_char = b"unknown\0" as *const u8 as *const libc::c_char;
    match stat as libc::c_uint {
        0 => {
            statStr = b"ok, no error\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            statStr = b"client canceled parse\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            statStr = b"parse error\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    statStr
}

#[no_mangle]
pub unsafe extern "C" fn yajl_parse(
    mut hand: *mut yajl_handle_t,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: libc::size_t,
) -> yajl_status {
    if hand.is_null() {
        return Status::Error as yajl_status;
    }
    let parser = unsafe { &mut *hand };

    if jsonText.is_null() {
        return Status::Error as yajl_status;
    }
    let json_text: &[u8] = slice::from_raw_parts(jsonText as *const u8, jsonTextLen);
    parser.parse(json_text) as yajl_status
}
#[no_mangle]
pub unsafe extern "C" fn yajl_complete_parse(mut hand: *mut yajl_handle_t) -> yajl_status {
    if hand.is_null() {
        return Status::Error as yajl_status;
    }
    let parser = unsafe { &mut *hand };
    parser.complete_parse() as yajl_status
}
#[no_mangle]
pub unsafe extern "C" fn yajl_get_error(
    mut hand: *mut yajl_handle_t,
    mut verbose: libc::c_int,
    mut jsonText: *const libc::c_uchar,
    mut jsonTextLen: libc::size_t,
) -> *mut libc::c_uchar {
    if hand.is_null() {
        return ptr::null_mut();
    }
    let parser = unsafe { &mut *hand };
    if jsonText.is_null() {
        return ptr::null_mut();
    }
    let json_text: &[u8] = slice::from_raw_parts(jsonText as *const u8, jsonTextLen);
    parser.get_error(verbose != 0, json_text)
}
#[no_mangle]
pub unsafe extern "C" fn yajl_get_bytes_consumed(mut hand: *mut yajl_handle_t) -> libc::size_t {
    if hand.is_null() {
        return 0;
    }
    let parser = unsafe { &mut *hand };
    parser.get_bytes_consumed()
}
#[no_mangle]
pub unsafe extern "C" fn yajl_free_error(
    mut hand: *mut yajl_handle_t,
    mut str: *mut libc::c_uchar,
) {
    if hand.is_null() || str.is_null() {
        return;
    }
    let parser = unsafe { &mut *hand };
    parser.free_error(str)
}
