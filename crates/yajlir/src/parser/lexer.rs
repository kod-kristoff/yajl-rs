use core::ffi::{c_char, c_void};

use crate::{buffer::Buffer, yajl_alloc::yajl_alloc_funcs};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Token {
    Bool = 0,
    Colon = 1,
    Comma = 2,
    Eof = 3,
    Error = 4,
    LeftBrace = 5,
    LeftBracket = 6,
    Null = 7,
    RightBrace = 8,
    RightBracket = 9,
    Integer = 10,
    Double = 11,
    String = 12,
    StringWithEscapes = 13,
    Comment = 14,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lexer {
    pub lineOff: usize,
    pub charOff: usize,
    pub error: LexError,
    pub buf: *mut Buffer,
    pub bufOff: usize,
    pub bufInUse: libc::c_uint,
    pub allowComments: libc::c_uint,
    pub validateUTF8: libc::c_uint,
    pub alloc: *mut yajl_alloc_funcs,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LexError {
    UnallowedComment = 10,
    MissingIntegerAfterMinus = 9,
    MissingIntegerAfterExponent = 8,
    MissingIntegerAfterDecimal = 7,
    InvalidString = 6,
    InvalidChar = 5,
    StringInvalidHexChar = 4,
    StringInvalidJsonChar = 3,
    StringInvalidEscapedChar = 2,
    StringInvalidUtf8 = 1,
    Ok = 0,
}

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

        (*lxr).lineOff = 0;
        (*lxr).charOff = 0;
        (*lxr).error = LexError::Ok;
        (*lxr).buf = Buffer::alloc(alloc);
        (*lxr).bufOff = 0;
        (*lxr).bufInUse = 0;
        (*lxr).allowComments = allowComments;
        (*lxr).validateUTF8 = validateUTF8;
        (*lxr).alloc = alloc;
        lxr
    }

    pub unsafe fn free(mut lxr: *mut Lexer) {
        Buffer::free((*lxr).buf);
        ((*(*lxr).alloc).free).expect("non-null function pointer")(
            (*(*lxr).alloc).ctx,
            lxr as *mut c_void,
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
    ) -> Token {
        if curChar as libc::c_int <= 0x7f as libc::c_int {
            return Token::String;
        } else if curChar as libc::c_int >> 5 as libc::c_int == 0x6 as libc::c_int {
            if *offset >= jsonTextLen {
                return Token::Eof;
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
                return Token::String;
            }
        } else if curChar as libc::c_int >> 4 as libc::c_int == 0xe as libc::c_int {
            if *offset >= jsonTextLen {
                return Token::Eof;
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
                    return Token::Eof;
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
                    return Token::String;
                }
            }
        } else if curChar as libc::c_int >> 3 as libc::c_int == 0x1e as libc::c_int {
            if *offset >= jsonTextLen {
                return Token::Eof;
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
                    return Token::Eof;
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
                        return Token::Eof;
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
                        return Token::String;
                    }
                }
            }
        }
        Token::Error
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
    ) -> Token {
        let mut tok: Token = Token::Error;
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
                tok = Token::Eof;
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
                    tok = Token::String;
                    break;
                } else if curChar as libc::c_int == '\\' as i32 {
                    hasEscapes = 1 as libc::c_int;
                    if *offset >= jsonTextLen {
                        tok = Token::Eof;
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
                            let mut i: libc::c_uint = 0;
                            i = 0;
                            while i < 4 {
                                if *offset >= jsonTextLen {
                                    tok = Token::Eof;
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
                                        self.error = LexError::StringInvalidHexChar;
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
                            self.error = LexError::StringInvalidEscapedChar;
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
                    self.error = LexError::StringInvalidJsonChar;
                    break;
                } else {
                    if self.validateUTF8 == 0 {
                        continue;
                    }
                    let mut t: Token = self.utf8_char(jsonText, jsonTextLen, offset, curChar);
                    if t == Token::Eof {
                        tok = Token::Eof;
                        break;
                    } else {
                        if t != Token::Error {
                            continue;
                        }
                        self.error = LexError::StringInvalidUtf8;
                        break;
                    }
                }
            }
        }
        if hasEscapes != 0 && tok == Token::String {
            tok = Token::StringWithEscapes;
        }
        tok
    }
    unsafe fn number(
        &mut self,
        mut jsonText: *const libc::c_uchar,
        mut jsonTextLen: usize,
        mut offset: *mut usize,
    ) -> Token {
        let mut c: libc::c_uchar = 0;
        let mut tok: Token = Token::Integer;
        if *offset >= jsonTextLen {
            return Token::Eof;
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
                return Token::Eof;
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
                return Token::Eof;
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
                    return Token::Eof;
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
            self.error = LexError::MissingIntegerAfterMinus;
            return Token::Error;
        }
        if c as libc::c_int == '.' as i32 {
            let mut numRd: libc::c_int = 0 as libc::c_int;
            if *offset >= jsonTextLen {
                return Token::Eof;
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
                    return Token::Eof;
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
                self.error = LexError::MissingIntegerAfterDecimal;
                return Token::Error;
            }
            tok = Token::Double;
        }
        if c as libc::c_int == 'e' as i32 || c as libc::c_int == 'E' as i32 {
            if *offset >= jsonTextLen {
                return Token::Eof;
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
                    return Token::Eof;
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
                        return Token::Eof;
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
                self.error = LexError::MissingIntegerAfterExponent;
                return Token::Error;
            }
            tok = Token::Double;
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
    ) -> Token {
        let mut c: libc::c_uchar = 0;
        let mut tok: Token = Token::Comment;
        if *offset >= jsonTextLen {
            return Token::Eof;
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
                    return Token::Eof;
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
                    return Token::Eof;
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
                    return Token::Eof;
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
            self.error = LexError::InvalidChar;
            tok = Token::Error;
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
    ) -> Token {
        let mut tok: Token = Token::Error;
        let mut c: libc::c_uchar = 0;
        let mut startOffset: usize = *offset;
        *outBuf = std::ptr::null::<libc::c_uchar>();
        *outLen = 0 as libc::c_int as usize;
        's_21: loop {
            if *offset >= jsonTextLen {
                tok = Token::Eof;
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
                        tok = Token::LeftBracket;
                        break;
                    }
                    125 => {
                        tok = Token::RightBracket;
                        break;
                    }
                    91 => {
                        tok = Token::LeftBrace;
                        break;
                    }
                    93 => {
                        tok = Token::RightBrace;
                        break;
                    }
                    44 => {
                        tok = Token::Comma;
                        break;
                    }
                    58 => {
                        tok = Token::Colon;
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
                                tok = Token::Eof;
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
                                    self.error = LexError::InvalidString;
                                    tok = Token::Error;
                                    break 's_21;
                                } else {
                                    want = want.offset(1);
                                    if *want == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        tok = Token::Bool;
                        break;
                    }
                    102 => {
                        let mut want_0: *const libc::c_char =
                            b"alse\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= jsonTextLen {
                                tok = Token::Eof;
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
                                    self.error = LexError::InvalidString;
                                    tok = Token::Error;
                                    break 's_21;
                                } else {
                                    want_0 = want_0.offset(1);
                                    if *want_0 == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        tok = Token::Bool;
                        break;
                    }
                    110 => {
                        let mut want_1: *const libc::c_char =
                            b"ull\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= jsonTextLen {
                                tok = Token::Eof;
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
                                    self.error = LexError::InvalidString;
                                    tok = Token::Error;
                                    break 's_21;
                                } else {
                                    want_1 = want_1.offset(1);
                                    if *want_1 == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        tok = Token::Null;
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
                            self.error = LexError::UnallowedComment;
                            tok = Token::Error;
                            break;
                        } else {
                            tok = self.comment(jsonText, jsonTextLen, offset);
                            if tok != Token::Comment {
                                break;
                            }
                            tok = Token::Error;
                            (*self.buf).clear();
                            self.bufInUse = 0;
                            startOffset = *offset;
                        }
                    }
                    _ => {
                        self.error = LexError::InvalidChar;
                        tok = Token::Error;
                        break;
                    }
                }
            }
        }
        if tok == Token::Eof || self.bufInUse != 0 {
            if self.bufInUse == 0 {
                (*self.buf).clear();
            }
            self.bufInUse = 1;
            (*self.buf).append(
                jsonText.add(startOffset) as *const libc::c_void,
                (*offset).wrapping_sub(startOffset),
            );
            self.bufOff = 0;
            if tok != Token::Eof {
                *outBuf = (*self.buf).data();
                *outLen = (*self.buf).len();
                self.bufInUse = 0;
            }
        } else if tok != Token::Error {
            *outBuf = jsonText.add(startOffset);
            *outLen = (*offset).wrapping_sub(startOffset);
        }
        if tok == Token::String || tok == Token::StringWithEscapes {
            *outBuf = (*outBuf).offset(1);
            *outLen = { *outLen }.wrapping_sub(2 as libc::c_int as usize);
        }
        tok
    }
}
impl LexError {
    pub fn as_c_str_ptr(&self) -> *const c_char {
        match *self {
            Self::Ok => b"ok, no error\0" as *const u8 as *const c_char,
            Self::StringInvalidUtf8 => {
                b"invalid bytes in UTF8 string.\0" as *const u8 as *const c_char
            }
            Self::StringInvalidEscapedChar => {
                b"inside a string, '\\' occurs before a character which it may not.\0" as *const u8
                    as *const c_char
            }
            Self::StringInvalidJsonChar => {
                b"invalid character inside string.\0" as *const u8 as *const c_char
            }
            Self::StringInvalidHexChar => {
                b"invalid (non-hex) character occurs after '\\u' inside string.\0" as *const u8
                    as *const c_char
            }
            Self::InvalidChar => b"invalid char in json text.\0" as *const u8 as *const c_char,
            Self::InvalidString => b"invalid string in json text.\0" as *const u8 as *const c_char,
            Self::MissingIntegerAfterExponent => {
                b"malformed number, a digit is required after the exponent.\0" as *const u8
                    as *const c_char
            }
            Self::MissingIntegerAfterDecimal => {
                b"malformed number, a digit is required after the decimal point.\0" as *const u8
                    as *const c_char
            }
            Self::MissingIntegerAfterMinus => {
                b"malformed number, a digit is required after the minus sign.\0" as *const u8
                    as *const c_char
            }
            Self::UnallowedComment => {
                b"probable comment found in input text, comments are not enabled.\0" as *const u8
                    as *const c_char
            }
        }
    }
}
impl Lexer {
    pub fn get_error(&self) -> LexError {
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
    ) -> Token {
        let mut outBuf: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
        let mut outLen: usize = 0;
        let mut bufLen: usize = (*self.buf).len();
        let mut bufOff: usize = self.bufOff;
        let mut bufInUse: libc::c_uint = self.bufInUse;
        let mut tok: Token = Token::Bool;
        tok = self.lex(jsonText, jsonTextLen, &mut offset, &mut outBuf, &mut outLen);
        self.bufOff = bufOff;
        self.bufInUse = bufInUse;
        (*self.buf).truncate(bufLen);
        tok
    }
}
