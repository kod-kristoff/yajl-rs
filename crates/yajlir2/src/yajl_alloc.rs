use ::libc;

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
