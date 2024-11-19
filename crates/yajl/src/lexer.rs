use ::libc;

use crate::{buffer::Buffer, yajl_alloc::yajl_alloc_funcs};

pub type yajl_tok = libc::c_uint;
pub const yajl_tok_comment: yajl_tok = 14;
pub const yajl_tok_string_with_escapes: yajl_tok = 13;
pub const yajl_tok_string: yajl_tok = 12;
pub const yajl_tok_double: yajl_tok = 11;
pub const yajl_tok_integer: yajl_tok = 10;
pub const yajl_tok_right_bracket: yajl_tok = 9;
pub const yajl_tok_right_brace: yajl_tok = 8;
pub const yajl_tok_null: yajl_tok = 7;
pub const yajl_tok_left_bracket: yajl_tok = 6;
pub const yajl_tok_left_brace: yajl_tok = 5;
pub const yajl_tok_error: yajl_tok = 4;
pub const yajl_tok_eof: yajl_tok = 3;
pub const yajl_tok_comma: yajl_tok = 2;
pub const yajl_tok_colon: yajl_tok = 1;
pub const yajl_tok_bool: yajl_tok = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lexer {
    pub lineOff: usize,
    pub charOff: usize,
    pub error: yajl_lex_error,
    pub buf: *mut Buffer,
    pub bufOff: usize,
    pub bufInUse: libc::c_uint,
    pub allowComments: libc::c_uint,
    pub validateUTF8: libc::c_uint,
    pub alloc: *mut yajl_alloc_funcs,
}
pub type yajl_lex_error = libc::c_uint;
pub const yajl_lex_unallowed_comment: yajl_lex_error = 10;
pub const yajl_lex_missing_integer_after_minus: yajl_lex_error = 9;
pub const yajl_lex_missing_integer_after_exponent: yajl_lex_error = 8;
pub const yajl_lex_missing_integer_after_decimal: yajl_lex_error = 7;
pub const yajl_lex_invalid_string: yajl_lex_error = 6;
pub const yajl_lex_invalid_char: yajl_lex_error = 5;
pub const yajl_lex_string_invalid_hex_char: yajl_lex_error = 4;
pub const yajl_lex_string_invalid_json_char: yajl_lex_error = 3;
pub const yajl_lex_string_invalid_escaped_char: yajl_lex_error = 2;
pub const yajl_lex_string_invalid_utf8: yajl_lex_error = 1;
pub const yajl_lex_e_ok: yajl_lex_error = 0;

impl Lexer {
    pub unsafe fn alloc(
        mut alloc: *mut yajl_alloc_funcs,
        mut allowComments: libc::c_uint,
        mut validateUTF8: libc::c_uint,
    ) -> *mut Lexer {
        let mut lxr: *mut Lexer = ((*alloc).malloc).expect("non-null function pointer")(
            (*alloc).ctx,
            ::core::mem::size_of::<Lexer>(),
        ) as *mut Lexer;
        std::ptr::write_bytes(lxr, 0, 1);

        (*lxr).buf = Buffer::alloc(alloc);
        (*lxr).allowComments = allowComments;
        (*lxr).validateUTF8 = validateUTF8;
        (*lxr).alloc = alloc;
        lxr
    }

