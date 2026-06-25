use core::ffi::{c_char, c_void, CStr};
use core::{fmt, ptr, slice};

use super::{Value, ValueError, ValueType};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub stack: *mut StackElem,
    pub root: *mut Value,
    pub errbuf: *mut c_char,
    pub errbuf_size: usize,
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("Context");
        if self.stack.is_null() {
            d.field("stack", &"NULL");
        } else {
            unsafe {
                d.field("stack", &*self.stack);
            }
        }
        if self.root.is_null() {
            d.field("root", &"NULL");
        } else {
            unsafe {
                d.field("root", &*self.root);
            }
        }
        if self.errbuf.is_null() {
            d.field("errbuf", &"NULL");
        } else {
            let bytes =
                unsafe { slice::from_raw_parts(self.errbuf as *const u8, self.errbuf_size) };
            d.field("errbuf", &String::from_utf8_lossy(bytes));
        }
        d.field("errbuf_size", &self.errbuf_size);
        d.finish()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackElem {
    pub key: *mut c_char,
    pub value: *mut Value,
    pub next: *mut StackElem,
}

impl fmt::Debug for StackElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("StackElem");
        if self.key.is_null() {
            d.field("key", &"NULL");
        } else {
            unsafe {
                d.field("key", &CStr::from_ptr(self.key));
            }
        }
        if self.value.is_null() {
            d.field("value", &"NULL");
        } else {
            unsafe {
                d.field("value", &*self.value);
            }
        }
        if self.next.is_null() {
            d.field("next", &"NULL");
        } else {
            unsafe {
                d.field("next", &*self.next);
            }
        }
        d.finish()
    }
}

impl Context {
    pub fn new(error_buffer: *mut c_char, error_buffer_size: usize) -> Self {
        if !error_buffer.is_null() {
            unsafe {
                ptr::write_bytes(error_buffer as *mut c_void, 0, error_buffer_size);
            }
        }
        Self {
            stack: ptr::null_mut(),
            root: ptr::null_mut(),
            errbuf: error_buffer,
            errbuf_size: error_buffer_size,
        }
    }
    pub unsafe fn push(mut ctx: *mut Context, mut v: *mut Value) -> Result<(), ValueError> {
        let stack = libc::malloc(::core::mem::size_of::<StackElem>()) as *mut StackElem;
        if stack.is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"Out of memory\0" as *const u8 as *const c_char,
                );
            }
            return Err(ValueError::OutOfMemory);
        }
        ptr::write_bytes(stack, 0, 1);

        (*stack).value = v;
        (*stack).next = (*ctx).stack;
        (*ctx).stack = stack;
        Ok(())
    }
    pub unsafe fn pop(mut ctx: *mut Context) -> Result<*mut Value, ValueError> {
        if ((*ctx).stack).is_null() {
            if !((*ctx).errbuf).is_null() {
                libc::snprintf(
                    (*ctx).errbuf,
                    (*ctx).errbuf_size,
                    b"context_pop: Bottom of stack reached prematurely\0" as *const u8
                        as *const c_char,
                );
            }
            return Err(ValueError::BottomOfStackReachedPrematurely);
        }
        let stack = (*ctx).stack;
        (*ctx).stack = (*stack).next;
        let v = (*stack).value;
        libc::free(stack as *mut c_void);
        Ok(v)
    }

    pub unsafe fn record_error(mut ctx: *mut Context, msg_with_nul: &[u8]) {
        debug_assert!(!ctx.is_null());
        if !(*ctx).errbuf.is_null() {
            libc::snprintf(
                (*ctx).errbuf,
                (*ctx).errbuf_size,
                msg_with_nul.as_ptr().cast(),
            );
        }
    }

    unsafe fn object_add_keyval(
        mut ctx: *mut Context,
        mut obj: *mut Value,
        mut key: *mut c_char,
        mut value: *mut Value,
    ) -> Result<(), ValueError> {
        let tmpk = libc::realloc(
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
            return Err(ValueError::OutOfMemory);
        }
        (*obj).u.object.keys = tmpk;
        let tmpv = libc::realloc(
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
            return Err(ValueError::OutOfMemory);
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
    ) -> Result<(), ValueError> {
        let tmp = libc::realloc(
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
            return Err(ValueError::OutOfMemory);
        }
        (*array).u.array.values = tmp;
        let fresh5 = &mut (*((*array).u.array.values).add((*array).u.array.len));
        *fresh5 = value;
        (*array).u.array.len = ((*array).u.array.len).wrapping_add(1);
        Ok(())
    }
    pub unsafe fn add_value(mut ctx: *mut Context, mut v: *mut Value) -> Result<(), ValueError> {
        if ((*ctx).stack).is_null() {
            (*ctx).root = v;
            Ok(())
        } else if !((*(*ctx).stack).value).is_null() && (*(*(*ctx).stack).value).is_object() {
            if ((*(*ctx).stack).key).is_null() {
                if !(!v.is_null() && (*v).is_string()) {
                    if !((*ctx).errbuf).is_null() {
                        libc::snprintf(
                            (*ctx).errbuf,
                            (*ctx).errbuf_size,
                            b"Context::add_value: Object key is not a string (%#04x)\0" as *const u8
                                as *const c_char,
                            (*v).type_0 as libc::c_uint,
                        );
                    }
                    return Err(ValueError::ObjectKeyIsNotAString);
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
            return Err(ValueError::CantAddValueToNonCompsiteType);
        }
    }
}
