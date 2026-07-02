extern "C" {
    fn yajl_tree_parse(
        input: *const c_char,
        error_buffer: *mut c_char,
        error_buffer_size: usize,
    ) -> yajl_val;
    fn yajl_tree_get(n: yajl_val, path: *mut *const c_char, type_0: yajl_type) -> yajl_val;

    fn yajl_tree_free(v: yajl_val);
}
use std::{
    ffi::{c_char, CStr},
    fs, ptr,
};

use rstest::{fixture, rstest};
use yajl_ffi_test_suite::{
    yajl_t_any, yajl_t_number, yajl_t_string, yajl_type, yajl_val, FreeGuard,
};

#[fixture]
fn sample_config_data() -> Vec<u8> {
    let file_data: Vec<u8> = fs::read("assets/sample.config").unwrap();
    file_data
}
#[rstest]
#[case(0)]
#[case(128)]
fn tree_parse_and_get_number(#[case] buffer_size: usize, sample_config_data: Vec<u8>) {
    let mut error_buffer = vec![0; buffer_size];
    let node = unsafe {
        yajl_tree_parse(
            sample_config_data.as_ptr() as *const c_char,
            if buffer_size == 0 {
                ptr::null_mut()
            } else {
                error_buffer.as_mut_ptr()
            },
            error_buffer.len(),
        )
    };
    assert!(!node.is_null());
    let _guard = FreeGuard::new(node, yajl_tree_free);

    for (mut path, expected_value) in [(
        [
            b"Logging\0" as *const u8 as *const c_char,
            b"fileRolloverKB\0" as *const u8 as *const c_char,
            ptr::null(),
        ],
        2048,
    )] {
        let val = unsafe { yajl_tree_get(node, path.as_mut_ptr(), yajl_t_number) };
        assert!(!val.is_null());
        let actual = unsafe { (*val).u.number.i };
        assert_eq!(actual, expected_value);
    }
}
#[rstest]
#[case(0)]
#[case(128)]
fn tree_parse_and_get_string(#[case] buffer_size: usize, sample_config_data: Vec<u8>) {
    let mut error_buffer = vec![0; buffer_size];
    let node = unsafe {
        yajl_tree_parse(
            sample_config_data.as_ptr() as *const c_char,
            if buffer_size == 0 {
                ptr::null_mut()
            } else {
                error_buffer.as_mut_ptr()
            },
            error_buffer.len(),
        )
    };
    if buffer_size > 0 {
        unsafe {
            dbg!(CStr::from_ptr(error_buffer.as_ptr().cast())
                .to_str()
                .unwrap());
        }
    }
    assert!(!node.is_null());
    let _guard = FreeGuard::new(node, yajl_tree_free);

    for (mut path, expected_value) in [(
        [
            b"Logging\0" as *const u8 as *const c_char,
            b"level\0" as *const u8 as *const c_char,
            ptr::null(),
        ],
        "BP_LOG_LEVEL",
    )] {
        let val = unsafe { yajl_tree_get(node, path.as_mut_ptr(), yajl_t_string) };
        assert!(!val.is_null());
        let actual = unsafe {
            CStr::from_ptr((*val).u.string as *const c_char)
                .to_str()
                .unwrap()
        };
        assert_eq!(actual, expected_value);
    }
}

#[rstest]
#[case(1)]
#[case(2)]
#[case(3)]
#[case(4)]
#[case(5)]
#[case(6)]
#[case(20)]
#[case(40)]
fn yajl_tree_parse_fails_when_passing_null_as_input(#[case] buffer_size: usize) {
    let mut error_buffer = vec![0u8; buffer_size];

    let node = unsafe {
        yajl_tree_parse(
            ptr::null(),
            error_buffer.as_mut_ptr() as *mut c_char,
            error_buffer.len(),
        )
    };
    assert!(node.is_null());

    let msg = CStr::from_bytes_until_nul(&error_buffer[..])
        .unwrap()
        .to_str()
        .unwrap();
    insta::assert_debug_snapshot!(
        format!(
            "yajl_tree_parse_fails_when_passing_null_as_input-case-{}",
            buffer_size
        ),
        msg
    );
}

#[rstest]
#[case(0)]
#[case(1)]
#[case(2)]
fn yajl_tree_parse_fails_when_passing_null_as_input_and_as_error_nuffer(
    #[case] buffer_size: usize,
) {
    let node = unsafe { yajl_tree_parse(ptr::null(), ptr::null_mut(), buffer_size) };
    assert!(node.is_null());
}

#[test]
fn yajl_tree_get_fails_when_passing_null_as_input() {
    let mut path: [*const c_char; 3] = [
        b"Logging\0" as *const u8 as *const c_char,
        b"fileRolloverKB\0" as *const u8 as *const c_char,
        ptr::null(),
    ];
    let val = unsafe { yajl_tree_get(ptr::null_mut(), path.as_mut_ptr(), yajl_t_any) };
    assert!(val.is_null());
}

#[rstest]
fn yajl_tree_get_fails_when_passing_null_as_path(sample_config_data: Vec<u8>) {
    let node = unsafe {
        yajl_tree_parse(
            sample_config_data.as_ptr() as *const c_char,
            ptr::null_mut(),
            0,
        )
    };
    assert!(!node.is_null());
    let _guard = FreeGuard::new(node, yajl_tree_free);

    let val = unsafe { yajl_tree_get(node, ptr::null_mut(), yajl_t_any) };
    assert!(val.is_null());
}

#[rstest]
fn yajl_tree_get_fails_when_passing_too_large_yajl_type(sample_config_data: Vec<u8>) {
    let node = unsafe {
        yajl_tree_parse(
            sample_config_data.as_ptr() as *const c_char,
            ptr::null_mut(),
            0,
        )
    };
    assert!(!node.is_null());
    let _guard = FreeGuard::new(node, yajl_tree_free);

    let val = unsafe { yajl_tree_get(node, ptr::null_mut(), 129) };
    assert!(val.is_null());
}
