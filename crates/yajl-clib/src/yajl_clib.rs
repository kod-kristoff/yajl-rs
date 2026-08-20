#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]

#[allow(non_camel_case_types)]
pub type yajl_handle_t = yajl::parser::Parser;

pub use yajl::parser::Parser;

pub use yajl::yajl_alloc::yajl_alloc_funcs;

pub mod yajl_gen;
pub mod yajl_parse;
pub mod yajl_tree;
pub mod yajl_version;
