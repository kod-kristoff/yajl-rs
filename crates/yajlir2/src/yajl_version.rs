pub const YAJL_MAJOR: i32 = 2;
pub const YAJL_MINOR: i32 = 1;
pub const YAJL_MICRO: i32 = 2;

pub fn yajl_version() -> i32 {
    YAJL_MAJOR * 10000 + YAJL_MINOR * 100 + YAJL_MICRO
}
