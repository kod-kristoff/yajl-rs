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
    buf_in_use: bool,
    allowComments: bool,
    validateUTF8: bool,
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
        allowComments: bool,
        validateUTF8: bool,
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
        (*lxr).buf_in_use = false;
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

/* a lookup table which lets us quickly determine three things:
 * VEC - valid escaped control char
 * note.  the solidus '/' may be escaped or not.
 * IJC - invalid json char
 * VHC - valid hex char
 * NFP - needs further processing (from a string scanning perspective)
 * NUC - needs utf8 checking when enabled (from a string scanning perspective)
 */
const VEC: u8 = 0x01;
const IJC: u8 = 0x02;
const VHC: u8 = 0x04;
const NFP: u8 = 0x08;
const NUC: u8 = 0x10;
#[rustfmt::skip]
const charLookupTable: [u8; 256] = [
/*00*/ IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    ,
/*08*/ IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    ,
/*10*/ IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    ,
/*18*/ IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    , IJC    ,

/*20*/ 0      , 0      , NFP|VEC|IJC, 0      , 0      , 0      , 0      , 0      ,
/*28*/ 0      , 0      , 0      , 0      , 0      , 0      , 0      , VEC    ,
/*30*/ VHC    , VHC    , VHC    , VHC    , VHC    , VHC    , VHC    , VHC    ,
/*38*/ VHC    , VHC    , 0      , 0      , 0      , 0      , 0      , 0      ,

/*40*/ 0      , VHC    , VHC    , VHC    , VHC    , VHC    , VHC    , 0      ,
/*48*/ 0      , 0      , 0      , 0      , 0      , 0      , 0      , 0      ,
/*50*/ 0      , 0      , 0      , 0      , 0      , 0      , 0      , 0      ,
/*58*/ 0      , 0      , 0      , 0      , NFP|VEC|IJC, 0      , 0      , 0      ,

/*60*/ 0      , VHC    , VEC|VHC, VHC    , VHC    , VHC    , VEC|VHC, 0      ,
/*68*/ 0      , 0      , 0      , 0      , 0      , 0      , VEC    , 0      ,
/*70*/ 0      , 0      , VEC    , 0      , VEC    , 0      , 0      , 0      ,
/*78*/ 0      , 0      , 0      , 0      , 0      , 0      , 0      , 0      ,

       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,

       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,

       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,

       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    ,
       NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC    , NUC
];

