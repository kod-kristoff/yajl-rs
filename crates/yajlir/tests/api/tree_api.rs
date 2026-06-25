use std::{
    ffi::{c_char, CStr},
    fs, ptr,
};

use rstest::{fixture, rstest};
use yajl::tree::{yajl_tree_get, yajl_tree_parse, Value, ValueType};

use crate::shared::FreeGuard;

#[fixture]
fn sample_config_data() -> Vec<u8> {
    let mut file_data: Vec<u8> = fs::read("assets/sample.config").unwrap();
    file_data.push(0);
    file_data
}
#[rstest]
#[case(0)]
#[case(128)]
fn tree_parse_and_get_number(#[case] buffer_size: usize, sample_config_data: Vec<u8>) {
    let mut error_buffer = vec![0; buffer_size];
    let node = unsafe {
        yajl_tree_parse(
            sample_config_data.as_ptr() as *const i8,
            if buffer_size == 0 {
                ptr::null_mut()
            } else {
                error_buffer.as_mut_ptr()
            },
            buffer_size,
        )
    }
    .unwrap();
    assert!(!node.is_null());
    unsafe { dbg!(&*node) };

    let _guard = FreeGuard::new(node, Value::tree_free);

    for (mut path, expected_value) in [(
        [
            b"Logging\0" as *const u8 as *const c_char,
            b"fileRolloverKB\0" as *const u8 as *const c_char,
            ptr::null(),
        ],
        2048,
    )] {
        let val = unsafe { yajl_tree_get(node, path.as_mut_ptr(), ValueType::Number) }.unwrap();
        assert!(!val.is_null());
        unsafe { dbg!(&*val) };
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
            sample_config_data.as_ptr() as *const i8,
            if buffer_size == 0 {
                ptr::null_mut()
            } else {
                error_buffer.as_mut_ptr()
            },
            error_buffer.len(),
        )
    }
    .unwrap();
    if buffer_size > 0 {
        unsafe {
            dbg!(CStr::from_ptr(error_buffer.as_ptr().cast())
                .to_str()
                .unwrap());
        }
    }
    assert!(!node.is_null());
    unsafe { dbg!(&*node) };
    let _guard = FreeGuard::new(node, Value::tree_free);

    for (mut path, expected_value) in [(
        [
            b"Logging\0" as *const u8 as *const c_char,
            b"level\0" as *const u8 as *const c_char,
            ptr::null(),
        ],
        "BP_LOG_LEVEL",
    )] {
        let val = unsafe { yajl_tree_get(node, path.as_mut_ptr(), ValueType::String) }.unwrap();
        assert!(!val.is_null());
        let actual = unsafe {
            CStr::from_ptr((*val).u.string as *const i8)
                .to_str()
                .unwrap()
        };
        assert_eq!(actual, expected_value);
    }
}

#[test]
fn yajl_tree_get_fails_when_passing_null_as_input() {
    let mut path: [*const c_char; 3] = [
        b"Logging\0" as *const u8 as *const c_char,
        b"fileRolloverKB\0" as *const u8 as *const c_char,
        ptr::null(),
    ];
    let val = unsafe { yajl_tree_get(ptr::null_mut(), path.as_mut_ptr(), ValueType::Any) };
    assert!(val.is_none());
}

#[rstest]
fn yajl_tree_get_fails_when_passing_null_as_path(sample_config_data: Vec<u8>) {
    let node =
        unsafe { yajl_tree_parse(sample_config_data.as_ptr() as *const i8, ptr::null_mut(), 0) }
            .unwrap();
    assert!(!node.is_null());
    let _guard = FreeGuard::new(node, Value::tree_free);

    let val = unsafe { yajl_tree_get(node, ptr::null_mut(), ValueType::Any) };
    assert!(val.is_none());
}
