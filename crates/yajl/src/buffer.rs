use core::ffi::c_void;
use core::ptr;
use core::ptr::NonNull;
use core::slice;

use crate::yajl_alloc::yajl_alloc_funcs;

pub struct Buffer {
    cap: usize,
    len: usize,
    data: NonNull<u8>,
    alloc: *mut yajl_alloc_funcs,
}
impl Buffer {
    fn ensure_available(&mut self, want: usize) {
        let mut need: usize = 0;
        if self.cap == 0 {
            self.cap = 2048;
            unsafe {
                let raw_data = ((*self.alloc).malloc).expect("non-null function pointer")(
                    (*self.alloc).ctx,
                    self.cap,
                ) as *mut u8;
                self.data = NonNull::new(raw_data).expect("non-null pointer");
                self.data.as_ptr().add(0).write(0);
            }
        }
        need = self.cap;
        while want >= need.wrapping_sub(self.len) {
            need <<= 1;
        }
        if need != self.cap {
            unsafe {
                let raw_data = ((*self.alloc).realloc).expect("non-null function pointer")(
                    (*self.alloc).ctx,
                    self.data.as_ptr() as *mut c_void,
                    need,
                ) as *mut u8;

                self.data = NonNull::new(raw_data).expect("non-null pointer");
            }
            self.cap = need;
        }
    }

    pub fn new(alloc: *mut yajl_alloc_funcs) -> Self {
        Self {
            cap: 0,
            len: 0,
            data: NonNull::dangling(),
            alloc,
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
        (*b).data = NonNull::dangling();
        b
    }

    pub fn free(mut buf: *mut Buffer) {
        unsafe {
            if (*buf).cap > 0 {
                ((*(*buf).alloc).free).expect("non-null function pointer")(
                    (*(*buf).alloc).ctx,
                    (*buf).data.as_ptr() as *mut c_void,
                );
            }
            ((*(*buf).alloc).free).expect("non-null function pointer")(
                (*(*buf).alloc).ctx,
                buf as *mut c_void,
            );
        }
    }

    pub(crate) unsafe fn reset(&mut self) {
        eprintln!("Buffer::reset called");
        unsafe {
            if self.cap > 0 {
                ((*self.alloc).free).expect("non-null function pointer")(
                    (*self.alloc).ctx,
                    self.data.as_ptr() as *mut c_void,
                );
                self.data = NonNull::dangling();
                self.cap = 0;
                self.len = 0;
            }
        }
    }

    pub fn append(&mut self, data: *const c_void, len: usize) {
        self.ensure_available(len);
        if len > 0 {
            unsafe {
                ptr::copy(data, self.data.as_ptr() as *mut c_void, len);
                self.len += len;
                self.data.as_ptr().add(self.len).write(0);
            }
        }
    }

    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.ensure_available(data.len());
        if !data.is_empty() {
            unsafe {
                ptr::copy(data.as_ptr(), self.data.as_ptr(), data.len());
                self.len += data.len();
                self.data.as_ptr().add(self.len).write(0);
            }
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        if self.cap > 0 {
            unsafe {
                self.data.as_ptr().add(self.len).write(0);
            }
        }
    }

    pub fn data(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data.as_ptr(), self.len) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn truncate(&mut self, len: usize) {
        self.len = len;
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        eprintln!("Buffer::drop called");
        unsafe {
            if self.cap > 0 {
                ((*self.alloc).free).expect("non-null function pointer")(
                    (*self.alloc).ctx,
                    self.data.as_ptr() as *mut c_void,
                );
            }
        }
    }
}

pub(crate) unsafe extern "C" fn yajl_buf_append(buf: *mut Buffer, data: *const c_void, len: usize) {
    (*buf).append(data, len)
}