    pub unsafe fn free(mut lxr: *mut Lexer) {
        Buffer::free((*lxr).buf);
        ((*(*lxr).alloc).free).expect("non-null function pointer")(
            (*(*lxr).alloc).ctx,
            lxr as *mut libc::c_void,
        );
    }
}
static mut charLookupTable: [libc::c_char; 256] = [
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0x2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    (0x8 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x1 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    (0x8 as libc::c_int | 0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    (0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    0x4 as libc::c_int as libc::c_char,
    (0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
    0x10 as libc::c_int as libc::c_char,
];
impl Lexer {
    unsafe fn utf8_char(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: *mut usize,
        mut curChar: libc::c_uchar,
    ) -> yajl_tok {
        if curChar as libc::c_int <= 0x7f as libc::c_int {
            return yajl_tok_string;
        } else if curChar as libc::c_int >> 5 as libc::c_int == 0x6 as libc::c_int {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            curChar =
                (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
                {
                    let fresh0 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh0) as libc::c_int
                } else {
                    let fresh1 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh1) as libc::c_int
                }) as libc::c_uchar;
            if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                return yajl_tok_string;
            }
        } else if curChar as libc::c_int >> 4 as libc::c_int == 0xe as libc::c_int {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            curChar =
                (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
                {
                    let fresh2 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh2) as libc::c_int
                } else {
                    let fresh3 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh3) as libc::c_int
                }) as libc::c_uchar;
            if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                curChar = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh4 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh4) as libc::c_int
                } else {
                    let fresh5 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh5) as libc::c_int
                }) as libc::c_uchar;
                if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                    return yajl_tok_string;
                }
            }
        } else if curChar as libc::c_int >> 3 as libc::c_int == 0x1e as libc::c_int {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            curChar =
                (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
                {
                    let fresh6 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh6) as libc::c_int
                } else {
                    let fresh7 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh7) as libc::c_int
                }) as libc::c_uchar;
            if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                curChar = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh8 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh8) as libc::c_int
                } else {
                    let fresh9 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh9) as libc::c_int
                }) as libc::c_uchar;
                if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                    if *offset >= jsonTextLen {
                        return yajl_tok_eof;
                    }
                    curChar = (if self.bufInUse != 0
                        && (*self.buf).len() != 0
                        && self.bufOff < (*self.buf).len()
                    {
                        let fresh10 = self.bufOff;
                        self.bufOff = (self.bufOff).wrapping_add(1);
                        *((*self.buf).data()).add(fresh10) as libc::c_int
                    } else {
                        let fresh11 = *offset;
                        *offset = (*offset).wrapping_add(1);
                        *jsonText.add(fresh11) as libc::c_int
                    }) as libc::c_uchar;
                    if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                        return yajl_tok_string;
                    }
                }
            }
        }
        yajl_tok_error
    }
}
unsafe fn yajl_string_scan(
    mut buf: *const libc::c_uchar,
    mut len: usize,
    mut utf8check: libc::c_int,
) -> usize {
    let mut mask: libc::c_uchar = (0x2 as libc::c_int
        | 0x8 as libc::c_int
        | (if utf8check != 0 {
            0x10 as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uchar;
    let mut skip: usize = 0 as libc::c_int as usize;
    while skip < len && charLookupTable[*buf as usize] as libc::c_int & mask as libc::c_int == 0 {
        skip = skip.wrapping_add(1);
        buf = buf.offset(1);
    }
    skip
}
impl Lexer {
    unsafe fn string(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: *mut usize,
    ) -> yajl_tok {
        let mut tok: yajl_tok = yajl_tok_error;
        let mut hasEscapes: libc::c_int = 0 as libc::c_int;
        's_10: loop {
            let mut curChar: libc::c_uchar = 0;
            let mut p: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
            let mut len: usize = 0;
            if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len() {
                p = ((*self.buf).data()).add(self.bufOff);
                len = ((*self.buf).len()).wrapping_sub(self.bufOff);
                self.bufOff = (self.bufOff).wrapping_add(yajl_string_scan(
                    p,
                    len,
                    self.validateUTF8 as libc::c_int,
                )) as usize;
            } else if *offset < jsonTextLen {
                p = jsonText.add(*offset);
                len = jsonTextLen.wrapping_sub(*offset);
                *offset = (*offset).wrapping_add(yajl_string_scan(
                    p,
                    len,
                    self.validateUTF8 as libc::c_int,
                )) as usize;
            }
            if *offset >= jsonTextLen {
                tok = yajl_tok_eof;
                break;
            } else {
                curChar = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh12 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh12) as libc::c_int
                } else {
                    let fresh13 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh13) as libc::c_int
                }) as libc::c_uchar;
                if curChar as libc::c_int == '"' as i32 {
                    tok = yajl_tok_string;
                    break;
                } else if curChar as libc::c_int == '\\' as i32 {
                    hasEscapes = 1 as libc::c_int;
                    if *offset >= jsonTextLen {
                        tok = yajl_tok_eof;
                        break;
                    } else {
                        curChar = (if self.bufInUse != 0
                            && (*self.buf).len() != 0
                            && self.bufOff < (*self.buf).len()
                        {
                            let fresh14 = self.bufOff;
                            self.bufOff = (self.bufOff).wrapping_add(1);
                            *((*self.buf).data()).add(fresh14) as libc::c_int
                        } else {
                            let fresh15 = *offset;
                            *offset = (*offset).wrapping_add(1);
                            *jsonText.add(fresh15) as libc::c_int
                        }) as libc::c_uchar;
                        if curChar as libc::c_int == 'u' as i32 {
                            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                            i = 0 as libc::c_int as libc::c_uint;
                            while i < 4 as libc::c_int as libc::c_uint {
                                if *offset >= jsonTextLen {
                                    tok = yajl_tok_eof;
                                    break 's_10;
                                } else {
                                    curChar = (if self.bufInUse != 0
                                        && (*self.buf).len() != 0
                                        && self.bufOff < (*self.buf).len()
                                    {
                                        let fresh16 = self.bufOff;
                                        self.bufOff = (self.bufOff).wrapping_add(1);
                                        *((*self.buf).data()).add(fresh16) as libc::c_int
                                    } else {
                                        let fresh17 = *offset;
                                        *offset = (*offset).wrapping_add(1);
                                        *jsonText.add(fresh17) as libc::c_int
                                    })
                                        as libc::c_uchar;
                                    if charLookupTable[curChar as usize] as libc::c_int
                                        & 0x4 as libc::c_int
                                        == 0
                                    {
                                        if *offset > 0 {
                                            *offset = (*offset).wrapping_sub(1);
                                        } else {
                                            self.bufOff = (self.bufOff).wrapping_sub(1);
                                        };
                                        self.error = yajl_lex_string_invalid_hex_char;
                                        break 's_10;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                            }
                        } else {
                            if charLookupTable[curChar as usize] as libc::c_int & 0x1 as libc::c_int
                                != 0
                            {
                                continue;
                            }
                            if *offset > 0 {
                                *offset = (*offset).wrapping_sub(1);
                            } else {
                                self.bufOff = (self.bufOff).wrapping_sub(1);
                            };
                            self.error = yajl_lex_string_invalid_escaped_char;
                            break;
                        }
                    }
                } else if charLookupTable[curChar as usize] as libc::c_int & 0x2 as libc::c_int != 0
                {
                    if *offset > 0 {
                        *offset = (*offset).wrapping_sub(1);
                    } else {
                        self.bufOff = (self.bufOff).wrapping_sub(1);
                    };
                    self.error = yajl_lex_string_invalid_json_char;
                    break;
                } else {
                    if self.validateUTF8 == 0 {
                        continue;
                    }
                    let mut t: yajl_tok = self.utf8_char(jsonText, jsonTextLen, offset, curChar);
                    if t as libc::c_uint == yajl_tok_eof as libc::c_int as libc::c_uint {
                        tok = yajl_tok_eof;
                        break;
                    } else {
                        if t as libc::c_uint != yajl_tok_error as libc::c_int as libc::c_uint {
                            continue;
                        }
                        self.error = yajl_lex_string_invalid_utf8;
                        break;
                    }
                }
            }
        }
        if hasEscapes != 0 && tok as libc::c_uint == yajl_tok_string as libc::c_int as libc::c_uint
        {
            tok = yajl_tok_string_with_escapes;
        }
        tok
    }
    unsafe fn number(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: *mut usize,
    ) -> yajl_tok {
        let mut c: libc::c_uchar = 0;
        let mut tok: yajl_tok = yajl_tok_integer;
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        c = (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len() {
            let fresh18 = self.bufOff;
            self.bufOff = (self.bufOff).wrapping_add(1);
            *((*self.buf).data()).add(fresh18) as libc::c_int
        } else {
            let fresh19 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh19) as libc::c_int
        }) as libc::c_uchar;
        if c as libc::c_int == '-' as i32 {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
            {
                let fresh20 = self.bufOff;
                self.bufOff = (self.bufOff).wrapping_add(1);
                *((*self.buf).data()).add(fresh20) as libc::c_int
            } else {
                let fresh21 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh21) as libc::c_int
            }) as libc::c_uchar;
        }
        if c as libc::c_int == '0' as i32 {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
            {
                let fresh22 = self.bufOff;
                self.bufOff = (self.bufOff).wrapping_add(1);
                *((*self.buf).data()).add(fresh22) as libc::c_int
            } else {
                let fresh23 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh23) as libc::c_int
            }) as libc::c_uchar;
        } else if c as libc::c_int >= '1' as i32 && c as libc::c_int <= '9' as i32 {
            loop {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh24 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh24) as libc::c_int
                } else {
                    let fresh25 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh25) as libc::c_int
                }) as libc::c_uchar;
                if !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32) {
                    break;
                }
            }
        } else {
            if *offset > 0 {
                *offset = (*offset).wrapping_sub(1);
            } else {
                self.bufOff = (self.bufOff).wrapping_sub(1);
            };
            self.error = yajl_lex_missing_integer_after_minus;
            return yajl_tok_error;
        }
        if c as libc::c_int == '.' as i32 {
            let mut numRd: libc::c_int = 0 as libc::c_int;
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
            {
                let fresh26 = self.bufOff;
                self.bufOff = (self.bufOff).wrapping_add(1);
                *((*self.buf).data()).add(fresh26) as libc::c_int
            } else {
                let fresh27 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh27) as libc::c_int
            }) as libc::c_uchar;
            while c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                numRd += 1;
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh28 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh28) as libc::c_int
                } else {
                    let fresh29 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh29) as libc::c_int
                }) as libc::c_uchar;
            }
            if numRd == 0 {
                if *offset > 0 {
                    *offset = (*offset).wrapping_sub(1);
                } else {
                    self.bufOff = (self.bufOff).wrapping_sub(1);
                };
                self.error = yajl_lex_missing_integer_after_decimal;
                return yajl_tok_error;
            }
            tok = yajl_tok_double;
        }
        if c as libc::c_int == 'e' as i32 || c as libc::c_int == 'E' as i32 {
            if *offset >= jsonTextLen {
                return yajl_tok_eof;
            }
            c = (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len()
            {
                let fresh30 = self.bufOff;
                self.bufOff = (self.bufOff).wrapping_add(1);
                *((*self.buf).data()).add(fresh30) as libc::c_int
            } else {
                let fresh31 = *offset;
                *offset = (*offset).wrapping_add(1);
                *jsonText.add(fresh31) as libc::c_int
            }) as libc::c_uchar;
            if c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32 {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh32 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh32) as libc::c_int
                } else {
                    let fresh33 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh33) as libc::c_int
                }) as libc::c_uchar;
            }
            if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                loop {
                    if *offset >= jsonTextLen {
                        return yajl_tok_eof;
                    }
                    c = (if self.bufInUse != 0
                        && (*self.buf).len() != 0
                        && self.bufOff < (*self.buf).len()
                    {
                        let fresh34 = self.bufOff;
                        self.bufOff = (self.bufOff).wrapping_add(1);
                        *((*self.buf).data()).add(fresh34) as libc::c_int
                    } else {
                        let fresh35 = *offset;
                        *offset = (*offset).wrapping_add(1);
                        *jsonText.add(fresh35) as libc::c_int
                    }) as libc::c_uchar;
                    if !(c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32) {
                        break;
                    }
                }
            } else {
                if *offset > 0 {
                    *offset = (*offset).wrapping_sub(1);
                } else {
                    self.bufOff = (self.bufOff).wrapping_sub(1);
                };
                self.error = yajl_lex_missing_integer_after_exponent;
                return yajl_tok_error;
            }
            tok = yajl_tok_double;
        }
        if *offset > 0 {
            *offset = (*offset).wrapping_sub(1);
        } else {
            self.bufOff = (self.bufOff).wrapping_sub(1);
        };
        tok
    }
    unsafe fn comment(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: *mut usize,
    ) -> yajl_tok {
        let mut c: libc::c_uchar = 0;
        let mut tok: yajl_tok = yajl_tok_comment;
        if *offset >= jsonTextLen {
            return yajl_tok_eof;
        }
        c = (if self.bufInUse != 0 && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len() {
            let fresh36 = self.bufOff;
            self.bufOff = (self.bufOff).wrapping_add(1);
            *((*self.buf).data()).add(fresh36) as libc::c_int
        } else {
            let fresh37 = *offset;
            *offset = (*offset).wrapping_add(1);
            *jsonText.add(fresh37) as libc::c_int
        }) as libc::c_uchar;
        if c as libc::c_int == '/' as i32 {
            loop {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh38 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh38) as libc::c_int
                } else {
                    let fresh39 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh39) as libc::c_int
                }) as libc::c_uchar;
                if c as libc::c_int == '\n' as i32 {
                    break;
                }
            }
        } else if c as libc::c_int == '*' as i32 {
            loop {
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh40 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh40) as libc::c_int
                } else {
                    let fresh41 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh41) as libc::c_int
                }) as libc::c_uchar;
                if c as libc::c_int != '*' as i32 {
                    continue;
                }
                if *offset >= jsonTextLen {
                    return yajl_tok_eof;
                }
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh42 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh42) as libc::c_int
                } else {
                    let fresh43 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh43) as libc::c_int
                }) as libc::c_uchar;
                if c as libc::c_int == '/' as i32 {
                    break;
                }
                if *offset > 0 {
                    *offset = (*offset).wrapping_sub(1);
                } else {
                    self.bufOff = (self.bufOff).wrapping_sub(1);
                };
            }
        } else {
            self.error = yajl_lex_invalid_char;
            tok = yajl_tok_error;
        }
        tok
    }

    pub unsafe fn lex(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: *mut usize,
        mut outBuf: *mut *const libc::c_uchar,
        mut outLen: *mut usize,
    ) -> yajl_tok {
        let mut tok: yajl_tok = yajl_tok_error;
        let mut c: libc::c_uchar = 0;
        let mut startOffset: usize = *offset;
        *outBuf = std::ptr::null::<libc::c_uchar>();
        *outLen = 0 as libc::c_int as usize;
        's_21: loop {
            if *offset >= jsonTextLen {
                tok = yajl_tok_eof;
                break;
            } else {
                c = (if self.bufInUse != 0
                    && (*self.buf).len() != 0
                    && self.bufOff < (*self.buf).len()
                {
                    let fresh44 = self.bufOff;
                    self.bufOff = (self.bufOff).wrapping_add(1);
                    *((*self.buf).data()).add(fresh44) as libc::c_int
                } else {
                    let fresh45 = *offset;
                    *offset = (*offset).wrapping_add(1);
                    *jsonText.add(fresh45) as libc::c_int
                }) as libc::c_uchar;
                match c as libc::c_int {
                    123 => {
                        tok = yajl_tok_left_bracket;
                        break;
                    }
                    125 => {
                        tok = yajl_tok_right_bracket;
                        break;
                    }
                    91 => {
                        tok = yajl_tok_left_brace;
                        break;
                    }
                    93 => {
                        tok = yajl_tok_right_brace;
                        break;
                    }
                    44 => {
                        tok = yajl_tok_comma;
                        break;
                    }
                    58 => {
                        tok = yajl_tok_colon;
                        break;
                    }
                    9 | 10 | 11 | 12 | 13 | 32 => {
                        startOffset = startOffset.wrapping_add(1);
                    }
                    116 => {
                        let mut want: *const libc::c_char =
                            b"rue\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= jsonTextLen {
                                tok = yajl_tok_eof;
                                break 's_21;
                            } else {
                                c = (if self.bufInUse != 0
                                    && (*self.buf).len() != 0
                                    && self.bufOff < (*self.buf).len()
                                {
                                    let fresh46 = self.bufOff;
                                    self.bufOff = (self.bufOff).wrapping_add(1);
                                    *((*self.buf).data()).add(fresh46) as libc::c_int
                                } else {
                                    let fresh47 = *offset;
                                    *offset = (*offset).wrapping_add(1);
                                    *jsonText.add(fresh47) as libc::c_int
                                }) as libc::c_uchar;
                                if c as libc::c_int != *want as libc::c_int {
                                    if *offset > 0 {
                                        *offset = (*offset).wrapping_sub(1);
                                    } else {
                                        self.bufOff = (self.bufOff).wrapping_sub(1);
                                    };
                                    self.error = yajl_lex_invalid_string;
                                    tok = yajl_tok_error;
                                    break 's_21;
                                } else {
                                    want = want.offset(1);
                                    if *want == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        tok = yajl_tok_bool;
                        break;
                    }
                    102 => {
                        let mut want_0: *const libc::c_char =
                            b"alse\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= jsonTextLen {
                                tok = yajl_tok_eof;
                                break 's_21;
                            } else {
                                c = (if self.bufInUse != 0
                                    && (*self.buf).len() != 0
                                    && self.bufOff < (*self.buf).len()
                                {
                                    let fresh48 = self.bufOff;
                                    self.bufOff = (self.bufOff).wrapping_add(1);
                                    *((*self.buf).data()).add(fresh48) as libc::c_int
                                } else {
                                    let fresh49 = *offset;
                                    *offset = (*offset).wrapping_add(1);
                                    *jsonText.add(fresh49) as libc::c_int
                                }) as libc::c_uchar;
                                if c as libc::c_int != *want_0 as libc::c_int {
                                    if *offset > 0 {
                                        *offset = (*offset).wrapping_sub(1);
                                    } else {
                                        self.bufOff = (self.bufOff).wrapping_sub(1);
                                    };
                                    self.error = yajl_lex_invalid_string;
                                    tok = yajl_tok_error;
                                    break 's_21;
                                } else {
                                    want_0 = want_0.offset(1);
                                    if *want_0 == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        tok = yajl_tok_bool;
                        break;
                    }
                    110 => {
                        let mut want_1: *const libc::c_char =
                            b"ull\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= jsonTextLen {
                                tok = yajl_tok_eof;
                                break 's_21;
                            } else {
                                c = (if self.bufInUse != 0
                                    && (*self.buf).len() != 0
                                    && self.bufOff < (*self.buf).len()
                                {
                                    let fresh50 = self.bufOff;
                                    self.bufOff = (self.bufOff).wrapping_add(1);
                                    *((*self.buf).data()).add(fresh50) as libc::c_int
                                } else {
                                    let fresh51 = *offset;
                                    *offset = (*offset).wrapping_add(1);
                                    *jsonText.add(fresh51) as libc::c_int
                                }) as libc::c_uchar;
                                if c as libc::c_int != *want_1 as libc::c_int {
                                    if *offset > 0 {
                                        *offset = (*offset).wrapping_sub(1);
                                    } else {
                                        self.bufOff = (self.bufOff).wrapping_sub(1);
                                    };
                                    self.error = yajl_lex_invalid_string;
                                    tok = yajl_tok_error;
                                    break 's_21;
                                } else {
                                    want_1 = want_1.offset(1);
                                    if *want_1 == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        tok = yajl_tok_null;
                        break;
                    }
                    34 => {
                        tok = self.string(jsonText, jsonTextLen, offset);
                        break;
                    }
                    45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        if *offset > 0 {
                            *offset = (*offset).wrapping_sub(1);
                        } else {
                            self.bufOff = (self.bufOff).wrapping_sub(1);
                        };
                        tok = self.number(jsonText, jsonTextLen, offset);
                        break;
                    }
                    47 => {
                        if self.allowComments == 0 {
                            if *offset > 0 {
                                *offset = (*offset).wrapping_sub(1);
                            } else {
                                self.bufOff = (self.bufOff).wrapping_sub(1);
                            };
                            self.error = yajl_lex_unallowed_comment;
                            tok = yajl_tok_error;
                            break;
                        } else {
                            tok = self.comment(jsonText, jsonTextLen, offset);
                            if tok as libc::c_uint
                                != yajl_tok_comment as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            tok = yajl_tok_error;
                            (*self.buf).clear();
                            self.bufInUse = 0 as libc::c_int as libc::c_uint;
                            startOffset = *offset;
                        }
                    }
                    _ => {
                        self.error = yajl_lex_invalid_char;
                        tok = yajl_tok_error;
                        break;
                    }
                }
            }
        }
        if tok as libc::c_uint == yajl_tok_eof as libc::c_int as libc::c_uint || self.bufInUse != 0
        {
            if self.bufInUse == 0 {
                (*self.buf).clear();
            }
            self.bufInUse = 1 as libc::c_int as libc::c_uint;
            (*self.buf).append(
                jsonText.add(startOffset) as *const libc::c_void,
                (*offset).wrapping_sub(startOffset),
            );
            self.bufOff = 0 as libc::c_int as usize;
            if tok as libc::c_uint != yajl_tok_eof as libc::c_int as libc::c_uint {
                *outBuf = (*self.buf).data();
                *outLen = (*self.buf).len();
                self.bufInUse = 0 as libc::c_int as libc::c_uint;
            }
        } else if tok as libc::c_uint != yajl_tok_error as libc::c_int as libc::c_uint {
            *outBuf = jsonText.add(startOffset);
            *outLen = (*offset).wrapping_sub(startOffset);
        }
        if tok as libc::c_uint == yajl_tok_string as libc::c_int as libc::c_uint
            || tok as libc::c_uint == yajl_tok_string_with_escapes as libc::c_int as libc::c_uint
        {
            *outBuf = (*outBuf).offset(1);
            *outLen = { *outLen }.wrapping_sub(2 as libc::c_int as usize);
        }
        tok
    }
}
pub unsafe fn yajl_lex_error_to_string(mut error: yajl_lex_error) -> *const libc::c_char {
    match error as libc::c_uint {
        0 => return b"ok, no error\0" as *const u8 as *const libc::c_char,
        1 => {
            return b"invalid bytes in UTF8 string.\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            return b"inside a string, '\\' occurs before a character which it may not.\0"
                as *const u8 as *const libc::c_char;
        }
        3 => {
            return b"invalid character inside string.\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            return b"invalid (non-hex) character occurs after '\\u' inside string.\0" as *const u8
                as *const libc::c_char;
        }
        5 => return b"invalid char in json text.\0" as *const u8 as *const libc::c_char,
        6 => return b"invalid string in json text.\0" as *const u8 as *const libc::c_char,
        8 => {
            return b"malformed number, a digit is required after the exponent.\0" as *const u8
                as *const libc::c_char;
        }
        7 => {
            return b"malformed number, a digit is required after the decimal point.\0" as *const u8
                as *const libc::c_char;
        }
        9 => {
            return b"malformed number, a digit is required after the minus sign.\0" as *const u8
                as *const libc::c_char;
        }
        10 => {
            return b"probable comment found in input text, comments are not enabled.\0" as *const u8
                as *const libc::c_char;
        }
        _ => {}
    }
    b"unknown error code\0" as *const u8 as *const libc::c_char
}
impl Lexer {
    pub fn get_error(&self) -> yajl_lex_error {
        self.error
    }

    pub fn current_line(&self) -> usize {
        self.lineOff
    }

    pub fn current_char(&self) -> usize {
        self.charOff
    }

    pub unsafe fn peek(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: usize,
    ) -> yajl_tok {
        let mut outBuf: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
        let mut outLen: usize = 0;
        let mut bufLen: usize = (*self.buf).len();
        let mut bufOff: usize = self.bufOff;
        let mut bufInUse: libc::c_uint = self.bufInUse;
        let mut tok: yajl_tok = yajl_tok_bool;
        tok = self.lex(jsonText, jsonTextLen, &mut offset, &mut outBuf, &mut outLen);
        self.bufOff = bufOff;
        self.bufInUse = bufInUse;
        (*self.buf).truncate(bufLen);
        tok
    }
}
