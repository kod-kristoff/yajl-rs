use std::ptr;

pub struct FreeGuard<T> {
    p: *mut T,
    free: unsafe fn(*mut T),
}
impl<T> FreeGuard<T> {
    pub fn new(p: *mut T, free: unsafe fn(*mut T)) -> Self {
        Self { p, free }
    }
}

impl<T> Drop for FreeGuard<T> {
    fn drop(&mut self) {
        if !self.p.is_null() {
            unsafe {
                (self.free)(self.p);
            }
            self.p = ptr::null_mut();
        }
    }
}
