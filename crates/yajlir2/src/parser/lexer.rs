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
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Lexer {
    pub lineOff: usize,
    pub charOff: usize,
    pub error: LexError,
    buf: Vec<u8>,
    // pub buf: *mut Buffer,
    pub bufOff: usize,
    pub bufInUse: bool,
    pub allow_comments: bool,
    pub validate_utf8: bool,
    // pub alloc: *mut yajl_alloc_funcs,
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
    pub fn new(allow_comments: bool, validate_utf8: bool) -> Self {
        Self {
            lineOff: 0,
            charOff: 0,
            error: LexError::Ok,
            buf: Vec::new(),
            bufOff: 0,
            bufInUse: false,
            allow_comments,
            validate_utf8,
        }
    }
    // pub unsafe fn alloc(
    //     mut alloc: *mut yajl_alloc_funcs,
    //     mut allow_comments: libc::c_uint,
    //     mut validate_utf8: libc::c_uint,
    // ) -> *mut Lexer {
    //     let mut lxr: *mut Lexer = ((*alloc).malloc).expect("non-null function pointer")(
    //         (*alloc).ctx,
    //         ::core::mem::size_of::<Lexer>(),
    //     ) as *mut Lexer;
    //
    //     (*lxr).lineOff = 0;
    //     (*lxr).charOff = 0;
    //     (*lxr).error = LexError::Ok;
    //     (*lxr).buf = Buffer::alloc(alloc);
    //     (*lxr).bufOff = 0;
    //     (*lxr).bufInUse = 0;
    //     (*lxr).allow_comments = allow_comments;
    //     (*lxr).validate_utf8 = validate_utf8;
    //     (*lxr).alloc = alloc;
    //     lxr
    // }

    // pub unsafe fn free(mut lxr: *mut Lexer) {
    //     Buffer::free((*lxr).buf);
    //     ((*(*lxr).alloc).free).expect("non-null function pointer")(
    //         (*(*lxr).alloc).ctx,
    //         lxr as *mut c_void,
    //     );
    // }
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
    fn read_char(&mut self, text: &[u8], off: &mut usize) -> u8 {
        if self.bufInUse && self.bufOff < self.buf.len() {
            let curr = self.bufOff;
            self.bufOff += 1;
            self.buf[curr]
        } else {
            let curr = *off;
            *off += 1;
            text[curr]
        }
    }
    fn unread_char(&mut self, offset: &mut usize) {
        if *offset > 0 {
            *offset = (*offset).wrapping_sub(1);
        } else {
            self.bufOff = (self.bufOff).wrapping_sub(1);
        };
    }
    unsafe fn utf8_char(
        &mut self,
        text: &[u8],
        // text: &[u8],
        // mut text.len(): usize,
        mut offset: &mut usize,
        mut curChar: u8,
    ) -> Token {
        if curChar as libc::c_int <= 0x7f as libc::c_int {
            return Token::String;
        } else if curChar as libc::c_int >> 5 as libc::c_int == 0x6 as libc::c_int {
            if *offset >= text.len() {
                return Token::Eof;
            }
            curChar = self.read_char(text, offset);
            if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                return Token::String;
            }
        } else if curChar as libc::c_int >> 4 as libc::c_int == 0xe as libc::c_int {
            if *offset >= text.len() {
                return Token::Eof;
            }
            curChar = self.read_char(text, offset);
            if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                curChar = self.read_char(text, offset);
                if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                    return Token::String;
                }
            }
        } else if curChar as libc::c_int >> 3 as libc::c_int == 0x1e as libc::c_int {
            if *offset >= text.len() {
                return Token::Eof;
            }
            curChar = self.read_char(text, offset);
            if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                curChar = self.read_char(text, offset);
                if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                    if *offset >= text.len() {
                        return Token::Eof;
                    }
                    curChar = self.read_char(text, offset);
                    if curChar as libc::c_int >> 6 as libc::c_int == 0x2 as libc::c_int {
                        return Token::String;
                    }
                }
            }
        }
        Token::Error
    }
}
unsafe fn yajl_string_scan(mut buf: &[u8], mut utf8check: bool) -> usize {
    let mut mask = 0x2 | 0x8 | (if utf8check { 0x10 } else { 0 });
    let mut skip: usize = 0;
    while skip < buf.len() && charLookupTable[buf[skip] as usize] & mask == 0 {
        skip = skip.wrapping_add(1);
    }
    skip
}
impl Lexer {
    unsafe fn string(&mut self, text: &[u8], offset: &mut usize) -> Token {
        let mut tok: Token = Token::Error;
        let mut hasEscapes: libc::c_int = 0 as libc::c_int;
        's_10: loop {
            let mut curChar: libc::c_uchar = 0;
            let mut p: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
            let mut len: usize = 0;
            if self.bufInUse && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len() {
                self.bufOff = (self.bufOff)
                    .wrapping_add(yajl_string_scan(&self.buf, self.validate_utf8))
                    as usize;
            } else if *offset < text.len() {
                *offset =
                    (*offset).wrapping_add(yajl_string_scan(text, self.validate_utf8)) as usize;
            }
            if *offset >= text.len() {
                tok = Token::Eof;
                break;
            } else {
                curChar = self.read_char(text, offset);
                if curChar as libc::c_int == '"' as i32 {
                    tok = Token::String;
                    break;
                } else if curChar as libc::c_int == '\\' as i32 {
                    hasEscapes = 1 as libc::c_int;
                    if *offset >= text.len() {
                        tok = Token::Eof;
                        break;
                    } else {
                        curChar = self.read_char(text, offset);
                        if curChar as libc::c_int == 'u' as i32 {
                            let mut i: libc::c_uint = 0;
                            i = 0;
                            while i < 4 {
                                if *offset >= text.len() {
                                    tok = Token::Eof;
                                    break 's_10;
                                } else {
                                    curChar = self.read_char(text, offset);
                                    if charLookupTable[curChar as usize] as libc::c_int
                                        & 0x4 as libc::c_int
                                        == 0
                                    {
                                        self.unread_char(offset);
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
                    if !self.validate_utf8 {
                        continue;
                    }
                    let mut t: Token = self.utf8_char(text, offset, curChar);
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
    unsafe fn number(&mut self, text: &[u8], offset: &mut usize) -> Token {
        let mut c: libc::c_uchar = 0;
        let mut tok: Token = Token::Integer;
        if *offset >= text.len() {
            return Token::Eof;
        }
        c = self.read_char(text, offset);
        if c as libc::c_int == '-' as i32 {
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
        }
        if c as libc::c_int == '0' as i32 {
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
        } else if c as libc::c_int >= '1' as i32 && c as libc::c_int <= '9' as i32 {
            loop {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
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
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
            while c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                numRd += 1;
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
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
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
            if c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32 {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
            }
            if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                loop {
                    if *offset >= text.len() {
                        return Token::Eof;
                    }
                    c = self.read_char(text, offset);
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
    unsafe fn comment(&mut self, text: &[u8], offset: &mut usize) -> Token {
        let mut c: libc::c_uchar = 0;
        let mut tok: Token = Token::Comment;
        if *offset >= text.len() {
            return Token::Eof;
        }
        c = self.read_char(text, offset);
        if c as libc::c_int == '/' as i32 {
            loop {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
                if c as libc::c_int == '\n' as i32 {
                    break;
                }
            }
        } else if c as libc::c_int == '*' as i32 {
            loop {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
                if c as libc::c_int != '*' as i32 {
                    continue;
                }
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
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
        text: &[u8],
        offset: &mut usize,
        mut outBuf: *mut *const libc::c_uchar,
        mut outLen: *mut usize,
    ) -> Token {
        let mut tok: Token = Token::Error;
        let mut c: libc::c_uchar = 0;
        let mut startOffset: usize = *offset;
        *outBuf = std::ptr::null::<libc::c_uchar>();
        *outLen = 0 as libc::c_int as usize;
        's_21: loop {
            if *offset >= text.len() {
                tok = Token::Eof;
                break;
            } else {
                c = self.read_char(text, offset);
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
                            if *offset >= text.len() {
                                tok = Token::Eof;
                                break 's_21;
                            } else {
                                c = self.read_char(text, offset);
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
                            if *offset >= text.len() {
                                tok = Token::Eof;
                                break 's_21;
                            } else {
                                c = self.read_char(text, offset);
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
                            if *offset >= text.len() {
                                tok = Token::Eof;
                                break 's_21;
                            } else {
                                c = self.read_char(text, offset);
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
                        tok = self.string(text, offset);
                        break;
                    }
                    45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        if *offset > 0 {
                            *offset = (*offset).wrapping_sub(1);
                        } else {
                            self.bufOff = (self.bufOff).wrapping_sub(1);
                        };
                        tok = self.number(text, offset);
                        break;
                    }
                    47 => {
                        if !self.allow_comments {
                            if *offset > 0 {
                                *offset = (*offset).wrapping_sub(1);
                            } else {
                                self.bufOff = (self.bufOff).wrapping_sub(1);
                            };
                            self.error = LexError::UnallowedComment;
                            tok = Token::Error;
                            break;
                        } else {
                            tok = self.comment(text, offset);
                            if tok != Token::Comment {
                                break;
                            }
                            tok = Token::Error;
                            self.buf.clear();
                            self.bufInUse = false;
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
        if tok == Token::Eof || self.bufInUse {
            if !self.bufInUse {
                self.buf.clear();
            }
            self.bufInUse = true;
            self.buf.extend_from_slice(&text[startOffset..]);
            self.bufOff = 0;
            if tok != Token::Eof {
                // *outBuf = (*self.buf).data();
                // *outLen = (*self.buf).len();
                self.bufInUse = false;
            }
        } else if tok != Token::Error {
            // *outBuf = text.add(startOffset);
            // *outLen = (*offset).wrapping_sub(startOffset);
        }
        if tok == Token::String || tok == Token::StringWithEscapes {
            // *outBuf = (*outBuf).offset(1);
            // *outLen = { *outLen }.wrapping_sub(2 as libc::c_int as usize);
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

    pub unsafe fn peek(&mut self, text: &[u8], mut offset: usize) -> Token {
        let mut outBuf: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
        let mut outLen: usize = 0;
        let mut bufLen: usize = (*self.buf).len();
        let mut bufOff: usize = self.bufOff;
        let mut bufInUse: libc::c_uint = self.bufInUse;
        let mut tok: Token = Token::Bool;
        tok = self.lex(text, text.len(), &mut offset, &mut outBuf, &mut outLen);
        self.bufOff = bufOff;
        self.bufInUse = bufInUse;
        (*self.buf).truncate(bufLen);
        tok
    }
}
