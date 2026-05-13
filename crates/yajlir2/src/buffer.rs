use core::{ffi::c_void, ptr};

use crate::yajl_alloc::yajl_alloc_funcs;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    cap: usize,
    len: usize,
    data: *mut u8,
    alloc: *mut yajl_alloc_funcs,
}
impl Buffer {
    unsafe fn ensure_available(&mut self, mut want: usize) {
        let mut need: usize = 0;
        if self.data.is_null() {
            self.cap = 2048;
            self.data = ((*self.alloc).malloc).expect("non-null function pointer")(
                (*self.alloc).ctx,
                self.cap,
            ) as *mut u8;
            *(self.data).offset(0) = 0;
        }
        need = self.cap;
        while want >= need.wrapping_sub(self.len) {
            need <<= 1;
        }
        if need != self.cap {
            self.data = ((*self.alloc).realloc).expect("non-null function pointer")(
                (*self.alloc).ctx,
                self.data as *mut c_void,
                need,
            ) as *mut u8;
            self.cap = need;
        }
    }

    pub unsafe fn alloc(mut alloc: *mut yajl_alloc_funcs) -> *mut Buffer {
        let mut b: *mut Buffer = ((*alloc).malloc).expect("non-null function pointer")(
            (*alloc).ctx,
            ::core::mem::size_of::<Buffer>(),
        ) as *mut Buffer;

        (*b).alloc = alloc;
        (*b).cap = 0;
        (*b).len = 0;
        (*b).data = ptr::null_mut();
        b
    }

    pub fn free(mut buf: *mut Buffer) {
        unsafe {
            if !(*buf).data.is_null() {
                ((*(*buf).alloc).free).expect("non-null function pointer")(
                    (*(*buf).alloc).ctx,
                    (*buf).data as *mut c_void,
                );
            }
            ((*(*buf).alloc).free).expect("non-null function pointer")(
                (*(*buf).alloc).ctx,
                buf as *mut c_void,
            );
        }
    }

    pub unsafe fn append(&mut self, mut data: *const c_void, mut len: usize) {
        self.ensure_available(len);
        if len > 0 {
            libc::memcpy((self.data).add(self.len) as *mut c_void, data, len);
            self.len = (self.len).wrapping_add(len);
            *(self.data).add(self.len) = 0;
        }
    }

    pub unsafe fn clear(&mut self) {
        self.len = 0;
        if !(self.data).is_null() {
            *(self.data).add(self.len) = 0;
        }
    }

    pub fn data(&self) -> *const libc::c_uchar {
        self.data
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn truncate(&mut self, mut len: usize) {
        self.len = len;
    }
}

pub(crate) unsafe extern "C" fn yajl_buf_append(buf: *mut Buffer, data: *const c_void, len: usize) {
    (*buf).append(data, len)
}
