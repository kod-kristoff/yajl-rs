#![allow(clippy::missing_safety_doc)]
use core::ffi::{c_char, c_void, CStr};
use core::ptr;

use crate::{yajl_alloc::yajl_alloc_funcs, yajl_encode::yajl_string_decode, ParserOption, Status};

use super::{lexer::Token, Lexer, ParseError, Parser};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseState {
    Start = 0,
    GotValue = 12,
    ArrayNeedVal = 11,
    ArrayGotVal = 10,
    ArrayStart = 9,
    MapNeedKey = 8,
    MapGotVal = 7,
    MapNeedVal = 6,
    MapSep = 5,
    MapStart = 4,
    LexicalError = 3,
    ParseError = 2,
    ParseComplete = 1,
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ByteStack {
    stack: *mut ParseState,
    cap: usize,
    len: usize,
    yaf: *mut yajl_alloc_funcs,
}

#[derive(Debug, Clone)]
pub enum ByteStackError {
    OutOfMemory,
    NoAllocFuncs,
}
impl ByteStack {
    pub fn new(yaf: *mut yajl_alloc_funcs) -> ByteStack {
        assert!(!yaf.is_null());
        ByteStack {
            stack: ptr::null_mut(),
            cap: 0,
            len: 0,
            yaf,
        }
    }

    pub fn free(&mut self) {
        if !self.stack.is_null() {
            unsafe {
                ((*self.yaf).free).expect("non-null function poiner")(
                    (*self.yaf).ctx,
                    self.stack as *mut c_void,
                );
            }
            self.stack = ptr::null_mut();
        }
    }

    pub fn push(&mut self, state: ParseState) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            *self.stack.add(self.len) = state;
        }
        self.len += 1;
    }
    pub fn pop(&mut self) {
        self.len -= 1;
    }

    pub fn top(&self) -> ParseState {
        debug_assert!(self.len > 0);
        unsafe { *self.stack.add(self.len - 1) }
    }
    pub fn top_mut(&mut self) -> &mut ParseState {
        debug_assert!(self.len > 0);
        unsafe { &mut *self.stack.add(self.len - 1) }
    }

    fn grow(&mut self) {
        self.cap = self.cap.wrapping_add(128);
        unsafe {
            self.stack = ((*self.yaf).realloc).expect("non-null function poiner")(
                (*self.yaf).ctx,
                self.stack as *mut c_void,
                self.cap,
            ) as *mut ParseState;
        }
        debug_assert!(!self.stack.is_null());
    }
}
pub type yajl_lexer = *mut Lexer;

pub type yajl_handle = *mut Parser;

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

