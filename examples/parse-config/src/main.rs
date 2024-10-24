use std::io::{self, Read};

use ::libc;
use yajl::yajl_tree::{yajl_t_string, yajl_tree_free, yajl_tree_get, yajl_tree_parse, yajl_val};

unsafe fn main_0() -> libc::c_int {
    let mut file_data: [libc::c_uchar; 65536] = [0; 65536];

    let mut errbuf: [libc::c_char; 1024] = [0; 1024];
    errbuf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    file_data[0 as libc::c_int as usize] = errbuf[0 as libc::c_int as usize] as libc::c_uchar;

    let mut stdin = io::stdin();
    let rd = match stdin.read(&mut file_data) {
        Err(err) => {
            eprintln!("Error encountered on file read: {:?}", err);
            return 1;
        }
        Ok(rd) => rd,
    };

    if rd >= (::core::mem::size_of::<[libc::c_uchar; 65536]>()).wrapping_sub(1) {
        eprintln!("config file too big");
        return 1 as libc::c_int;
    }
    let node = yajl_tree_parse(
        file_data.as_mut_ptr() as *const libc::c_char,
        errbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>(),
    );
    if node.is_null() {
        eprint!("parse_error: ");
        if libc::strlen(errbuf.as_mut_ptr()) != 0 {
            eprintln!(
                "{}",
                String::from_utf8_lossy(unsafe { &*(&errbuf[..] as *const _ as *const [u8]) })
            );
        } else {
            eprintln!("unknown error");
        }
        return 1 as libc::c_int;
    }
    let mut path: [*const libc::c_char; 3] = [
        b"Logging\0" as *const u8 as *const libc::c_char,
        b"timeFormat\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let v: yajl_val = yajl_tree_get(node, path.as_mut_ptr(), yajl_t_string);
    if !v.is_null() {
        libc::printf(
            b"%s/%s: %s\n\0" as *const u8 as *const libc::c_char,
            path[0 as libc::c_int as usize],
            path[1 as libc::c_int as usize],
            if !v.is_null()
                && (*v).type_0 as libc::c_uint == yajl_t_string as libc::c_int as libc::c_uint
            {
                (*v).u.string
            } else {
                0 as *mut libc::c_char
            },
        );
    } else {
        libc::printf(
            b"no such node: %s/%s\n\0" as *const u8 as *const libc::c_char,
            path[0 as libc::c_int as usize],
            path[1 as libc::c_int as usize],
        );
    }
    yajl_tree_free(node);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
