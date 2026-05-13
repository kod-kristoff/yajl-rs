#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Status {
    Ok = 0,
    ClientCanceled = 1,
    Error = 2,
}
