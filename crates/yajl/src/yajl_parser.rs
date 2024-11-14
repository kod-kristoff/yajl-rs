#![allow(clippy::missing_safety_doc)]
use core::{
    ffi::{c_char, c_double, c_int, c_long, c_longlong, c_uchar, c_uint, c_void},
    ptr,
};

use crate::{
    yajl_alloc::{yajl_alloc_funcs, yajl_set_default_alloc_funcs},
    yajl_buf::{
        yajl_buf_alloc, yajl_buf_append, yajl_buf_clear, yajl_buf_data, yajl_buf_free,
        yajl_buf_len, yajl_buf_t,
    },
    yajl_encode::yajl_string_decode,
    yajl_lex::{
        yajl_lex_error_to_string, yajl_lex_free, yajl_lex_get_error, yajl_lex_lex, yajl_lexer_t,
    },
    yajl_option::yajl_option,
};

#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "haiku",
    target_os = "illumos",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "solaris"
))]
#[allow(dead_code)]
use crate::util_libc::{get_last_error, set_last_error};
// pub type usize = usize;

pub type yajl_status = c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_ok: yajl_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_handle_t {
    pub callbacks: *const yajl_callbacks,
    pub ctx: *mut c_void,
    pub lexer: yajl_lexer,
    pub parseError: *const c_char,
    pub bytesConsumed: usize,
    pub decodeBuf: yajl_buf,
    pub stateStack: yajl_bytestack,
    pub alloc: yajl_alloc_funcs,
    pub flags: c_uint,
}

