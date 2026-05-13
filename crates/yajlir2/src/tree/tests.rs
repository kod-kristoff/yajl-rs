use std::ptr::{self, addr_of_mut};

use rstest::rstest;

use super::{
    handle_boolean, handle_end_array, handle_end_map, handle_null, handle_number,
    handle_start_array, handle_start_map, handle_string,
};

use super::context::Context;

#[rstest]
#[case(0)]
#[case(1)]
fn handle_boolean_works(#[case] bool_value: i32) {
    let mut ctx = Context::new(ptr::null_mut(), 0);

    let status = unsafe { handle_boolean(addr_of_mut!(ctx).cast(), bool_value) };

    assert_ne!(status, 0);

    insta::assert_debug_snapshot!(format!("handle_boolean_works-{}", bool_value), ctx);
}
#[test]
fn handle_string_works() {
    let mut ctx = Context::new(ptr::null_mut(), 0);

    let status = unsafe { handle_string(addr_of_mut!(ctx).cast(), b"key\0".as_ptr(), 3) };

    assert_ne!(status, 0);

    insta::assert_debug_snapshot!(ctx);
}
#[rstest]
#[case(&b"123"[..],0)]
#[case(&b"123"[..],128)]
#[case(&b"-123"[..],0)]
#[case(&b"-123"[..],128)]
#[case(&b"-123"[..],0)]
#[case(&b"-123"[..],128)]
#[case(&b"-12.3"[..],0)]
#[case(&b"-12.3"[..],128)]
#[case(&b"nan"[..],0)]
#[case(&b"nan"[..],128)]
fn handle_number_works(#[case] num_string: &[u8], #[case] buffer_size: usize) {
    let mut error_buffer = vec![0; buffer_size];
    let mut ctx = Context::new(
        if buffer_size > 0 {
            error_buffer.as_mut_ptr()
        } else {
            ptr::null_mut()
        },
        buffer_size,
    );

    let status = unsafe {
        handle_number(
            addr_of_mut!(ctx).cast(),
            num_string.as_ptr().cast(),
            num_string.len(),
        )
    };

    assert_ne!(status, 0);

    insta::assert_debug_snapshot!(
        format!(
            "handle_number_works-{}-{}",
            num_string.escape_ascii(),
            buffer_size
        ),
        ctx
    );
}
#[test]
fn handle_null_works() {
    let mut ctx = Context::new(ptr::null_mut(), 0);

    let status = unsafe { handle_null(addr_of_mut!(ctx).cast()) };

    assert_ne!(status, 0);

    insta::assert_debug_snapshot!(ctx);
}

#[test]
fn handle_start_and_end_array_works() {
    let mut ctx = Context::new(ptr::null_mut(), 0);
    let status = unsafe { handle_start_array(addr_of_mut!(ctx).cast()) };
    insta::assert_debug_snapshot!(ctx);
    let status = unsafe { handle_null(addr_of_mut!(ctx).cast()) };
    insta::assert_debug_snapshot!(ctx);
    let status = unsafe { handle_end_array(addr_of_mut!(ctx).cast()) };
    insta::assert_debug_snapshot!(ctx);
}

#[test]
fn handle_start_and_end_map_works() {
    let mut ctx = Context::new(ptr::null_mut(), 0);
    let status = unsafe { handle_start_map(addr_of_mut!(ctx).cast()) };
    insta::assert_debug_snapshot!(ctx);
    let status = unsafe { handle_string(addr_of_mut!(ctx).cast(), b"key\0".as_ptr(), 3) };
    insta::assert_debug_snapshot!(ctx);
    let status = unsafe { handle_boolean(addr_of_mut!(ctx).cast(), 1) };
    insta::assert_debug_snapshot!(ctx);
    let status = unsafe { handle_end_map(addr_of_mut!(ctx).cast()) };
    insta::assert_debug_snapshot!(ctx);
}