impl Lexer {
    unsafe fn read_char(&mut self, json_text: &[u8], offset: &mut usize) -> u8 {
        if self.buf_in_use && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len() {
            let fresh0 = self.bufOff;
            self.bufOff = (self.bufOff).wrapping_add(1);
            *((*self.buf).data()).add(fresh0)
        } else {
            let fresh1 = *offset;
            *offset = (*offset).wrapping_add(1);
            json_text[fresh1]
        }
    }
    unsafe fn unread_char(&mut self, offset: &mut usize) {
        if *offset > 0 {
            *offset = (*offset).wrapping_sub(1);
        } else {
            self.bufOff = (self.bufOff).wrapping_sub(1);
        };
    }
    unsafe fn utf8_char(&mut self, json_text: &[u8], offset: &mut usize, mut curChar: u8) -> Token {
        if curChar <= 0x7f {
            return Token::String;
        } else if curChar >> 5 == 0x6 {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            curChar = self.read_char(json_text, offset);
            if curChar >> 6 == 0x2 {
                return Token::String;
            }
        } else if curChar >> 4 == 0xe {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            curChar = self.read_char(json_text, offset);
            if curChar >> 6 == 0x2 {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                curChar = self.read_char(json_text, offset);
                if curChar >> 6 == 0x2 {
                    return Token::String;
                }
            }
        } else if curChar >> 3 == 0x1e {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            curChar = self.read_char(json_text, offset);

            if curChar >> 6 == 0x2 {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                curChar = self.read_char(json_text, offset);
                if curChar >> 6 == 0x2 {
                    if *offset >= json_text.len() {
                        return Token::Eof;
                    }
                    curChar = self.read_char(json_text, offset);
                    if curChar >> 6 == 0x2 {
                        return Token::String;
                    }
                }
            }
        }
        Token::Error
    }
}
unsafe fn yajl_string_scan(mut buf: *const libc::c_uchar, len: usize, utf8check: bool) -> usize {
    let mut mask = IJC | NFP | (if utf8check { NUC } else { 0 });
    let mut skip: usize = 0;
    while skip < len && charLookupTable[*buf as usize] & mask == 0 {
        skip = skip.wrapping_add(1);
        buf = buf.offset(1);
    }
    skip
}
impl Lexer {
    unsafe fn string(&mut self, json_text: &[u8], offset: &mut usize) -> Token {
        let mut tok: Token = Token::Error;
        let mut hasEscapes = false;
        's_10: loop {
            let mut curChar: libc::c_uchar = 0;
            let mut p: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
            let mut len: usize = 0;
            if self.buf_in_use && (*self.buf).len() != 0 && self.bufOff < (*self.buf).len() {
                p = ((*self.buf).data()).add(self.bufOff);
                len = ((*self.buf).len()).wrapping_sub(self.bufOff);
                self.bufOff =
                    (self.bufOff).wrapping_add(yajl_string_scan(p, len, self.validateUTF8))
                        as usize;
            } else if *offset < json_text.len() {
                p = json_text.as_ptr().add(*offset);
                len = json_text.len().wrapping_sub(*offset);
                *offset =
                    (*offset).wrapping_add(yajl_string_scan(p, len, self.validateUTF8)) as usize;
            }
            if *offset >= json_text.len() {
                tok = Token::Eof;
                break;
            } else {
                curChar = self.read_char(json_text, offset);
                if curChar == b'"' {
                    tok = Token::String;
                    break;
                } else if curChar == b'\\' {
                    hasEscapes = true;
                    if *offset >= json_text.len() {
                        tok = Token::Eof;
                        break;
                    } else {
                        curChar = self.read_char(json_text, offset);
                        if curChar == b'u' {
                            let mut i: i32 = 0;
                            while i < 4 {
                                if *offset >= json_text.len() {
                                    tok = Token::Eof;
                                    break 's_10;
                                } else {
                                    curChar = self.read_char(json_text, offset);
                                    if charLookupTable[curChar as usize] & VHC == 0 {
                                        self.unread_char(offset);
                                        self.error = LexError::StringInvalidHexChar;
                                        break 's_10;
                                    } else {
                                        i = i.wrapping_add(1);
                                    }
                                }
                            }
                        } else {
                            if charLookupTable[curChar as usize] & VEC != 0 {
                                continue;
                            }
                            self.unread_char(offset);
                            self.error = LexError::StringInvalidEscapedChar;
                            break;
                        }
                    }
                } else if charLookupTable[curChar as usize] & IJC != 0 {
                    self.unread_char(offset);
                    self.error = LexError::StringInvalidJsonChar;
                    break;
                } else {
                    if !self.validateUTF8 {
                        continue;
                    }
                    let mut t: Token = self.utf8_char(json_text, offset, curChar);
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
        if hasEscapes && tok == Token::String {
            tok = Token::StringWithEscapes;
        }
        tok
    }
    unsafe fn number(&mut self, json_text: &[u8], offset: &mut usize) -> Token {
        let mut tok: Token = Token::Integer;
        if *offset >= json_text.len() {
            return Token::Eof;
        }
        let mut c = self.read_char(json_text, offset);
        if c == b'-' {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            c = self.read_char(json_text, offset);
        }
        if c == b'0' {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            c = self.read_char(json_text, offset);
        } else if (b'1'..=b'9').contains(&c) {
            loop {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.read_char(json_text, offset);
                if !c.is_ascii_digit() {
                    break;
                }
            }
        } else {
            self.unread_char(offset);
            self.error = LexError::MissingIntegerAfterMinus;
            return Token::Error;
        }
        if c == b'.' {
            let mut numRd = 0;
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            c = self.read_char(json_text, offset);
            while c.is_ascii_digit() {
                numRd += 1;
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.read_char(json_text, offset);
            }
            if numRd == 0 {
                self.unread_char(offset);
                self.error = LexError::MissingIntegerAfterDecimal;
                return Token::Error;
            }
            tok = Token::Double;
        }
        if c == b'e' || c == b'E' {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            c = self.read_char(json_text, offset);
            if c == b'+' || c == b'-' {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.read_char(json_text, offset);
            }
            if c.is_ascii_digit() {
                loop {
                    if *offset >= json_text.len() {
                        return Token::Eof;
                    }
                    c = self.read_char(json_text, offset);
                    if !c.is_ascii_digit() {
                        break;
                    }
                }
            } else {
                self.unread_char(offset);
                self.error = LexError::MissingIntegerAfterExponent;
                return Token::Error;
            }
            tok = Token::Double;
        }
        self.unread_char(offset);
        tok
    }
    unsafe fn comment(&mut self, json_text: &[u8], offset: &mut usize) -> Token {
        let mut tok: Token = Token::Comment;
        if *offset >= json_text.len() {
            return Token::Eof;
        }
        let mut c = self.read_char(json_text, offset);
        if c == b'/' {
            loop {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.read_char(json_text, offset);
                if c == b'\n' {
                    break;
                }
            }
        } else if c == b'*' {
            loop {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.read_char(json_text, offset);
                if c != b'*' {
                    continue;
                }
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.read_char(json_text, offset);
                if c == b'/' {
                    break;
                }
                self.unread_char(offset);
            }
        } else {
            self.error = LexError::InvalidChar;
            tok = Token::Error;
        }
        tok
    }

    pub unsafe fn lex(
        &mut self,
        json_text: &[u8],
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
            if *offset >= json_text.len() {
                tok = Token::Eof;
                break;
            } else {
                c = self.read_char(json_text, offset);
                match c {
                    b'{' => {
                        tok = Token::LeftBracket;
                        break;
                    }
                    b'}' => {
                        tok = Token::RightBracket;
                        break;
                    }
                    b'[' => {
                        tok = Token::LeftBrace;
                        break;
                    }
                    b']' => {
                        tok = Token::RightBrace;
                        break;
                    }
                    b',' => {
                        tok = Token::Comma;
                        break;
                    }
                    b':' => {
                        tok = Token::Colon;
                        break;
                    }
                    9 | 10 | 11 | 12 | 13 | 32 => {
                        startOffset = startOffset.wrapping_add(1);
                    }
                    b't' => {
                        let mut want: *const libc::c_char =
                            b"rue\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= json_text.len() {
                                tok = Token::Eof;
                                break 's_21;
                            } else {
                                c = self.read_char(json_text, offset);
                                if c as libc::c_int != *want as libc::c_int {
                                    self.unread_char(offset);
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
                    b'f' => {
                        let mut want_0: *const libc::c_char =
                            b"alse\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= json_text.len() {
                                tok = Token::Eof;
                                break 's_21;
                            } else {
                                c = self.read_char(json_text, offset);
                                if c as libc::c_int != *want_0 as libc::c_int {
                                    self.unread_char(offset);
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
                    b'n' => {
                        let mut want_1: *const libc::c_char =
                            b"ull\0" as *const u8 as *const libc::c_char;
                        loop {
                            if *offset >= json_text.len() {
                                tok = Token::Eof;
                                break 's_21;
                            } else {
                                c = self.read_char(json_text, offset);
                                if c as libc::c_int != *want_1 as libc::c_int {
                                    self.unread_char(offset);
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
                    b'"' => {
                        tok = self.string(json_text, offset);
                        break;
                    }
                    b'-' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                        self.unread_char(offset);
                        tok = self.number(json_text, offset);
                        break;
                    }
                    b'/' => {
                        if !self.allowComments {
                            self.unread_char(offset);
                            self.error = LexError::UnallowedComment;
                            tok = Token::Error;
                            break;
                        } else {
                            tok = self.comment(json_text, offset);
                            if tok != Token::Comment {
                                break;
                            }
                            tok = Token::Error;
                            (*self.buf).clear();
                            self.buf_in_use = false;
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
        if tok == Token::Eof || self.buf_in_use {
            if !self.buf_in_use {
                (*self.buf).clear();
            }
            self.buf_in_use = true;
            (*self.buf).append(
                json_text.as_ptr().add(startOffset) as *const libc::c_void,
                (*offset).wrapping_sub(startOffset),
            );
            self.bufOff = 0;
            if tok != Token::Eof {
                *outBuf = (*self.buf).data();
                *outLen = (*self.buf).len();
                self.buf_in_use = false;
            }
        } else if tok != Token::Error {
            *outBuf = json_text.as_ptr().add(startOffset);
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

    pub unsafe fn peek(&mut self, json_text: &[u8], mut offset: usize) -> Token {
        let mut outBuf: *const libc::c_uchar = std::ptr::null::<libc::c_uchar>();
        let mut outLen: usize = 0;
        let mut bufLen: usize = (*self.buf).len();
        let mut bufOff: usize = self.bufOff;
        let buf_in_use = self.buf_in_use;
        let mut tok: Token = Token::Bool;
        tok = self.lex(json_text, &mut offset, &mut outBuf, &mut outLen);
        self.bufOff = bufOff;
        self.buf_in_use = buf_in_use;
        (*self.buf).truncate(bufLen);
        tok
    }
}
