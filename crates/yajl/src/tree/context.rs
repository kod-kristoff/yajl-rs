use core::ffi::{c_char, c_void};
use core::ptr;

use super::{Value, ValueType};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub stack: *mut StackElem,
    pub root: *mut Value,
    pub errbuf: *mut c_char,
    pub errbuf_size: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackElem {
    pub key: *mut c_char,
    pub value: *mut Value,
    pub next: *mut StackElem,
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum ContextError {
    OutOfMemory = 12,
    ObjectKeyIsNotAString,
    CantAddValueToNonCompsiteType = 22,
    BottomOfStackReachedPrematurely,
}

impl Context {
    pub unsafe fn push(mut ctx: *mut Context, mut v: *mut Value) -> Result<(), ContextError> {
        eprintln!("Context::push: v={:?}", *v);
        let mut stack: *mut StackElem = ptr::null_mut::<StackElem>();
        stack = libc::malloc(::core::mem::size_of::<StackElem>()) as *mut StackElem;
        if stack.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        ptr::write_bytes(stack, 0, 1);

        (*stack).value = v;
        (*stack).next = (*ctx).stack;
        (*ctx).stack = stack;
        Ok(())
    }
    pub unsafe fn pop(mut ctx: *mut Context) -> Result<*mut Value, ContextError> {
        let mut stack: *mut StackElem = ptr::null_mut::<StackElem>();
        let mut v: *mut Value = ptr::null_mut::<Value>();
        if ((*ctx).stack).is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"context_pop: Bottom of stack reached prematurely\0" as *const u8
                        as *const c_char,
                );
            }
            return dbg!(Err(ContextError::BottomOfStackReachedPrematurely));
        }
        stack = (*ctx).stack;
        (*ctx).stack = (*stack).next;
        v = (*stack).value;
        libc::free(stack as *mut c_void);
        eprintln!("Context::pop: v={:?}", *v);
        Ok(v)
    }

    unsafe fn object_add_keyval(
        mut ctx: *mut Context,
        mut obj: *mut Value,
        mut key: *mut c_char,
        mut value: *mut Value,
    ) -> Result<(), ContextError> {
        let mut tmpk: *mut *const c_char = ptr::null_mut::<*const c_char>();
        let mut tmpv: *mut *mut Value = ptr::null_mut::<*mut Value>();
        tmpk = libc::realloc(
            (*obj).u.object.keys as *mut c_void,
            (::core::mem::size_of::<*const c_char>())
                .wrapping_mul(((*obj).u.object.len).wrapping_add(1)),
        ) as *mut *const c_char;
        if tmpk.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        (*obj).u.object.keys = tmpk;
        tmpv = libc::realloc(
            (*obj).u.object.values as *mut c_void,
            (::core::mem::size_of::<*mut Value>())
                .wrapping_mul(((*obj).u.object.len).wrapping_add(1)),
        ) as *mut *mut Value;
        if tmpv.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        (*obj).u.object.values = tmpv;
        let fresh3 = &mut (*((*obj).u.object.keys).add((*obj).u.object.len));
        *fresh3 = key;
        let fresh4 = &mut (*((*obj).u.object.values).add((*obj).u.object.len));
        *fresh4 = value;
        (*obj).u.object.len = ((*obj).u.object.len).wrapping_add(1);
        Ok(())
    }
    unsafe fn array_add_value(
        mut ctx: *mut Context,
        mut array: *mut Value,
        mut value: *mut Value,
    ) -> Result<(), ContextError> {
        let mut tmp: *mut *mut Value = ptr::null_mut::<*mut Value>();
        tmp = libc::realloc(
            (*array).u.array.values as *mut c_void,
            (::core::mem::size_of::<*mut Value>())
                .wrapping_mul(((*array).u.array.len).wrapping_add(1)),
        ) as *mut *mut Value;
        if tmp.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ContextError::OutOfMemory);
        }
        (*array).u.array.values = tmp;
        let fresh5 = &mut (*((*array).u.array.values).add((*array).u.array.len));
        *fresh5 = value;
        (*array).u.array.len = ((*array).u.array.len).wrapping_add(1);
        Ok(())
    }
    pub unsafe fn add_value(mut ctx: *mut Context, mut v: *mut Value) -> Result<(), ContextError> {
        if ((*ctx).stack).is_null() {
            (*ctx).root = v;
            Ok(())
        } else if !((*(*ctx).stack).value).is_null()
            && (*(*(*ctx).stack).value).type_0 as libc::c_uint
                == ValueType::Object as libc::c_int as libc::c_uint
        {
            if ((*(*ctx).stack).key).is_null() {
                if !(!v.is_null()
                    && (*v).type_0 as libc::c_uint
                        == ValueType::String as libc::c_int as libc::c_uint)
                {
                    if !((*ctx).errbuf).is_null() {
                        libc::snprintf(
                            (*ctx).errbuf,
                            (*ctx).errbuf_size,
                            b"Context::add_value: Object key is not a string (%#04x)\0" as *const u8
                                as *const c_char,
                            (*v).type_0 as libc::c_uint,
                        );
                    }
                    return Err(ContextError::ObjectKeyIsNotAString);
                }
                (*(*ctx).stack).key = (*v).u.string;
                (*v).u.string = ptr::null_mut::<c_char>();
                libc::free(v as *mut c_void);
                return Ok(());
            } else {
                let mut key: *mut c_char = ptr::null_mut::<c_char>();
                key = (*(*ctx).stack).key;
                (*(*ctx).stack).key = ptr::null_mut::<c_char>();
                return Context::object_add_keyval(ctx, (*(*ctx).stack).value, key, v);
            }
        } else if !((*(*ctx).stack).value).is_null()
            && (*(*(*ctx).stack).value).type_0 as libc::c_uint
                == ValueType::Array as libc::c_int as libc::c_uint
        {
            return Context::array_add_value(ctx, (*(*ctx).stack).value, v);
        } else {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                b"Context::add_value: Cannot add value to a value of type %#04x (not a composite type)\0"
                    as *const u8 as *const c_char,
                (*(*(*ctx).stack).value).type_0 as libc::c_uint,
            );
            }
            return Err(ContextError::CantAddValueToNonCompsiteType);
        }
    }
}