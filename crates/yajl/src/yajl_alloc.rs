use ::libc;
use core::ffi::c_void;

pub trait AllocFuncs {
    unsafe fn malloc_func(&self, size: usize) -> *mut libc::c_void;
    unsafe fn free_func(&self, p: *mut libc::c_void);
    unsafe fn realloc_func(&self, p: *mut libc::c_void, size: usize) -> *mut libc::c_void;
}

pub type yajl_malloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void>;
pub type yajl_free_func = Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type yajl_realloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, usize) -> *mut libc::c_void>;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut libc::c_void,
}
pub unsafe trait AllocFuncs {
    fn malloc(len: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, len: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

impl AllocFuncs for yajl_alloc_funcs {
    unsafe fn malloc_func(&self, size: usize) -> *mut libc::c_void {
        debug_assert!(!self.malloc.is_none());
        (self.malloc.expect("non-none fn pointer"))(self.ctx, size)
    }
    unsafe fn free_func(&self, p: *mut libc::c_void) {
        debug_assert!(!self.free.is_none());
        (self.free.expect("non-none fn pointer"))(self.ctx, p)
    }
    unsafe fn realloc_func(&self, p: *mut libc::c_void, size: usize) -> *mut libc::c_void {
        debug_assert!(!self.realloc.is_none());
        (self.realloc.expect("non-none fn pointer"))(self.ctx, p, size)
    }
}

unsafe extern "C" fn yajl_internal_malloc(
    mut ctx: *mut libc::c_void,
    mut sz: usize,
) -> *mut libc::c_void {
    libc::malloc(sz)
}
unsafe extern "C" fn yajl_internal_realloc(
    mut ctx: *mut libc::c_void,
    mut previous: *mut libc::c_void,
    mut sz: usize,
) -> *mut libc::c_void {
    libc::realloc(previous, sz)
}
unsafe extern "C" fn yajl_internal_free(mut ctx: *mut libc::c_void, mut ptr: *mut libc::c_void) {
    libc::free(ptr);
}

pub unsafe extern "C" fn yajl_set_default_alloc_funcs(mut yaf: *mut yajl_alloc_funcs) {
    (*yaf).malloc = Some(
        yajl_internal_malloc as unsafe extern "C" fn(*mut libc::c_void, usize) -> *mut libc::c_void,
    );
    (*yaf).free = Some(
        yajl_internal_free as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    );
    (*yaf).realloc = Some(
        yajl_internal_realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_void,
                usize,
            ) -> *mut libc::c_void,
    );
    (*yaf).ctx = std::ptr::null_mut::<libc::c_void>();
}