impl yajl_handle_t {
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
    /// The caller is responsible for free the handle by calling `yajl_handle_t::free`
    pub unsafe fn alloc(
        mut callbacks: *const yajl_callbacks,
        mut afs: *mut yajl_alloc_funcs,
        mut ctx: *mut c_void,
    ) -> *mut yajl_handle_t {
        let mut hand: yajl_handle = 0 as yajl_handle;
        let mut afsBuffer: yajl_alloc_funcs = yajl_alloc_funcs {
            malloc: None,
            realloc: None,
            free: None,
            ctx: ptr::null_mut::<c_void>(),
        };
        if !afs.is_null() {
            if ((*afs).malloc).is_none() || ((*afs).realloc).is_none() || ((*afs).free).is_none() {
                return 0 as yajl_handle;
            }
        } else {
            yajl_set_default_alloc_funcs(&mut afsBuffer);
            afs = &mut afsBuffer;
        }
        hand = ((*afs).malloc).expect("non-null function pointer")(
            (*afs).ctx,
            ::core::mem::size_of::<yajl_handle_t>(),
        ) as yajl_handle;
        libc::memcpy(
            &mut (*hand).alloc as *mut yajl_alloc_funcs as *mut c_void,
            afs as *mut c_void,
            ::core::mem::size_of::<yajl_alloc_funcs>(),
        );
        (*hand).callbacks = callbacks;
        (*hand).ctx = ctx;
        (*hand).lexer = 0 as yajl_lexer;
        (*hand).bytesConsumed = 0;
        (*hand).decodeBuf = yajl_buf_alloc(&mut (*hand).alloc);
        (*hand).flags = 0;
        (*hand).stateStack.stack = ptr::null_mut();
        (*hand).stateStack.size = 0;
        (*hand).stateStack.used = 0;
        (*hand).stateStack.yaf = &mut (*hand).alloc;
        if ((*hand).stateStack.size).wrapping_sub((*hand).stateStack.used) == 0 {
            (*hand).stateStack.size = ((*hand).stateStack.size).wrapping_add(128);
            (*hand).stateStack.stack = ((*(*hand).stateStack.yaf).realloc)
                .expect("non-null function pointer")(
                (*(*hand).stateStack.yaf).ctx,
                (*hand).stateStack.stack as *mut c_void,
                (*hand).stateStack.size as usize,
            ) as *mut c_uchar;
        }
        let fresh0 = (*hand).stateStack.used;
        (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_add(1);
        *((*hand).stateStack.stack).add(fresh0) = yajl_state_start as u8;
        hand
    }

    pub unsafe fn free(mut handle: yajl_handle) {
        if !((*handle).stateStack.stack).is_null() {
            ((*(*handle).stateStack.yaf).free).expect("non-null function pointer")(
                (*(*handle).stateStack.yaf).ctx,
                (*handle).stateStack.stack as *mut c_void,
            );
        }
        yajl_buf_free((*handle).decodeBuf);
        if !((*handle).lexer).is_null() {
            yajl_lex_free((*handle).lexer);
            (*handle).lexer = 0 as yajl_lexer;
        }
        ((*handle).alloc.free).expect("non-null function pointer")(
            (*handle).alloc.ctx,
            handle as *mut c_void,
        );
    }

    #[cfg(feature = "nightly")]
    pub unsafe extern "C" fn config(
        mut h: yajl_handle,
        mut opt: yajl_option,
        mut args: ...
    ) -> c_int {
        let mut rv: c_int = 1 as c_int;
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        match opt as c_uint {
            1 | 2 | 4 | 8 | 16 => {
                if ap.arg::<c_int>() != 0 {
                    (*h).flags |= opt as c_uint;
                } else {
                    (*h).flags &= !(opt as c_uint);
                }
            }
            _ => {
                rv = 0 as c_int;
            }
        }
        return rv;
    }
    #[cfg(not(feature = "nightly"))]
    pub extern "C" fn config(
        &mut self,
        // mut h: yajl_handle,
        mut opt: yajl_option,
        mut arg: c_int,
    ) -> c_int {
        let mut rv: c_int = 1 as c_int;
        match opt as c_uint {
            1 | 2 | 4 | 8 | 16 => {
                if arg != 0 {
                    self.flags |= opt as c_uint;
                } else {
                    self.flags &= !(opt as c_uint);
                }
            }
            _ => {
                rv = 0 as c_int;
            }
        }
        rv
    }
}
pub type yajl_bytestack = yajl_bytestack_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_bytestack_t {
    pub stack: *mut c_uchar,
    pub size: usize,
    pub used: usize,
    pub yaf: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_boolean: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub yajl_integer: Option<unsafe extern "C" fn(*mut c_void, c_longlong) -> c_int>,
    pub yajl_double: Option<unsafe extern "C" fn(*mut c_void, c_double) -> c_int>,
    pub yajl_number: Option<unsafe extern "C" fn(*mut c_void, *const c_char, usize) -> c_int>,
    pub yajl_string: Option<unsafe extern "C" fn(*mut c_void, *const c_uchar, usize) -> c_int>,
    pub yajl_start_map: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_map_key: Option<unsafe extern "C" fn(*mut c_void, *const c_uchar, usize) -> c_int>,
    pub yajl_end_map: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_start_array: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub yajl_end_array: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}
pub type yajl_handle = *mut yajl_handle_t;
pub type C2RustUnnamed = c_uint;
pub const yajl_allow_partial_values: C2RustUnnamed = 16;
pub const yajl_allow_multiple_values: C2RustUnnamed = 8;
pub const yajl_allow_trailing_garbage: C2RustUnnamed = 4;
pub const yajl_dont_validate_strings: C2RustUnnamed = 2;
pub const yajl_allow_comments: C2RustUnnamed = 1;
pub type yajl_tok = c_uint;
pub const yajl_tok_comment: yajl_tok = 14;
pub const yajl_tok_string_with_escapes: yajl_tok = 13;
pub const yajl_tok_string: yajl_tok = 12;
pub const yajl_tok_double: yajl_tok = 11;
pub const yajl_tok_integer: yajl_tok = 10;
pub const yajl_tok_right_bracket: yajl_tok = 9;
pub const yajl_tok_right_brace: yajl_tok = 8;
pub const yajl_tok_null: yajl_tok = 7;
pub const yajl_tok_left_bracket: yajl_tok = 6;
pub const yajl_tok_left_brace: yajl_tok = 5;
pub const yajl_tok_error: yajl_tok = 4;
pub const yajl_tok_eof: yajl_tok = 3;
pub const yajl_tok_comma: yajl_tok = 2;
pub const yajl_tok_colon: yajl_tok = 1;
pub const yajl_tok_bool: yajl_tok = 0;
pub type yajl_lex_error = c_uint;
pub const yajl_lex_unallowed_comment: yajl_lex_error = 10;
pub const yajl_lex_missing_integer_after_minus: yajl_lex_error = 9;
pub const yajl_lex_missing_integer_after_exponent: yajl_lex_error = 8;
pub const yajl_lex_missing_integer_after_decimal: yajl_lex_error = 7;
pub const yajl_lex_invalid_string: yajl_lex_error = 6;
pub const yajl_lex_invalid_char: yajl_lex_error = 5;
pub const yajl_lex_string_invalid_hex_char: yajl_lex_error = 4;
pub const yajl_lex_string_invalid_json_char: yajl_lex_error = 3;
pub const yajl_lex_string_invalid_escaped_char: yajl_lex_error = 2;
pub const yajl_lex_string_invalid_utf8: yajl_lex_error = 1;
pub const yajl_lex_e_ok: yajl_lex_error = 0;
pub type yajl_state = c_uint;
pub const yajl_state_got_value: yajl_state = 12;
pub const yajl_state_array_need_val: yajl_state = 11;
pub const yajl_state_array_got_val: yajl_state = 10;
pub const yajl_state_array_start: yajl_state = 9;
pub const yajl_state_map_need_key: yajl_state = 8;
pub const yajl_state_map_got_val: yajl_state = 7;
pub const yajl_state_map_need_val: yajl_state = 6;
pub const yajl_state_map_sep: yajl_state = 5;
pub const yajl_state_map_start: yajl_state = 4;
pub const yajl_state_lexical_error: yajl_state = 3;
pub const yajl_state_parse_error: yajl_state = 2;
pub const yajl_state_parse_complete: yajl_state = 1;
pub const yajl_state_start: yajl_state = 0;

pub unsafe extern "C" fn yajl_parse_integer(
    mut number: *const c_uchar,
    mut length: c_uint,
) -> c_longlong {
    let mut ret: c_longlong = 0 as c_int as c_longlong;
    let mut sign: c_long = 1 as c_int as c_long;
    let mut pos: *const c_uchar = number;
    if *pos as c_int == '-' as i32 {
        pos = pos.offset(1);
        sign = -(1 as c_int) as c_long;
    }
    if *pos as c_int == '+' as i32 {
        pos = pos.offset(1);
    }
    while pos < number.offset(length as isize) {
        if ret
            > 9223372036854775807 as c_longlong / 10 as c_int as c_longlong
                + 9223372036854775807 as c_longlong % 10 as c_int as c_longlong
        {
            set_last_error(34);
            return if sign == 1 as c_int as c_long {
                9223372036854775807 as c_longlong
            } else {
                -(9223372036854775807 as c_longlong) - 1 as c_longlong
            };
        }
        ret *= 10 as c_int as c_longlong;
        if 9223372036854775807 as c_longlong - ret < (*pos as c_int - '0' as i32) as c_longlong {
            set_last_error(34);
            return if sign == 1 as c_int as c_long {
                9223372036854775807 as c_longlong
            } else {
                -(9223372036854775807 as c_longlong) - 1 as c_longlong
            };
        }
        if (*pos as c_int) < '0' as i32 || *pos as c_int > '9' as i32 {
            set_last_error(34);
            return if sign == 1 as c_int as c_long {
                9223372036854775807 as c_longlong
            } else {
                -(9223372036854775807 as c_longlong) - 1 as c_longlong
            };
        }
        let fresh0 = pos;
        pos = pos.offset(1);
        ret += (*fresh0 as c_int - '0' as i32) as c_longlong;
    }
    sign as c_longlong * ret
}

pub unsafe extern "C" fn yajl_render_error_string(
    mut hand: yajl_handle,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
    mut verbose: c_int,
) -> *mut c_uchar {
    let mut offset: usize = (*hand).bytesConsumed;
    let mut str: *mut c_uchar = ptr::null_mut::<c_uchar>();
    let mut errorType: *const c_char = ptr::null::<c_char>();
    let mut errorText: *const c_char = ptr::null::<c_char>();
    let mut text: [c_char; 72] = [0; 72];
    let mut arrow: *const c_char =
        b"                     (right here) ------^\n\0" as *const u8 as *const c_char;
    if *((*hand).stateStack.stack).add(((*hand).stateStack.used).wrapping_sub(1)) as c_int
        == yajl_state_parse_error as c_int
    {
        errorType = b"parse\0" as *const u8 as *const c_char;
        errorText = (*hand).parseError;
    } else if *((*hand).stateStack.stack).add(((*hand).stateStack.used).wrapping_sub(1)) as c_int
        == yajl_state_lexical_error as c_int
    {
        errorType = b"lexical\0" as *const u8 as *const c_char;
        errorText = yajl_lex_error_to_string(yajl_lex_get_error((*hand).lexer));
    } else {
        errorType = b"unknown\0" as *const u8 as *const c_char;
    }
    let mut memneeded: usize = 0;
    memneeded = (memneeded).wrapping_add(libc::strlen(errorType));
    memneeded = (memneeded).wrapping_add(libc::strlen(b" error\0" as *const u8 as *const c_char));
    if !errorText.is_null() {
        memneeded = memneeded.wrapping_add(libc::strlen(b": \0" as *const u8 as *const c_char));
        memneeded = memneeded.wrapping_add(libc::strlen(errorText));
    }
    str = ((*hand).alloc.malloc).expect("non-null function pointer")(
        (*hand).alloc.ctx,
        memneeded.wrapping_add(2),
    ) as *mut c_uchar;
    if str.is_null() {
        return ptr::null_mut::<c_uchar>();
    }
    *str.offset(0 as c_int as isize) = 0;
    libc::strcat(str as *mut c_char, errorType);
    libc::strcat(
        str as *mut c_char,
        b" error\0" as *const u8 as *const c_char,
    );
    if !errorText.is_null() {
        libc::strcat(str as *mut c_char, b": \0" as *const u8 as *const c_char);
        libc::strcat(str as *mut c_char, errorText);
    }
    libc::strcat(str as *mut c_char, b"\n\0" as *const u8 as *const c_char);
    if verbose != 0 {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut i: usize = 0;
        let mut spacesNeeded: usize = 0;
        spacesNeeded = if offset < 30 {
            40usize.wrapping_sub(offset)
        } else {
            10
        };
        start = if offset >= 30 {
            offset.wrapping_sub(30)
        } else {
            0
        };
        end = if offset.wrapping_add(30) > jsonTextLen {
            jsonTextLen
        } else {
            offset.wrapping_add(30)
        };
        i = 0;
        while i < spacesNeeded {
            text[i] = ' ' as i32 as c_char;
            i = i.wrapping_add(1);
        }
        while start < end {
            if *jsonText.add(start) as c_int != '\n' as i32
                && *jsonText.add(start) as c_int != '\r' as i32
            {
                text[i] = *jsonText.add(start) as c_char;
            } else {
                text[i] = ' ' as i32 as c_char;
            }
            start = start.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        let fresh1 = i;
        i = i.wrapping_add(1);
        text[fresh1] = '\n' as i32 as c_char;
        text[i] = 0 as c_int as c_char;
        let mut newStr: *mut c_char = ((*hand).alloc.malloc).expect("non-null function pointer")(
            (*hand).alloc.ctx,
            (libc::strlen(str as *mut c_char))
                .wrapping_add(libc::strlen(text.as_mut_ptr()))
                .wrapping_add(libc::strlen(arrow))
                .wrapping_add(1),
        ) as *mut c_char;
        if !newStr.is_null() {
            *newStr.offset(0 as c_int as isize) = 0 as c_int as c_char;
            libc::strcat(newStr, str as *mut c_char);
            libc::strcat(newStr, text.as_mut_ptr());
            libc::strcat(newStr, arrow);
        }
        ((*hand).alloc.free).expect("non-null function pointer")(
            (*hand).alloc.ctx,
            str as *mut c_void,
        );
        str = newStr as *mut c_uchar;
    }
    str
}

pub unsafe extern "C" fn yajl_do_finish(mut hand: yajl_handle) -> yajl_status {
    let mut stat: yajl_status = yajl_status_ok;
    stat = yajl_do_parse(
        hand,
        b" \0" as *const u8 as *const c_char as *const c_uchar,
        1,
    );
    if stat as c_uint != yajl_status_ok {
        return stat;
    }
    match *((*hand).stateStack.stack).add(((*hand).stateStack.used).wrapping_sub(1)) as c_int {
        2 | 3 => yajl_status_error,
        12 | 1 => yajl_status_ok,
        _ => {
            if (*hand).flags & yajl_allow_partial_values == 0 {
                *((*hand).stateStack.stack).add(((*hand).stateStack.used).wrapping_sub(1)) =
                    yajl_state_parse_error as u8;
                (*hand).parseError = b"premature EOF\0" as *const u8 as *const c_char;
                return yajl_status_error;
            }
            yajl_status_ok
        }
    }
}

pub unsafe extern "C" fn yajl_do_parse(
    mut hand: yajl_handle,
    mut jsonText: *const c_uchar,
    mut jsonTextLen: usize,
) -> yajl_status {
    let mut current_block: u64;
    let mut tok: yajl_tok = yajl_tok_bool;
    let mut buf: *const c_uchar = ptr::null::<c_uchar>();
    let mut bufLen: usize = 0;
    let mut offset: *mut usize = &mut (*hand).bytesConsumed;
    *offset = 0;
    loop {
        match *((*hand).stateStack.stack).add(((*hand).stateStack.used).wrapping_sub(1)) as c_int {
            1 => {
                if (*hand).flags & yajl_allow_multiple_values != 0 {
                    *((*hand).stateStack.stack).add(((*hand).stateStack.used).wrapping_sub(1)) =
                        yajl_state_got_value as u8;
                } else {
                    if (*hand).flags & yajl_allow_trailing_garbage != 0 {
                        break;
                    }
                    if *offset == jsonTextLen {
                        break;
                    }
                    tok = yajl_lex_lex(
                        (*hand).lexer,
                        jsonText,
                        jsonTextLen,
                        offset,
                        &mut buf,
                        &mut bufLen,
                    );
                    if tok as c_uint != yajl_tok_eof {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError = b"trailing garbage\0" as *const u8 as *const c_char;
                    }
                }
            }
            3 | 2 => return yajl_status_error,
            0 | 12 | 6 | 11 | 9 => {
                let mut stateToPush: yajl_state = yajl_state_start;
                tok = yajl_lex_lex(
                    (*hand).lexer,
                    jsonText,
                    jsonTextLen,
                    offset,
                    &mut buf,
                    &mut bufLen,
                );
                match tok as c_uint {
                    3 => return yajl_status_ok,
                    4 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_lexical_error as u8;
                        continue;
                    }
                    12 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_string).is_some()
                            && ((*(*hand).callbacks).yajl_string)
                                .expect("non-null function pointer")(
                                (*hand).ctx, buf, bufLen
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        current_block = 6407515180622463684;
                    }
                    13 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_string).is_some()
                        {
                            yajl_buf_clear((*hand).decodeBuf);
                            yajl_string_decode((*hand).decodeBuf, buf, bufLen);
                            if ((*(*hand).callbacks).yajl_string)
                                .expect("non-null function pointer")(
                                (*hand).ctx,
                                yajl_buf_data((*hand).decodeBuf),
                                yajl_buf_len((*hand).decodeBuf),
                            ) == 0
                            {
                                *((*hand).stateStack.stack)
                                    .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                    yajl_state_parse_error as u8;
                                (*hand).parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const c_char;
                                return yajl_status_client_canceled;
                            }
                        }
                        current_block = 6407515180622463684;
                    }
                    0 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_boolean).is_some()
                            && ((*(*hand).callbacks).yajl_boolean)
                                .expect("non-null function pointer")(
                                (*hand).ctx,
                                (*buf as c_int == 't' as i32) as c_int,
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        current_block = 6407515180622463684;
                    }
                    7 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_null).is_some()
                            && ((*(*hand).callbacks).yajl_null).expect("non-null function pointer")(
                                (*hand).ctx,
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        current_block = 6407515180622463684;
                    }
                    6 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_start_map).is_some()
                            && ((*(*hand).callbacks).yajl_start_map)
                                .expect("non-null function pointer")(
                                (*hand).ctx
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        stateToPush = yajl_state_map_start;
                        current_block = 6407515180622463684;
                    }
                    5 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_start_array).is_some()
                            && ((*(*hand).callbacks).yajl_start_array)
                                .expect("non-null function pointer")(
                                (*hand).ctx
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        stateToPush = yajl_state_array_start;
                        current_block = 6407515180622463684;
                    }
                    10 => {
                        if !((*hand).callbacks).is_null() {
                            if ((*(*hand).callbacks).yajl_number).is_some() {
                                if ((*(*hand).callbacks).yajl_number)
                                    .expect("non-null function pointer")(
                                    (*hand).ctx,
                                    buf as *const c_char,
                                    bufLen,
                                ) == 0
                                {
                                    *((*hand).stateStack.stack)
                                        .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                        yajl_state_parse_error as u8;
                                    (*hand).parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const c_char;
                                    return yajl_status_client_canceled;
                                }
                            } else if ((*(*hand).callbacks).yajl_integer).is_some() {
                                let mut i: c_longlong = 0 as c_int as c_longlong;
                                set_last_error(0);
                                i = yajl_parse_integer(buf, bufLen as c_uint);
                                if (i == -(9223372036854775807 as c_longlong) - 1 as c_longlong
                                    || i == 9223372036854775807 as c_longlong)
                                    && get_last_error() == 34 as c_int
                                {
                                    *((*hand).stateStack.stack)
                                        .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                        yajl_state_parse_error as u8;
                                    (*hand).parseError =
                                        b"integer overflow\0" as *const u8 as *const c_char;
                                    if *offset >= bufLen {
                                        *offset = { *offset }.wrapping_sub(bufLen);
                                    } else {
                                        *offset = 0;
                                    }
                                    continue;
                                } else if ((*(*hand).callbacks).yajl_integer)
                                    .expect("non-null function pointer")(
                                    (*hand).ctx, i
                                ) == 0
                                {
                                    *((*hand).stateStack.stack)
                                        .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                        yajl_state_parse_error as u8;
                                    (*hand).parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const c_char;
                                    return yajl_status_client_canceled;
                                }
                            }
                            current_block = 6407515180622463684;
                        } else {
                            current_block = 6407515180622463684;
                        }
                    }
                    11 => {
                        if !((*hand).callbacks).is_null() {
                            if ((*(*hand).callbacks).yajl_number).is_some() {
                                if ((*(*hand).callbacks).yajl_number)
                                    .expect("non-null function pointer")(
                                    (*hand).ctx,
                                    buf as *const c_char,
                                    bufLen,
                                ) == 0
                                {
                                    *((*hand).stateStack.stack)
                                        .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                        yajl_state_parse_error as u8;
                                    (*hand).parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const c_char;
                                    return yajl_status_client_canceled;
                                }
                            } else if ((*(*hand).callbacks).yajl_double).is_some() {
                                let mut d: c_double = 0.0f64;
                                yajl_buf_clear((*hand).decodeBuf);
                                yajl_buf_append((*hand).decodeBuf, buf as *const c_void, bufLen);
                                buf = yajl_buf_data((*hand).decodeBuf);
                                set_last_error(0);
                                d = libc::strtod(
                                    buf as *mut c_char,
                                    ptr::null_mut::<*mut c_char>(),
                                );
                                if d.is_infinite() && get_last_error() == 34 as c_int {
                                    *((*hand).stateStack.stack)
                                        .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                        yajl_state_parse_error as u8;
                                    (*hand).parseError = b"numeric (floating point) overflow\0"
                                        as *const u8
                                        as *const c_char;
                                    if *offset >= bufLen {
                                        *offset = { *offset }.wrapping_sub(bufLen);
                                    } else {
                                        *offset = 0;
                                    }
                                    continue;
                                } else if ((*(*hand).callbacks).yajl_double)
                                    .expect("non-null function pointer")(
                                    (*hand).ctx, d
                                ) == 0
                                {
                                    *((*hand).stateStack.stack)
                                        .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                        yajl_state_parse_error as u8;
                                    (*hand).parseError =
                                        b"client cancelled parse via callback return value\0"
                                            as *const u8
                                            as *const c_char;
                                    return yajl_status_client_canceled;
                                }
                            }
                            current_block = 6407515180622463684;
                        } else {
                            current_block = 6407515180622463684;
                        }
                    }
                    8 => {
                        if *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1))
                            as c_int
                            == yajl_state_array_start as c_int
                        {
                            if !((*hand).callbacks).is_null()
                                && ((*(*hand).callbacks).yajl_end_array).is_some()
                                && ((*(*hand).callbacks).yajl_end_array)
                                    .expect("non-null function pointer")(
                                    (*hand).ctx
                                ) == 0
                            {
                                *((*hand).stateStack.stack)
                                    .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                    yajl_state_parse_error as u8;
                                (*hand).parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const c_char;
                                return yajl_status_client_canceled;
                            }
                            (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_sub(1);
                            continue;
                        } else {
                            current_block = 13495271385072242379;
                        }
                    }
                    1 | 2 | 9 => {
                        current_block = 13495271385072242379;
                    }
                    _ => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError =
                            b"invalid token, internal error\0" as *const u8 as *const c_char;
                        continue;
                    }
                }
                match current_block {
                    6407515180622463684 => {
                        let mut s: yajl_state = *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1))
                            as yajl_state;
                        if s as c_uint == yajl_state_start || s as c_uint == yajl_state_got_value {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_complete as u8;
                        } else if s as c_uint == yajl_state_map_need_val {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_map_got_val as u8;
                        } else {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_array_got_val as u8;
                        }
                        if stateToPush as c_uint != yajl_state_start {
                            if ((*hand).stateStack.size).wrapping_sub((*hand).stateStack.used) == 0
                            {
                                (*hand).stateStack.size = (*hand).stateStack.size.wrapping_add(128);
                                (*hand).stateStack.stack = ((*(*hand).stateStack.yaf).realloc)
                                    .expect("non-null function pointer")(
                                    (*(*hand).stateStack.yaf).ctx,
                                    (*hand).stateStack.stack as *mut c_void,
                                    (*hand).stateStack.size,
                                )
                                    as *mut c_uchar;
                            }
                            let fresh2 = (*hand).stateStack.used;
                            (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_add(1);
                            *((*hand).stateStack.stack).add(fresh2) = stateToPush as c_uchar;
                        }
                    }
                    _ => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError = b"unallowed token at this point in JSON text\0"
                            as *const u8
                            as *const c_char;
                    }
                }
            }
            4 | 8 => {
                tok = yajl_lex_lex(
                    (*hand).lexer,
                    jsonText,
                    jsonTextLen,
                    offset,
                    &mut buf,
                    &mut bufLen,
                );
                match tok as c_uint {
                    3 => return yajl_status_ok,
                    4 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_lexical_error as u8;
                        continue;
                    }
                    13 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_map_key).is_some()
                        {
                            yajl_buf_clear((*hand).decodeBuf);
                            yajl_string_decode((*hand).decodeBuf, buf, bufLen);
                            buf = yajl_buf_data((*hand).decodeBuf);
                            bufLen = yajl_buf_len((*hand).decodeBuf);
                        }
                        current_block = 5544887021832600539;
                    }
                    12 => {
                        current_block = 5544887021832600539;
                    }
                    9 => {
                        if *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1))
                            as c_int
                            == yajl_state_map_start as c_int
                        {
                            if !((*hand).callbacks).is_null()
                                && ((*(*hand).callbacks).yajl_end_map).is_some()
                                && ((*(*hand).callbacks).yajl_end_map)
                                    .expect("non-null function pointer")(
                                    (*hand).ctx
                                ) == 0
                            {
                                *((*hand).stateStack.stack)
                                    .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                    yajl_state_parse_error as u8;
                                (*hand).parseError =
                                    b"client cancelled parse via callback return value\0"
                                        as *const u8
                                        as *const c_char;
                                return yajl_status_client_canceled;
                            }
                            (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_sub(1);
                            continue;
                        } else {
                            current_block = 17513148302838498461;
                        }
                    }
                    _ => {
                        current_block = 17513148302838498461;
                    }
                }
                match current_block {
                    5544887021832600539 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_map_key).is_some()
                            && ((*(*hand).callbacks).yajl_map_key)
                                .expect("non-null function pointer")(
                                (*hand).ctx, buf, bufLen
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_map_sep as u8;
                    }
                    _ => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError = b"invalid object key (must be a string)\0" as *const u8
                            as *const c_char;
                    }
                }
            }
            5 => {
                tok = yajl_lex_lex(
                    (*hand).lexer,
                    jsonText,
                    jsonTextLen,
                    offset,
                    &mut buf,
                    &mut bufLen,
                );
                match tok as c_uint {
                    1 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_map_need_val as u8;
                    }
                    3 => return yajl_status_ok,
                    4 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_lexical_error as u8;
                    }
                    _ => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError =
                            b"object key and value must be separated by a colon (':')\0"
                                as *const u8 as *const c_char;
                    }
                }
            }
            7 => {
                tok = yajl_lex_lex(
                    (*hand).lexer,
                    jsonText,
                    jsonTextLen,
                    offset,
                    &mut buf,
                    &mut bufLen,
                );
                match tok as c_uint {
                    9 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_end_map).is_some()
                            && ((*(*hand).callbacks).yajl_end_map)
                                .expect("non-null function pointer")(
                                (*hand).ctx
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_sub(1);
                    }
                    2 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_map_need_key as u8;
                    }
                    3 => return yajl_status_ok,
                    4 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_lexical_error as u8;
                    }
                    _ => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError =
                            b"after key and value, inside map, I expect ',' or '}'\0" as *const u8
                                as *const c_char;
                        if *offset >= bufLen {
                            *offset = { *offset }.wrapping_sub(bufLen);
                        } else {
                            *offset = 0;
                        }
                    }
                }
            }
            10 => {
                tok = yajl_lex_lex(
                    (*hand).lexer,
                    jsonText,
                    jsonTextLen,
                    offset,
                    &mut buf,
                    &mut bufLen,
                );
                match tok as c_uint {
                    8 => {
                        if !((*hand).callbacks).is_null()
                            && ((*(*hand).callbacks).yajl_end_array).is_some()
                            && ((*(*hand).callbacks).yajl_end_array)
                                .expect("non-null function pointer")(
                                (*hand).ctx
                            ) == 0
                        {
                            *((*hand).stateStack.stack)
                                .add(((*hand).stateStack.used).wrapping_sub(1)) =
                                yajl_state_parse_error as u8;
                            (*hand).parseError =
                                b"client cancelled parse via callback return value\0" as *const u8
                                    as *const c_char;
                            return yajl_status_client_canceled;
                        }
                        (*hand).stateStack.used = ((*hand).stateStack.used).wrapping_sub(1);
                    }
                    2 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_array_need_val as u8;
                    }
                    3 => return yajl_status_ok,
                    4 => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_lexical_error as u8;
                    }
                    _ => {
                        *((*hand).stateStack.stack)
                            .add(((*hand).stateStack.used).wrapping_sub(1)) =
                            yajl_state_parse_error as u8;
                        (*hand).parseError = b"after array element, I expect ',' or ']'\0"
                            as *const u8
                            as *const c_char;
                    }
                }
            }
            _ => {
                libc::abort();
            }
        }
    }
    yajl_status_ok
}
