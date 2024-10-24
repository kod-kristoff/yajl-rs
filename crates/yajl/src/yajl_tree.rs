#![allow(clippy::missing_safety_doc)]
#![allow(unused_unsafe)]
#![allow(clippy::nonminimal_bool)]
use std::ptr::{addr_of, null_mut};

use ::libc;

use crate::{
    yajl::{yajl_complete_parse, yajl_config, yajl_get_error},
    yajl_alloc::yajl_alloc_funcs,
    yajl_buf::yajl_buf_t,
    yajl_lex::yajl_lexer_t,
    yajl_parse,
    yajl_parser::{yajl_callbacks, yajl_handle_t, yajl_parse_integer},
};
// extern "C" {

//     #[cfg_attr(target_os = "android", link_name = "__errno")]
//     fn __errno_location() -> *mut libc::c_int;
//     // fn yajl_alloc(
//     //     callbacks: *const yajl_callbacks,
//     //     afs: *mut yajl_alloc_funcs,
//     //     ctx: *mut libc::c_void,
//     // ) -> yajl_handle;
//     fn yajl_config(h: yajl_handle, opt: yajl_option, _: ...) -> libc::c_int;
//     fn yajl_free(handle: yajl_handle);
//     fn yajl_parse(
//         hand: yajl_handle,
//         jsonText: *const libc::c_uchar,
//         jsonTextLength: usize,
//     ) -> yajl_status;
//     fn yajl_complete_parse(hand: yajl_handle) -> yajl_status;
//     fn yajl_get_error(
//         hand: yajl_handle,
//         verbose: libc::c_int,
//         jsonText: *const libc::c_uchar,
//         jsonTextLength: usize,
//     ) -> *mut libc::c_uchar;
//     fn yajl_parse_integer(number: *const libc::c_uchar, length: libc::c_uint) -> libc::c_longlong;
// }
pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize) -> *mut libc::c_void>;

pub type yajl_type = libc::c_uint;
pub const yajl_t_any: yajl_type = 8;
pub const yajl_t_null: yajl_type = 7;
pub const yajl_t_false: yajl_type = 6;
pub const yajl_t_true: yajl_type = 5;
pub const yajl_t_array: yajl_type = 4;
pub const yajl_t_object: yajl_type = 3;
pub const yajl_t_number: yajl_type = 2;
pub const yajl_t_string: yajl_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_val_s {
    pub type_0: yajl_type,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: *mut libc::c_char,
    pub number: C2RustUnnamed_2,
    pub object: C2RustUnnamed_1,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub values: *mut yajl_val,
    pub len: usize,
}
pub type yajl_val = *mut yajl_val_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keys: *mut *const libc::c_char,
    pub values: *mut yajl_val,
    pub len: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub i: libc::c_longlong,
    pub d: libc::c_double,
    pub r: *mut libc::c_char,
    pub flags: libc::c_uint,
}
pub type context_t = context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context_s {
    pub stack: *mut stack_elem_t,
    pub root: yajl_val,
    pub errbuf: *mut libc::c_char,
    pub errbuf_size: usize,
}
pub type stack_elem_t = stack_elem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_elem_s {
    pub key: *mut libc::c_char,
    pub value: yajl_val,
    pub next: *mut stack_elem_t,
}
pub type yajl_handle = *mut yajl_handle_t;

pub type yajl_bytestack = yajl_bytestack_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yajl_bytestack_t {
    pub stack: *mut libc::c_uchar,
    pub size: usize,
    pub used: usize,
    pub yaf: *mut yajl_alloc_funcs,
}
pub type yajl_buf = *mut yajl_buf_t;
pub type yajl_lexer = *mut yajl_lexer_t;

pub const yajl_status_ok: yajl_status = 0;
pub type yajl_status = libc::c_uint;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_status_client_canceled: yajl_status = 1;
pub type yajl_option = libc::c_uint;
pub const yajl_allow_partial_values: yajl_option = 16;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_comments: yajl_option = 1;
type Errno = libc::c_int;
fn get_last_error() -> Errno {
    // SAFETY:
    //  The only way to safely access the referenced errno is to use either
    //  `get_last_error` or `set_last_error`, ensuring that no one currently
    //  holds a mutable reference to the underlying value.
    unsafe { *libc::__errno_location() }
}

