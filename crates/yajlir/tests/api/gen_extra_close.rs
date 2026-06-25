use yajl::yajl_gen::{
    yajl_gen_alloc, yajl_gen_free, yajl_gen_generation_complete, yajl_gen_map_close,
    yajl_gen_map_open, yajl_gen_status_ok,
};

#[test]
fn gen_extra_close() {
    let yg = unsafe { yajl_gen_alloc(std::ptr::null()) };
    assert_eq!(unsafe { yajl_gen_map_open(yg) }, yajl_gen_status_ok);
    assert_eq!(unsafe { yajl_gen_map_close(yg) }, yajl_gen_status_ok);

    let s = unsafe { yajl_gen_map_close(yg) };
    assert_eq!(yajl_gen_generation_complete, s);
    unsafe { yajl_gen_free(yg) };
}
