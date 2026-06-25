use ::libc;

use crate::buffer::Buffer;

type yajl_buf = *mut Buffer;
pub type yajl_print_t =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, usize) -> ()>;
unsafe extern "C" fn CharToHex(mut c: libc::c_uchar, mut hexBuf: *mut libc::c_char) {
    let mut hexchar: *const libc::c_char =
        b"0123456789ABCDEF\0" as *const u8 as *const libc::c_char;
    *hexBuf.offset(0 as libc::c_int as isize) =
        *hexchar.offset((c as libc::c_int >> 4 as libc::c_int) as isize);
    *hexBuf.offset(1 as libc::c_int as isize) =
        *hexchar.offset((c as libc::c_int & 0xf as libc::c_int) as isize);
}

pub unsafe extern "C" fn yajl_string_encode(
    print: yajl_print_t,
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_uchar,
    mut len: usize,
    mut escape_solidus: libc::c_int,
) {
    let mut beg: usize = 0 as libc::c_int as usize;
    let mut end: usize = 0 as libc::c_int as usize;
    let mut hexBuf: [libc::c_char; 7] = [0; 7];
    hexBuf[0 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
    hexBuf[1 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    hexBuf[2 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    hexBuf[3 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    hexBuf[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    while end < len {
        let mut escaped: *const libc::c_char = std::ptr::null::<libc::c_char>();
        match *str.add(end) as libc::c_int {
            13 => {
                escaped = b"\\r\0" as *const u8 as *const libc::c_char;
            }
            10 => {
                escaped = b"\\n\0" as *const u8 as *const libc::c_char;
            }
            92 => {
                escaped = b"\\\\\0" as *const u8 as *const libc::c_char;
            }
            47 => {
                if escape_solidus != 0 {
                    escaped = b"\\/\0" as *const u8 as *const libc::c_char;
                }
            }
            34 => {
                escaped = b"\\\"\0" as *const u8 as *const libc::c_char;
            }
            12 => {
                escaped = b"\\f\0" as *const u8 as *const libc::c_char;
            }
            8 => {
                escaped = b"\\b\0" as *const u8 as *const libc::c_char;
            }
            9 => {
                escaped = b"\\t\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                if (*str.add(end) as libc::c_int) < 32 as libc::c_int {
                    CharToHex(
                        *str.add(end),
                        hexBuf.as_mut_ptr().offset(4 as libc::c_int as isize),
                    );
                    escaped = hexBuf.as_mut_ptr();
                }
            }
        }
        if !escaped.is_null() {
            print.expect("non-null function pointer")(
                ctx,
                str.add(beg) as *const libc::c_char,
                end.wrapping_sub(beg),
            );
            print.expect("non-null function pointer")(
                ctx,
                escaped,
                libc::strlen(escaped) as libc::c_uint as usize,
            );
            end = end.wrapping_add(1);
            beg = end;
        } else {
            end = end.wrapping_add(1);
        }
    }
    print.expect("non-null function pointer")(
        ctx,
        str.add(beg) as *const libc::c_char,
        end.wrapping_sub(beg),
    );
}
unsafe extern "C" fn hexToDigit(mut val: *mut libc::c_uint, mut hex: *const libc::c_uchar) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        let mut c: libc::c_uchar = *hex.offset(i as isize);
        if c as libc::c_int >= 'A' as i32 {
            c = ((c as libc::c_int & !(0x20 as libc::c_int)) - 7 as libc::c_int) as libc::c_uchar;
        }
        c = (c as libc::c_int - '0' as i32) as libc::c_uchar;
        *val = *val << 4 as libc::c_int | c as libc::c_uint;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn Utf32toUtf8(mut codepoint: libc::c_uint, mut utf8Buf: *mut libc::c_char) {
    if codepoint < 0x80 as libc::c_int as libc::c_uint {
        *utf8Buf.offset(0 as libc::c_int as isize) = codepoint as libc::c_char;
        *utf8Buf.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else if codepoint < 0x800 as libc::c_int as libc::c_uint {
        *utf8Buf.offset(0 as libc::c_int as isize) =
            (codepoint >> 6 as libc::c_int | 0xc0 as libc::c_int as libc::c_uint) as libc::c_char;
        *utf8Buf.offset(1 as libc::c_int as isize) =
            (codepoint & 0x3f as libc::c_int as libc::c_uint | 0x80 as libc::c_int as libc::c_uint)
                as libc::c_char;
        *utf8Buf.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else if codepoint < 0x10000 as libc::c_int as libc::c_uint {
        *utf8Buf.offset(0 as libc::c_int as isize) =
            (codepoint >> 12 as libc::c_int | 0xe0 as libc::c_int as libc::c_uint) as libc::c_char;
        *utf8Buf.offset(1 as libc::c_int as isize) =
            (codepoint >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        *utf8Buf.offset(2 as libc::c_int as isize) =
            (codepoint & 0x3f as libc::c_int as libc::c_uint | 0x80 as libc::c_int as libc::c_uint)
                as libc::c_char;
        *utf8Buf.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else if codepoint < 0x200000 as libc::c_int as libc::c_uint {
        *utf8Buf.offset(0 as libc::c_int as isize) =
            (codepoint >> 18 as libc::c_int | 0xf0 as libc::c_int as libc::c_uint) as libc::c_char;
        *utf8Buf.offset(1 as libc::c_int as isize) =
            (codepoint >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        *utf8Buf.offset(2 as libc::c_int as isize) =
            (codepoint >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint
                | 0x80 as libc::c_int as libc::c_uint) as libc::c_char;
        *utf8Buf.offset(3 as libc::c_int as isize) =
            (codepoint & 0x3f as libc::c_int as libc::c_uint | 0x80 as libc::c_int as libc::c_uint)
                as libc::c_char;
        *utf8Buf.offset(4 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        *utf8Buf.offset(0 as libc::c_int as isize) = '?' as i32 as libc::c_char;
        *utf8Buf.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    };
}

pub unsafe extern "C" fn yajl_string_decode(
    mut buf: *mut Buffer,
    mut str: *const libc::c_uchar,
    mut len: usize,
) {
    let mut beg: usize = 0 as libc::c_int as usize;
    let mut end: usize = 0 as libc::c_int as usize;
    let mut current_block_25: u64;
    while end < len {
        if *str.add(end) as libc::c_int == '\\' as i32 {
            let mut utf8Buf: [libc::c_char; 5] = [0; 5];
            let mut unescaped: *const libc::c_char = b"?\0" as *const u8 as *const libc::c_char;
            (*buf).append(str.add(beg) as *const libc::c_void, end.wrapping_sub(beg));
            end = end.wrapping_add(1);
            match *str.add(end) as libc::c_int {
                114 => {
                    unescaped = b"\r\0" as *const u8 as *const libc::c_char;
                }
                110 => {
                    unescaped = b"\n\0" as *const u8 as *const libc::c_char;
                }
                92 => {
                    unescaped = b"\\\0" as *const u8 as *const libc::c_char;
                }
                47 => {
                    unescaped = b"/\0" as *const u8 as *const libc::c_char;
                }
                34 => {
                    unescaped = b"\"\0" as *const u8 as *const libc::c_char;
                }
                102 => {
                    unescaped = b"\x0C\0" as *const u8 as *const libc::c_char;
                }
                98 => {
                    unescaped = b"\x08\0" as *const u8 as *const libc::c_char;
                }
                116 => {
                    unescaped = b"\t\0" as *const u8 as *const libc::c_char;
                }
                117 => {
                    let mut codepoint: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    end = end.wrapping_add(1);
                    hexToDigit(&mut codepoint, str.add(end));
                    end = end.wrapping_add(3 as libc::c_int as usize);
                    if codepoint & 0xfc00 as libc::c_int as libc::c_uint
                        == 0xd800 as libc::c_int as libc::c_uint
                    {
                        end = end.wrapping_add(1);
                        if *str.add(end) as libc::c_int == '\\' as i32
                            && *str.add(end.wrapping_add(1 as libc::c_int as usize)) as libc::c_int
                                == 'u' as i32
                        {
                            let mut surrogate: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                            hexToDigit(
                                &mut surrogate,
                                str.add(end).offset(2 as libc::c_int as isize),
                            );
                            codepoint = (codepoint & 0x3f as libc::c_int as libc::c_uint)
                                << 10 as libc::c_int
                                | (codepoint >> 6 as libc::c_int
                                    & 0xf as libc::c_int as libc::c_uint)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                                    << 16 as libc::c_int
                                | surrogate & 0x3ff as libc::c_int as libc::c_uint;
                            end = end.wrapping_add(5 as libc::c_int as usize);
                            current_block_25 = 13472856163611868459;
                        } else {
                            unescaped = b"?\0" as *const u8 as *const libc::c_char;
                            current_block_25 = 11459959175219260272;
                        }
                    } else {
                        current_block_25 = 13472856163611868459;
                    }
                    match current_block_25 {
                        11459959175219260272 => {}
                        _ => {
                            Utf32toUtf8(codepoint, utf8Buf.as_mut_ptr());
                            unescaped = utf8Buf.as_mut_ptr();
                            if codepoint == 0 as libc::c_int as libc::c_uint {
                                (*buf).append(
                                    unescaped as *const libc::c_void,
                                    1 as libc::c_int as usize,
                                );
                                end = end.wrapping_add(1);
                                beg = end;
                                continue;
                            }
                        }
                    }
                }
                _ => {}
            }
            (*buf).append(
                unescaped as *const libc::c_void,
                libc::strlen(unescaped) as libc::c_uint as usize,
            );
            end = end.wrapping_add(1);
            beg = end;
        } else {
            end = end.wrapping_add(1);
        }
    }
    (*buf).append(str.add(beg) as *const libc::c_void, end.wrapping_sub(beg));
}

pub unsafe extern "C" fn yajl_string_validate_utf8(
    mut s: *const libc::c_uchar,
    mut len: usize,
) -> libc::c_int {
    if len == 0 {
        return 1 as libc::c_int;
    }
    if s.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if fresh0 == 0 {
            break;
        }
        if *s as libc::c_int > 0x7f as libc::c_int {
            if *s as libc::c_int >> 5 as libc::c_int == 0x6 as libc::c_int {
                s = s.offset(1);
                let fresh1 = len;
                len = len.wrapping_sub(1);
                if fresh1 == 0 {
                    return 0 as libc::c_int;
                }
                if *s as libc::c_int >> 6 as libc::c_int != 0x2 as libc::c_int {
                    return 0 as libc::c_int;
                }
            } else if *s as libc::c_int >> 4 as libc::c_int == 0xe as libc::c_int {
                s = s.offset(1);
                let fresh2 = len;
                len = len.wrapping_sub(1);
                if fresh2 == 0 {
                    return 0 as libc::c_int;
                }
                if *s as libc::c_int >> 6 as libc::c_int != 0x2 as libc::c_int {
                    return 0 as libc::c_int;
                }
                s = s.offset(1);
                let fresh3 = len;
                len = len.wrapping_sub(1);
                if fresh3 == 0 {
                    return 0 as libc::c_int;
                }
                if *s as libc::c_int >> 6 as libc::c_int != 0x2 as libc::c_int {
                    return 0 as libc::c_int;
                }
            } else if *s as libc::c_int >> 3 as libc::c_int == 0x1e as libc::c_int {
                s = s.offset(1);
                let fresh4 = len;
                len = len.wrapping_sub(1);
                if fresh4 == 0 {
                    return 0 as libc::c_int;
                }
                if *s as libc::c_int >> 6 as libc::c_int != 0x2 as libc::c_int {
                    return 0 as libc::c_int;
                }
                s = s.offset(1);
                let fresh5 = len;
                len = len.wrapping_sub(1);
                if fresh5 == 0 {
                    return 0 as libc::c_int;
                }
                if *s as libc::c_int >> 6 as libc::c_int != 0x2 as libc::c_int {
                    return 0 as libc::c_int;
                }
                s = s.offset(1);
                let fresh6 = len;
                len = len.wrapping_sub(1);
                if fresh6 == 0 {
                    return 0 as libc::c_int;
                }
                if *s as libc::c_int >> 6 as libc::c_int != 0x2 as libc::c_int {
                    return 0 as libc::c_int;
                }
            } else {
                return 0 as libc::c_int;
            }
        }
        s = s.offset(1);
    }
    1 as libc::c_int
}