fn set_last_error(code: Errno) {
    // SAFETY:
    //  The only way to safely access the referenced errno is to use either
    //  `set_last_error` or `get_last_error`, ensuring that no one currently
    //  holds any reference to the underlying value.
    unsafe { *libc::__errno_location() = code };
}

unsafe extern "C" fn value_alloc(mut type_0: yajl_type) -> yajl_val {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = libc::malloc(::core::mem::size_of::<yajl_val_s>()) as yajl_val;
    if v.is_null() {
        return 0 as yajl_val;
    }
    libc::memset(
        v as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<yajl_val_s>(),
    );
    (*v).type_0 = type_0;
    v
}
unsafe extern "C" fn yajl_object_free(mut v: yajl_val) {
    let mut i: usize = 0;
    if !(!v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_object as libc::c_int as libc::c_uint)
    {
        return;
    }
    i = 0 as libc::c_int as usize;
    while i < (*v).u.object.len {
        libc::free(*((*v).u.object.keys).add(i) as *mut libc::c_char as *mut libc::c_void);
        let fresh0 = &mut (*((*v).u.object.keys).add(i));
        *fresh0 = std::ptr::null::<libc::c_char>();
        yajl_tree_free(*((*v).u.object.values).add(i));
        let fresh1 = &mut (*((*v).u.object.values).add(i));
        *fresh1 = 0 as yajl_val;
        i = i.wrapping_add(1);
    }
    libc::free((*v).u.object.keys as *mut libc::c_void);
    libc::free((*v).u.object.values as *mut libc::c_void);
    libc::free(v as *mut libc::c_void);
}
unsafe extern "C" fn yajl_array_free(mut v: yajl_val) {
    let mut i: usize = 0;
    if !(!v.is_null() && (*v).type_0 as libc::c_uint == yajl_t_array as libc::c_int as libc::c_uint)
    {
        return;
    }
    i = 0 as libc::c_int as usize;
    while i < (*v).u.array.len {
        yajl_tree_free(*((*v).u.array.values).add(i));
        let fresh2 = &mut (*((*v).u.array.values).add(i));
        *fresh2 = 0 as yajl_val;
        i = i.wrapping_add(1);
    }
    libc::free((*v).u.array.values as *mut libc::c_void);
    libc::free(v as *mut libc::c_void);
}
unsafe extern "C" fn context_push(mut ctx: *mut context_t, mut v: yajl_val) -> libc::c_int {
    let mut stack: *mut stack_elem_t = std::ptr::null_mut::<stack_elem_t>();
    stack = libc::malloc(::core::mem::size_of::<stack_elem_t>()) as *mut stack_elem_t;
    if stack.is_null() {
        if !((*ctx).errbuf).is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    libc::memset(
        stack as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<stack_elem_t>(),
    );
    (*stack).value = v;
    (*stack).next = (*ctx).stack;
    (*ctx).stack = stack;
    0 as libc::c_int
}
unsafe extern "C" fn context_pop(mut ctx: *mut context_t) -> yajl_val {
    let mut stack: *mut stack_elem_t = std::ptr::null_mut::<stack_elem_t>();
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    if ((*ctx).stack).is_null() {
        if !((*ctx).errbuf).is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"context_pop: Bottom of stack reached prematurely\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as yajl_val;
    }
    stack = (*ctx).stack;
    (*ctx).stack = (*stack).next;
    v = (*stack).value;
    libc::free(stack as *mut libc::c_void);
    v
}
unsafe extern "C" fn object_add_keyval(
    mut ctx: *mut context_t,
    mut obj: yajl_val,
    mut key: *mut libc::c_char,
    mut value: yajl_val,
) -> libc::c_int {
    let mut tmpk: *mut *const libc::c_char = std::ptr::null_mut::<*const libc::c_char>();
    let mut tmpv: *mut yajl_val = std::ptr::null_mut::<yajl_val>();
    tmpk = libc::realloc(
        (*obj).u.object.keys as *mut libc::c_void,
        (::core::mem::size_of::<*const libc::c_char>())
            .wrapping_mul(((*obj).u.object.len).wrapping_add(1)),
    ) as *mut *const libc::c_char;
    if tmpk.is_null() {
        if !((*ctx).errbuf).is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    (*obj).u.object.keys = tmpk;
    tmpv = libc::realloc(
        (*obj).u.object.values as *mut libc::c_void,
        (::core::mem::size_of::<yajl_val>()).wrapping_mul(((*obj).u.object.len).wrapping_add(1)),
    ) as *mut yajl_val;
    if tmpv.is_null() {
        if !((*ctx).errbuf).is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    (*obj).u.object.values = tmpv;
    let fresh3 = &mut (*((*obj).u.object.keys).add((*obj).u.object.len));
    *fresh3 = key;
    let fresh4 = &mut (*((*obj).u.object.values).add((*obj).u.object.len));
    *fresh4 = value;
    (*obj).u.object.len = ((*obj).u.object.len).wrapping_add(1);
    0 as libc::c_int
}
unsafe extern "C" fn array_add_value(
    mut ctx: *mut context_t,
    mut array: yajl_val,
    mut value: yajl_val,
) -> libc::c_int {
    let mut tmp: *mut yajl_val = std::ptr::null_mut::<yajl_val>();
    tmp = libc::realloc(
        (*array).u.array.values as *mut libc::c_void,
        (::core::mem::size_of::<yajl_val>()).wrapping_mul(((*array).u.array.len).wrapping_add(1)),
    ) as *mut yajl_val;
    if tmp.is_null() {
        if !((*ctx).errbuf).is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 12 as libc::c_int;
    }
    (*array).u.array.values = tmp;
    let fresh5 = &mut (*((*array).u.array.values).add((*array).u.array.len));
    *fresh5 = value;
    (*array).u.array.len = ((*array).u.array.len).wrapping_add(1);
    0 as libc::c_int
}
unsafe extern "C" fn context_add_value(mut ctx: *mut context_t, mut v: yajl_val) -> libc::c_int {
    if ((*ctx).stack).is_null() {
        (*ctx).root = v;
        0 as libc::c_int
    } else if !((*(*ctx).stack).value).is_null()
        && (*(*(*ctx).stack).value).type_0 as libc::c_uint
            == yajl_t_object as libc::c_int as libc::c_uint
    {
        if ((*(*ctx).stack).key).is_null() {
            if !(!v.is_null()
                && (*v).type_0 as libc::c_uint == yajl_t_string as libc::c_int as libc::c_uint)
            {
                if !((*ctx).errbuf).is_null() {
                    libc::snprintf(
                        (*ctx).errbuf,
                        (*ctx).errbuf_size,
                        b"context_add_value: Object key is not a string (%#04x)\0" as *const u8
                            as *const libc::c_char,
                        (*v).type_0 as libc::c_uint,
                    );
                }
                return 22 as libc::c_int;
            }
            (*(*ctx).stack).key = (*v).u.string;
            (*v).u.string = std::ptr::null_mut::<libc::c_char>();
            libc::free(v as *mut libc::c_void);
            return 0 as libc::c_int;
        } else {
            let mut key: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
            key = (*(*ctx).stack).key;
            (*(*ctx).stack).key = std::ptr::null_mut::<libc::c_char>();
            return object_add_keyval(ctx, (*(*ctx).stack).value, key, v);
        }
    } else if !((*(*ctx).stack).value).is_null()
        && (*(*(*ctx).stack).value).type_0 as libc::c_uint
            == yajl_t_array as libc::c_int as libc::c_uint
    {
        return array_add_value(ctx, (*(*ctx).stack).value, v);
    } else {
        if !((*ctx).errbuf).is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"context_add_value: Cannot add value to a value of type %#04x (not a composite type)\0"
                    as *const u8 as *const libc::c_char,
                (*(*(*ctx).stack).value).type_0 as libc::c_uint,
            );
        }
        return 22 as libc::c_int;
    }
}
unsafe extern "C" fn handle_string(
    mut ctx: *mut libc::c_void,
    mut string: *const libc::c_uchar,
    mut string_length: usize,
) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = value_alloc(yajl_t_string);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.string = libc::malloc(string_length.wrapping_add(1)) as *mut libc::c_char;
    if ((*v).u.string).is_null() {
        libc::free(v as *mut libc::c_void);
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    libc::memcpy(
        (*v).u.string as *mut libc::c_void,
        string as *const libc::c_void,
        string_length,
    );
    *((*v).u.string).add(string_length) = 0 as libc::c_int as libc::c_char;
    if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_number(
    mut ctx: *mut libc::c_void,
    mut string: *const libc::c_char,
    mut string_length: usize,
) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    let mut endptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    v = value_alloc(yajl_t_number);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.number.r = libc::malloc(string_length.wrapping_add(1)) as *mut libc::c_char;
    if ((*v).u.number.r).is_null() {
        libc::free(v as *mut libc::c_void);
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    libc::memcpy(
        (*v).u.number.r as *mut libc::c_void,
        string as *const libc::c_void,
        string_length,
    );
    *((*v).u.number.r).add(string_length) = 0 as libc::c_int as libc::c_char;
    (*v).u.number.flags = 0 as libc::c_int as libc::c_uint;
    set_last_error(0);
    (*v).u.number.i = yajl_parse_integer(
        (*v).u.number.r as *const libc::c_uchar,
        libc::strlen((*v).u.number.r) as libc::c_uint,
    );
    if get_last_error() == 0 as libc::c_int {
        (*v).u.number.flags |= 0x1 as libc::c_int as libc::c_uint;
    }
    endptr = std::ptr::null_mut::<libc::c_char>();
    set_last_error(0);
    (*v).u.number.d = libc::strtod((*v).u.number.r, &mut endptr);
    if get_last_error() == 0 as libc::c_int
        && !endptr.is_null()
        && *endptr as libc::c_int == 0 as libc::c_int
    {
        (*v).u.number.flags |= 0x2 as libc::c_int as libc::c_uint;
    }
    if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_start_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = value_alloc(yajl_t_object);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.object.keys = std::ptr::null_mut::<*const libc::c_char>();
    (*v).u.object.values = std::ptr::null_mut::<yajl_val>();
    (*v).u.object.len = 0 as libc::c_int as usize;
    if context_push(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_end_map(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = context_pop(ctx as *mut context_t);
    if v.is_null() {
        return 0 as libc::c_int;
    }
    if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_start_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = value_alloc(yajl_t_array);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    (*v).u.array.values = std::ptr::null_mut::<yajl_val>();
    (*v).u.array.len = 0 as libc::c_int as usize;
    if context_push(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_end_array(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = context_pop(ctx as *mut context_t);
    if v.is_null() {
        return 0 as libc::c_int;
    }
    if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_boolean(
    mut ctx: *mut libc::c_void,
    mut boolean_value: libc::c_int,
) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = value_alloc(
        (if boolean_value != 0 {
            yajl_t_true as libc::c_int
        } else {
            yajl_t_false as libc::c_int
        }) as yajl_type,
    );
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
unsafe extern "C" fn handle_null(mut ctx: *mut libc::c_void) -> libc::c_int {
    let mut v: yajl_val = std::ptr::null_mut::<yajl_val_s>();
    v = value_alloc(yajl_t_null);
    if v.is_null() {
        if !((*(ctx as *mut context_t)).errbuf).is_null() {
            libc::snprintf(
                (*(ctx as *mut context_t)).errbuf,
                (*(ctx as *mut context_t)).errbuf_size,
                b"Out of memory\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if context_add_value(ctx as *mut context_t, v) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}

pub unsafe extern "C" fn yajl_tree_parse(
    mut input: *const libc::c_char,
    mut error_buffer: *mut libc::c_char,
    mut error_buffer_size: usize,
) -> yajl_val {
    static mut callbacks: yajl_callbacks = unsafe {
        {
            yajl_callbacks {
                yajl_null: Some(
                    handle_null as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_boolean: Some(
                    handle_boolean
                        as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> libc::c_int,
                ),
                yajl_integer: None,
                yajl_double: None,
                yajl_number: Some(
                    handle_number
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            usize,
                        ) -> libc::c_int,
                ),
                yajl_string: Some(
                    handle_string
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_uchar,
                            usize,
                        ) -> libc::c_int,
                ),
                yajl_start_map: Some(
                    handle_start_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_map_key: Some(
                    handle_string
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_uchar,
                            usize,
                        ) -> libc::c_int,
                ),
                yajl_end_map: Some(
                    handle_end_map as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_start_array: Some(
                    handle_start_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
                yajl_end_array: Some(
                    handle_end_array as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                ),
            }
        }
    };
    let mut handle: yajl_handle = std::ptr::null_mut::<yajl_handle_t>();
    let mut status: yajl_status = yajl_status_ok;
    let mut internal_err_str: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut ctx: context_t = {
        context_s {
            stack: std::ptr::null_mut::<stack_elem_t>(),
            root: 0 as yajl_val,
            errbuf: std::ptr::null_mut::<libc::c_char>(),
            errbuf_size: 0 as libc::c_int as usize,
        }
    };
    ctx.errbuf = error_buffer;
    ctx.errbuf_size = error_buffer_size;
    if !error_buffer.is_null() {
        libc::memset(
            error_buffer as *mut libc::c_void,
            0 as libc::c_int,
            error_buffer_size,
        );
    }
    handle = yajl_handle_t::alloc(
        addr_of!(callbacks),
        std::ptr::null_mut::<yajl_alloc_funcs>(),
        &mut ctx as *mut context_t as *mut libc::c_void,
    );
    yajl_config(handle, yajl_allow_comments, 1 as libc::c_int);
    status = yajl_parse(handle, input as *mut libc::c_uchar, libc::strlen(input));
    status = yajl_complete_parse(handle);
    if status as libc::c_uint != yajl_status_ok as libc::c_int as libc::c_uint {
        if !error_buffer.is_null() && error_buffer_size > 0 as libc::c_int as usize {
            internal_err_str = yajl_get_error(
                handle,
                1 as libc::c_int,
                input as *const libc::c_uchar,
                libc::strlen(input),
            ) as *mut libc::c_char;
            libc::snprintf(
                error_buffer,
                error_buffer_size,
                b"%s\0" as *const u8 as *const libc::c_char,
                internal_err_str,
            );
            ((*handle).alloc.free).expect("non-null function pointer")(
                (*handle).alloc.ctx,
                internal_err_str as *mut libc::c_void,
            );
        }
        yajl_handle_t::free(handle);
        return 0 as yajl_val;
    }
    yajl_handle_t::free(handle);
    ctx.root
}

pub unsafe extern "C" fn yajl_tree_get(
    mut n: yajl_val,
    mut path: *mut *const libc::c_char,
    mut type_0: yajl_type,
) -> yajl_val {
    if path.is_null() {
        return 0 as yajl_val;
    }
    while !n.is_null() && !(*path).is_null() {
        let mut i: usize = 0;
        let mut len: usize = 0;
        if (*n).type_0 as libc::c_uint != yajl_t_object as libc::c_int as libc::c_uint {
            return 0 as yajl_val;
        }
        len = (*n).u.object.len;
        i = 0 as libc::c_int as usize;
        while i < len {
            if libc::strcmp(*path, *((*n).u.object.keys).add(i)) == 0 {
                n = *((*n).u.object.values).add(i);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i == len {
            return 0 as yajl_val;
        }
        path = path.offset(1);
    }
    if !n.is_null()
        && type_0 as libc::c_uint != yajl_t_any as libc::c_int as libc::c_uint
        && type_0 as libc::c_uint != (*n).type_0 as libc::c_uint
    {
        n = 0 as yajl_val;
    }
    n
}

pub unsafe extern "C" fn yajl_tree_free(mut v: yajl_val) {
    if v.is_null() {
        return;
    }
    if !v.is_null() && (*v).type_0 as libc::c_uint == yajl_t_string as libc::c_int as libc::c_uint {
        libc::free((*v).u.string as *mut libc::c_void);
        libc::free(v as *mut libc::c_void);
    } else if !v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_number as libc::c_int as libc::c_uint
    {
        libc::free((*v).u.number.r as *mut libc::c_void);
        libc::free(v as *mut libc::c_void);
    } else if !if !v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_object as libc::c_int as libc::c_uint
    {
        &mut (*v).u.object as *mut C2RustUnnamed_1
    } else {
        null_mut::<C2RustUnnamed_1>()
    }
    .is_null()
    {
        yajl_object_free(v);
    } else if !if !v.is_null()
        && (*v).type_0 as libc::c_uint == yajl_t_array as libc::c_int as libc::c_uint
    {
        &mut (*v).u.array as *mut C2RustUnnamed_0
    } else {
        null_mut::<C2RustUnnamed_0>()
    }
    .is_null()
    {
        yajl_array_free(v);
    } else {
        libc::free(v as *mut libc::c_void);
    };
}
