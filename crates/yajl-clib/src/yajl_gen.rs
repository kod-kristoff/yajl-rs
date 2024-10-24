use ::libc;

use yajl::{yajl_alloc::yajl_alloc_funcs, yajl_buf::yajl_buf_t, yajl_gen::yajl_gen_t};

pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize) -> *mut libc::c_void>;

pub type yajl_gen_status = libc::c_uint;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_gen_status_ok: yajl_gen_status = 0;

pub type yajl_print_t =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> ()>;
pub type yajl_gen_state = libc::c_uint;
pub const yajl_gen_error: yajl_gen_state = 7;
pub const yajl_gen_complete: yajl_gen_state = 6;
pub const yajl_gen_in_array: yajl_gen_state = 5;
pub const yajl_gen_array_start: yajl_gen_state = 4;
pub const yajl_gen_map_val: yajl_gen_state = 3;
pub const yajl_gen_map_key: yajl_gen_state = 2;
pub const yajl_gen_map_start: yajl_gen_state = 1;
pub const yajl_gen_start: yajl_gen_state = 0;
pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_gen_option = libc::c_uint;
pub const yajl_gen_escape_solidus: yajl_gen_option = 16;
pub const yajl_gen_validate_utf8: yajl_gen_option = 8;
pub const yajl_gen_print_callback: yajl_gen_option = 4;
pub const yajl_gen_indent_string: yajl_gen_option = 2;
pub const yajl_gen_beautify: yajl_gen_option = 1;
pub type va_list = __builtin_va_list;
pub type yajl_buf = *mut yajl_buf_t;
#[cfg(feature = "nightly")]
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_config(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut args: ...
) -> libc::c_int {
    yajl::yajl_gen::yajl_gen_config(g, opt, args)
}
#[cfg(not(feature = "nightly"))]
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_config_set_indent(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut indent: *const libc::c_char,
) -> libc::c_int {
    yajl::yajl_gen::yajl_gen_config_set_indent(g, opt, indent)
}

#[cfg(not(feature = "nightly"))]
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_config(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut arg: libc::c_int,
) -> libc::c_int {
    yajl::yajl_gen::yajl_gen_config(g, opt, arg)
}
#[cfg(not(feature = "nightly"))]
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_config_print_callback(
    mut g: yajl_gen,
    mut opt: yajl_gen_option,
    mut arg: libc::c_int,
    print: unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> (),
    ctx: *mut libc::c_void,
) -> libc::c_int {
    yajl::yajl_gen::yajl_gen_config_print_callback(g, opt, arg, print, ctx)
}
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_alloc(mut afs: *const yajl_alloc_funcs) -> yajl_gen {
    yajl::yajl_gen::yajl_gen_alloc(afs)
}
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_reset(mut g: yajl_gen, mut sep: *const libc::c_char) {
    yajl::yajl_gen::yajl_gen_reset(g, sep)
}
#[no_mangle]
pub unsafe extern "C" fn yajl_gen_free(mut g: yajl_gen) {
    yajl::yajl_gen::yajl_gen_free(g)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_integer(
    mut g: yajl_gen,
    mut number: libc::c_longlong,
) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_integer(g, number)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_double(
    mut g: yajl_gen,
    mut number: libc::c_double,
) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_double(g, number)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_number(
    mut g: yajl_gen,
    mut s: *const libc::c_char,
    mut l: usize,
) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_number(g, s, l)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_string(
    mut g: yajl_gen,
    mut str: *const libc::c_uchar,
    mut len: usize,
) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_string(g, str, len)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_null(mut g: yajl_gen) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_null(g)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_bool(
    mut g: yajl_gen,
    mut boolean: libc::c_int,
) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_bool(g, boolean)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_map_open(mut g: yajl_gen) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_map_open(g)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_map_close(mut g: yajl_gen) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_map_close(g)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_array_open(mut g: yajl_gen) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_array_open(g)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_array_close(mut g: yajl_gen) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_array_close(g)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_get_buf(
    mut g: yajl_gen,
    mut buf: *mut *const libc::c_uchar,
    mut len: *mut usize,
) -> yajl_gen_status {
    yajl::yajl_gen::yajl_gen_get_buf(g, buf, len)
}

#[no_mangle]
pub unsafe extern "C" fn yajl_gen_clear(mut g: yajl_gen) {
    yajl::yajl_gen::yajl_gen_clear(g)
}