const MAX_VALUE_TO_MULTIPLY: i64 = i64::MAX / 10 + i64::MAX % 10;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseIntegerError {
    /// Too large integer detected
    Overflow,
    /// Too small integer detected
    Underflow,
    /// Non-numerical char detected
    NonNumerical(u8),
}
pub unsafe fn parse_integer(
    mut number: *const u8,
    length: usize,
) -> Result<i64, ParseIntegerError> {
    let mut ret: i64 = 0;
    let mut sign: i8 = 1;
    let mut pos: *const u8 = number;
    if *pos as i32 == '-' as i32 {
        pos = pos.offset(1);
        sign = -1;
    }
    if *pos as i32 == '+' as i32 {
        pos = pos.offset(1);
    }
    while pos < number.add(length) {
        if ret > MAX_VALUE_TO_MULTIPLY {
            return if sign == 1 {
                Err(ParseIntegerError::Overflow)
            } else {
                Err(ParseIntegerError::Underflow)
            };
        }
        ret *= 10;
        if i64::MAX - ret < (*pos as i32 - '0' as i32) as i64 {
            return if sign == 1 {
                Err(ParseIntegerError::Overflow)
            } else {
                Err(ParseIntegerError::Underflow)
            };
        }
        if *pos < b'0' || *pos > b'9' {
            return Err(ParseIntegerError::NonNumerical(*pos));
        }
        let fresh0 = pos;
        pos = pos.offset(1);
        ret += (*fresh0 as libc::c_int - '0' as i32) as libc::c_longlong;
    }
    Ok(sign as i64 * ret)
}
impl Parser {
    pub unsafe fn render_error_string(
        &self,
        json_text: &[u8],

        mut verbose: bool,
    ) -> *mut libc::c_uchar {
        let mut offset: usize = self.bytesConsumed;
        let mut str: *mut libc::c_uchar = std::ptr::null_mut::<libc::c_uchar>();
        let mut errorType: *const c_char = ptr::null();
        let mut errorText: *const c_char = ptr::null();
        let mut text: [libc::c_char; 72] = [0; 72];
        let mut arrow: *const libc::c_char =
            b"                     (right here) ------^\n\0" as *const u8 as *const libc::c_char;
        match self.stateStack.top() {
            ParseState::ParseError => {
                errorType = b"parse\0" as *const u8 as *const libc::c_char;
                errorText = self.parseError.unwrap().as_c_str_ptr();
            }
            ParseState::LexicalError => {
                errorType = b"lexical\0" as *const u8 as *const libc::c_char;
                errorText = (*self.lexer).get_error().as_c_str_ptr();
            }
            _ => {
                errorType = b"unknown\0" as *const u8 as *const libc::c_char;
            }
        }
        let mut memneeded: usize = 0;
        memneeded = (memneeded).wrapping_add(libc::strlen(errorType));
        memneeded = (memneeded).wrapping_add(libc::strlen(
            b" error\0" as *const u8 as *const libc::c_char,
        ));
        if !errorText.is_null() {
            memneeded =
                memneeded.wrapping_add(libc::strlen(b": \0" as *const u8 as *const libc::c_char));
            memneeded = memneeded.wrapping_add(libc::strlen(errorText));
        }
        str = (self.alloc.malloc).expect("non-null function pointer")(
            self.alloc.ctx,
            memneeded.wrapping_add(2 as libc::c_int as usize),
        ) as *mut libc::c_uchar;
        if str.is_null() {
            return ptr::null_mut::<libc::c_uchar>();
        }
        *str.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
        libc::strcat(str as *mut libc::c_char, errorType);
        libc::strcat(
            str as *mut libc::c_char,
            b" error\0" as *const u8 as *const libc::c_char,
        );
        if !errorText.is_null() {
            libc::strcat(
                str as *mut libc::c_char,
                b": \0" as *const u8 as *const libc::c_char,
            );
            libc::strcat(str as *mut libc::c_char, errorText);
        }
        libc::strcat(
            str as *mut libc::c_char,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
        if verbose {
            let mut start: usize = 0;
            let mut end: usize = 0;
            let mut i: usize = 0;
            let mut spacesNeeded: usize = 0;
            spacesNeeded = if offset < 30 as libc::c_int as usize {
                (40 as libc::c_int as usize).wrapping_sub(offset)
            } else {
                10 as libc::c_int as usize
            };
            start = if offset >= 30 as libc::c_int as usize {
                offset.wrapping_sub(30 as libc::c_int as usize)
            } else {
                0 as libc::c_int as usize
            };
            end = if offset.wrapping_add(30 as libc::c_int as usize) > json_text.len() {
                json_text.len()
            } else {
                offset.wrapping_add(30 as libc::c_int as usize)
            };
            i = 0 as libc::c_int as usize;
            while i < spacesNeeded {
                text[i] = ' ' as i32 as libc::c_char;
                i = i.wrapping_add(1);
            }
            while start < end {
                if json_text[start] != b'\n' && json_text[start] != b'\r' {
                    text[i] = json_text[start];
                } else {
                    text[i] = b' ';
                }
                start = start.wrapping_add(1);
                i = i.wrapping_add(1);
            }
            let fresh1 = i;
            i = i.wrapping_add(1);
            text[fresh1] = '\n' as i32 as libc::c_char;
            text[i] = 0 as libc::c_int as libc::c_char;
            let mut newStr: *mut libc::c_char = (self.alloc.malloc)
                .expect("non-null function pointer")(
                self.alloc.ctx,
                (libc::strlen(str as *mut libc::c_char))
                    .wrapping_add(libc::strlen(text.as_mut_ptr()))
                    .wrapping_add(libc::strlen(arrow))
                    .wrapping_add(1),
            ) as *mut libc::c_char;
            if !newStr.is_null() {
                *newStr.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
                libc::strcat(newStr, str as *mut libc::c_char);
                libc::strcat(newStr, text.as_mut_ptr());
                libc::strcat(newStr, arrow);
            }
            (self.alloc.free).expect("non-null function pointer")(
                self.alloc.ctx,
                str as *mut libc::c_void,
            );
            str = newStr as *mut libc::c_uchar;
        }
        str
    }
}
impl Parser {
    pub unsafe fn do_finish(&mut self) -> Status {
        let stat = self.do_parse(b" ");
        if stat != Status::Ok {
            return stat;
        }
        match self.stateStack.top() {
            ParseState::ParseError | ParseState::LexicalError => Status::Error,
            ParseState::GotValue | ParseState::ParseComplete => Status::Ok,
            _ => {
                if self.flags & ParserOption::AllowPartialValues as u32 == 0 {
                    *self.stateStack.top_mut() = ParseState::ParseError;
                    self.parseError = Some(ParseError::PrematureEof);
                    return Status::Error;
                }
                Status::Ok
            }
        }
    }

