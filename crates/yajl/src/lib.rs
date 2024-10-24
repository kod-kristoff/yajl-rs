#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(clippy::missing_safety_doc)]
#![cfg_attr(feature = "nightly", feature(c_variadic))]
#![cfg_attr(feature = "nightly", feature(extern_types))]

extern crate libc;
pub mod yajl;
pub mod yajl_alloc;
pub mod yajl_buf;
pub mod yajl_encode;
pub mod yajl_gen;
pub mod yajl_lex;
pub mod yajl_option;
pub mod yajl_parser;
pub mod yajl_status;
pub mod yajl_tree;
pub mod yajl_version;

pub use yajl::yajl_parse;