    pub unsafe fn do_parse(&mut self, json_text: &[u8]) -> Status {
        let mut current_block: u64;
        let mut tok: Token = Token::Bool;
        let mut buf: *const libc::c_uchar = ptr::null::<libc::c_uchar>();
        let mut bufLen: usize = 0;
        let offset = &mut self.bytesConsumed;
        *offset = 0;
        loop {
            match self.stateStack.top() {
                ParseState::ParseComplete => {
                    if self.flags & ParserOption::AllowMultipleValues as u32 != 0 {
                        *self.stateStack.top_mut() = ParseState::GotValue;
                    } else {
                        if self.flags & ParserOption::AllowTrailingGarbage as libc::c_uint != 0 {
                            break;
                        }
                        if *offset == json_text.len() {
                            break;
                        }
                        tok = (*self.lexer).lex(json_text, offset, &mut buf, &mut bufLen);
                        if tok != Token::Eof {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::TrailingGarbage);
                        }
                    }
                }
                ParseState::ParseError | ParseState::LexicalError => return Status::Error,
                ParseState::Start
                | ParseState::GotValue
                | ParseState::MapNeedVal
                | ParseState::ArrayNeedVal
                | ParseState::ArrayStart => {
                    let mut stateToPush = ParseState::Start;
                    tok = (*self.lexer).lex(json_text, offset, &mut buf, &mut bufLen);
                    match tok {
                        Token::Eof => return Status::Ok,
                        Token::Error => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                            continue;
                        }
                        Token::String => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_string).is_some()
                                && ((*self.callbacks).yajl_string)
                                    .expect("non-null function pointer")(
                                    self.ctx, buf, bufLen
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            current_block = 6407515180622463684;
                        }
                        Token::StringWithEscapes => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_string).is_some()
                            {
                                (*self.decodeBuf).clear();
                                yajl_string_decode(self.decodeBuf, buf, bufLen);
                                if ((*self.callbacks).yajl_string)
                                    .expect("non-null function pointer")(
                                    self.ctx,
                                    (*self.decodeBuf).data(),
                                    (*self.decodeBuf).len(),
                                ) == 0
                                {
                                    *self.stateStack.top_mut() = ParseState::ParseError;
                                    self.parseError = Some(ParseError::ClientCancelled);
                                    return Status::ClientCanceled;
                                }
                            }
                            current_block = 6407515180622463684;
                        }
                        Token::Bool => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_boolean).is_some()
                                && ((*self.callbacks).yajl_boolean)
                                    .expect("non-null function pointer")(
                                    self.ctx,
                                    (*buf as libc::c_int == 't' as i32) as libc::c_int,
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            current_block = 6407515180622463684;
                        }
                        Token::Null => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_null).is_some()
                                && ((*self.callbacks).yajl_null).expect("non-null function pointer")(
                                    self.ctx,
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            current_block = 6407515180622463684;
                        }
                        Token::LeftBracket => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_start_map).is_some()
                                && ((*self.callbacks).yajl_start_map)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            stateToPush = ParseState::MapStart;
                            current_block = 6407515180622463684;
                        }
                        Token::LeftBrace => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_start_array).is_some()
                                && ((*self.callbacks).yajl_start_array)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            stateToPush = ParseState::ArrayStart;
                            current_block = 6407515180622463684;
                        }
                        Token::Integer => {
                            if !(self.callbacks).is_null() {
                                if ((*self.callbacks).yajl_number).is_some() {
                                    if ((*self.callbacks).yajl_number)
                                        .expect("non-null function pointer")(
                                        self.ctx,
                                        buf as *const libc::c_char,
                                        bufLen,
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = Some(ParseError::ClientCancelled);
                                        return Status::ClientCanceled;
                                    }
                                } else if ((*self.callbacks).yajl_integer).is_some() {
                                    let Ok(i) = parse_integer(buf, bufLen) else {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = Some(ParseError::IntegerOverflow);
                                        if *offset >= bufLen {
                                            *offset = { *offset }.wrapping_sub(bufLen);
                                        } else {
                                            *offset = 0;
                                        }
                                        continue;
                                    };
                                    if ((*self.callbacks).yajl_integer)
                                        .expect("non-null function pointer")(
                                        self.ctx, i
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = Some(ParseError::ClientCancelled);
                                        return Status::ClientCanceled;
                                    }
                                }
                                current_block = 6407515180622463684;
                            } else {
                                current_block = 6407515180622463684;
                            }
                        }
                        Token::Double => {
                            if !(self.callbacks).is_null() {
                                if ((*self.callbacks).yajl_number).is_some() {
                                    if ((*self.callbacks).yajl_number)
                                        .expect("non-null function pointer")(
                                        self.ctx,
                                        buf as *const libc::c_char,
                                        bufLen,
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = Some(ParseError::ClientCancelled);
                                        return Status::ClientCanceled;
                                    }
                                } else if ((*self.callbacks).yajl_double).is_some() {
                                    let mut d: f64 = 0.0f64;
                                    (*self.decodeBuf).clear();
                                    (*self.decodeBuf).append(buf as *const c_void, bufLen);
                                    buf = (*self.decodeBuf).data();
                                    let (d, d_is_error) = if let Ok(s) =
                                        CStr::from_ptr(buf as *const libc::c_char).to_str()
                                    {
                                        if let Some((d, d_len)) = strtod::strtod(s) {
                                            (d, false)
                                        } else {
                                            (0f64, true)
                                        }
                                    } else {
                                        (0f64, true)
                                    };

                                    if d_is_error {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = Some(ParseError::FloatingPointOverflow);
                                        if *offset >= bufLen {
                                            *offset = { *offset }.wrapping_sub(bufLen);
                                        } else {
                                            *offset = 0 as libc::c_int as usize;
                                        }
                                        continue;
                                    } else if ((*self.callbacks).yajl_double)
                                        .expect("non-null function pointer")(
                                        self.ctx, d
                                    ) == 0
                                    {
                                        *self.stateStack.top_mut() = ParseState::ParseError;
                                        self.parseError = Some(ParseError::ClientCancelled);
                                        return Status::ClientCanceled;
                                    }
                                }
                                current_block = 6407515180622463684;
                            } else {
                                current_block = 6407515180622463684;
                            }
                        }
                        Token::RightBrace => {
                            if self.stateStack.top() == ParseState::ArrayStart {
                                if !(self.callbacks).is_null()
                                    && ((*self.callbacks).yajl_end_array).is_some()
                                    && ((*self.callbacks).yajl_end_array)
                                        .expect("non-null function pointer")(
                                        self.ctx
                                    ) == 0
                                {
                                    *self.stateStack.top_mut() = ParseState::ParseError;
                                    self.parseError = Some(ParseError::ClientCancelled);
                                    return Status::ClientCanceled;
                                }
                                self.stateStack.pop();
                                continue;
                            } else {
                                current_block = 13495271385072242379;
                            }
                        }
                        Token::Colon | Token::Comma | Token::RightBracket => {
                            current_block = 13495271385072242379;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::InvalidToken);
                            continue;
                        }
                    }
                    match current_block {
                        6407515180622463684 => {
                            let mut s = self.stateStack.top();
                            if s == ParseState::Start || s == ParseState::GotValue {
                                *self.stateStack.top_mut() = ParseState::ParseComplete;
                            } else if s == ParseState::MapNeedVal {
                                *self.stateStack.top_mut() = ParseState::MapGotVal;
                            } else {
                                *self.stateStack.top_mut() = ParseState::ArrayGotVal;
                            }
                            if stateToPush != ParseState::Start {
                                self.stateStack.push(stateToPush);
                            }
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::UnallowedToken);
                        }
                    }
                }
                ParseState::MapStart | ParseState::MapNeedKey => {
                    tok = (*self.lexer).lex(json_text, offset, &mut buf, &mut bufLen);
                    match tok {
                        Token::Eof => return Status::Ok,
                        Token::Error => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                            continue;
                        }
                        Token::StringWithEscapes => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_map_key).is_some()
                            {
                                (*self.decodeBuf).clear();
                                yajl_string_decode(self.decodeBuf, buf, bufLen);
                                buf = (*self.decodeBuf).data();
                                bufLen = (*self.decodeBuf).len();
                            }
                            current_block = 5544887021832600539;
                        }
                        Token::String => {
                            current_block = 5544887021832600539;
                        }
                        Token::RightBracket => {
                            if self.stateStack.top() == ParseState::MapStart {
                                if !(self.callbacks).is_null()
                                    && ((*self.callbacks).yajl_end_map).is_some()
                                    && ((*self.callbacks).yajl_end_map)
                                        .expect("non-null function pointer")(
                                        self.ctx
                                    ) == 0
                                {
                                    *self.stateStack.top_mut() = ParseState::ParseError;
                                    self.parseError = Some(ParseError::ClientCancelled);
                                    return Status::ClientCanceled;
                                }
                                self.stateStack.pop();
                                continue;
                            } else {
                                current_block = 17513148302838498461;
                            }
                        }
                        _ => {
                            current_block = 17513148302838498461;
                        }
                    }
                    match current_block {
                        5544887021832600539 => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_map_key).is_some()
                                && ((*self.callbacks).yajl_map_key)
                                    .expect("non-null function pointer")(
                                    self.ctx, buf, bufLen
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            *self.stateStack.top_mut() = ParseState::MapSep;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::InvalidObjectKey);
                        }
                    }
                }
                ParseState::MapSep => {
                    tok = (*self.lexer).lex(json_text, offset, &mut buf, &mut bufLen);
                    match tok {
                        Token::Colon => {
                            *self.stateStack.top_mut() = ParseState::MapNeedVal;
                        }
                        Token::Eof => return Status::Ok,
                        Token::Error => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::InvalidKeyValueSeparator);
                        }
                    }
                }
                ParseState::MapGotVal => {
                    tok = (*self.lexer).lex(json_text, offset, &mut buf, &mut bufLen);
                    match tok {
                        Token::RightBracket => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_end_map).is_some()
                                && ((*self.callbacks).yajl_end_map)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            self.stateStack.pop();
                        }
                        Token::Comma => {
                            *self.stateStack.top_mut() = ParseState::MapNeedKey;
                        }
                        Token::Eof => return Status::Ok,
                        Token::Error => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::InvalidObjectSeparator);
                            if *offset >= bufLen {
                                *offset = { *offset }.wrapping_sub(bufLen);
                            } else {
                                *offset = 0 as libc::c_int as usize;
                            }
                        }
                    }
                }
                ParseState::ArrayGotVal => {
                    tok = (*self.lexer).lex(json_text, offset, &mut buf, &mut bufLen);
                    match tok {
                        Token::RightBrace => {
                            if !(self.callbacks).is_null()
                                && ((*self.callbacks).yajl_end_array).is_some()
                                && ((*self.callbacks).yajl_end_array)
                                    .expect("non-null function pointer")(
                                    self.ctx
                                ) == 0
                            {
                                *self.stateStack.top_mut() = ParseState::ParseError;
                                self.parseError = Some(ParseError::ClientCancelled);
                                return Status::ClientCanceled;
                            }
                            self.stateStack.pop();
                        }
                        Token::Comma => {
                            *self.stateStack.top_mut() = ParseState::ArrayNeedVal;
                        }
                        Token::Eof => return Status::Ok,
                        Token::Error => {
                            *self.stateStack.top_mut() = ParseState::LexicalError;
                        }
                        _ => {
                            *self.stateStack.top_mut() = ParseState::ParseError;
                            self.parseError = Some(ParseError::InvalidArraySeparator);
                        }
                    }
                }
            }
        }
        Status::Ok
    }
}
