use crate::parser::{
    lexer::{LexError, Token},
    ParserOptions,
};

pub struct RLexer {
    buf: Vec<u8>,
    buf_in_use: bool,
    buf_off: usize,
    error: Option<LexError>,
    allow_comments: bool,
    validate_utf8: bool,
}

impl RLexer {
    pub fn new(allow_comments: bool, validate_utf8: bool) -> Self {
        Self {
            buf: Vec::new(),
            buf_in_use: false,
            buf_off: 0,
            error: None,
            allow_comments,
            validate_utf8,
        }
    }

    fn comment(&mut self, json_text: &[u8], offset: &mut usize) -> Token {
        if *offset >= json_text.len() {
            return Token::Eof;
        }
        let mut tok = Token::Comment;
        let mut c = self.next_char(json_text, offset);
        if c == b'/' {
            loop {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.next_char(json_text, offset);
                if c == b'\n' {
                    break;
                }
            }
        } else if c == b'*' {
            loop {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.next_char(json_text, offset);
                if c != b'*' {
                    continue;
                }
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                c = self.next_char(json_text, offset);
                if c == b'/' {
                    break;
                }
                if *offset > 0 {
                    *offset -= 1;
                } else {
                    self.buf_off -= 1;
                };
            }
        } else {
            self.error = Some(LexError::InvalidChar);
            tok = Token::Error;
        }

        tok
    }

    #[inline]
    fn next_char(&mut self, json_text: &[u8], offset: &mut usize) -> u8 {
        if self.buf_in_use && self.buf_off < self.buf.len() {
            let curr = self.buf_off;
            self.buf_off += 1;
            self.buf[curr]
        } else {
            let curr = *offset;
            *offset += 1;
            json_text[curr]
        }
    }

    fn string(&mut self, json_text: &[u8], offset: &mut usize) -> Token {
        let mut tok = Token::Error;
        let mut has_escapes = false;
        's10: loop {
            let mut curr_char = 0;
            let mut len = 0;
            if self.buf_in_use && self.buf_off < self.buf.len() {
                len = self.buf.len() - self.buf_off;
                self.buf_off += string_scan(&self.buf[self.buf_off..], self.validate_utf8);
            } else {
                *offset += string_scan(&json_text[*offset..], self.validate_utf8);
            }
            if *offset >= json_text.len() {
                tok = Token::Eof;
                break;
            } else {
                curr_char = self.next_char(json_text, offset);
                if curr_char == b'"' {
                    tok = Token::String;
                    break;
                }
                if curr_char == b'\\' {
                    has_escapes = true;
                    if *offset >= json_text.len() {
                        tok = Token::Eof;
                        break;
                    }
                    curr_char = self.next_char(json_text, offset);
                    if curr_char == b'u' {
                        for i in 0..4 {
                            if *offset >= json_text.len() {
                                tok = Token::Eof;
                                break 's10;
                            }
                            curr_char = self.next_char(json_text, offset);
                            if CHAR_LOOKUP_TABLE[curr_char as usize] & 0x4 == 0 {
                                if *offset > 0 {
                                    *offset -= 1;
                                } else {
                                    self.buf_off -= 1;
                                }
                                self.error = Some(LexError::StringInvalidHexChar);
                                break 's10;
                            }
                        }
                    } else {
                        if CHAR_LOOKUP_TABLE[curr_char as usize] & 0x1 != 0 {
                            continue;
                        }
                        if *offset > 0 {
                            *offset -= 1;
                        } else {
                            self.buf_off -= 1;
                        }
                        self.error = Some(LexError::StringInvalidEscapedChar);
                        break;
                    }
                } else if CHAR_LOOKUP_TABLE[curr_char as usize] & 0x2 != 0 {
                    if *offset > 0 {
                        *offset -= 1;
                    } else {
                        self.buf_off -= 1;
                    }
                    self.error = Some(LexError::StringInvalidJsonChar);
                    break;
                } else {
                    if !self.validate_utf8 {
                        continue;
                    }
                    let mut t = self.utf8_char(json_text, offset, curr_char);
                    if t == Token::Eof {
                        tok = Token::Eof;
                        break;
                    } else {
                        if t != Token::Error {
                            continue;
                        }
                        self.error = Some(LexError::StringInvalidUtf8);
                        break;
                    }
                }
            }
        }
        if has_escapes && tok == Token::String {
            tok = Token::StringWithEscapes;
        }
        tok
    }

    fn utf8_char(&mut self, json_text: &[u8], offset: &mut usize, mut curr_char: u8) -> Token {
        if curr_char <= 0x7f {
            return Token::String;
        }
        if curr_char >> 5 == 0x6 {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            curr_char = self.next_char(json_text, offset);
            if curr_char >> 6 == 0x2 {
                return Token::String;
            }
        } else if curr_char >> 4 == 0xe {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            curr_char = self.next_char(json_text, offset);
            if curr_char >> 6 == 0x2 {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }
                curr_char = self.next_char(json_text, offset);
                if curr_char >> 6 == 0x2 {
                    return Token::String;
                }
            }
        } else if curr_char >> 3 == 0x1e {
            if *offset >= json_text.len() {
                return Token::Eof;
            }
            curr_char = self.next_char(json_text, offset);
            if curr_char >> 6 == 0x2 {
                if *offset >= json_text.len() {
                    return Token::Eof;
                }

                curr_char = self.next_char(json_text, offset);
                if curr_char >> 6 == 0x2 {
                    if *offset >= json_text.len() {
                        return Token::Eof;
                    }

                    curr_char = self.next_char(json_text, offset);
                    if curr_char >> 6 == 0x2 {
                        return Token::String;
                    }
                }
            }
        }
        Token::Error
    }
}

fn string_scan(buf: &[u8], utf8check: bool) -> usize {
    let mut i = 0;
    let mut mask = 0x2 | 0x8 | (if utf8check { 0x10 } else { 0 });
    let mut skip = 0;
    while skip < buf.len() && CHAR_LOOKUP_TABLE[i] & mask == 0 {
        skip += 1;
        i += 1;
    }
    skip
}

impl RLexer {
    pub fn lex(&mut self, json_text: &[u8], offset: &mut usize) -> Token {
        let mut tok = Token::Error;
        let mut start_offset = *offset;
        let mut c = 0;
        loop {
            if *offset >= json_text.len() {
                tok = Token::Eof;
                break;
            } else {
                c = self.next_char(json_text, offset);
                match c {
                    b'{' => {
                        tok = Token::LeftBracket;
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
                    b'\n' | b' ' => {
                        start_offset += 1;
                    }
                    b'"' => {
                        tok = self.string(json_text, offset);
                        break;
                    }
                    b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                        todo!()
                    }
                    b'/' => {
                        if !self.allow_comments {
                            todo!()
                        } else {
                            tok = self.comment(json_text, offset);
                            if tok != Token::Comment {
                                break;
                            }
                            tok = Token::Error;
                            self.buf.clear();
                            self.buf_in_use = false;
                            start_offset = *offset;
                        }
                    }
                    x => {
                        todo!("handle {}: '{}'", x, String::from_utf8_lossy(&[c]));
                        self.error = Some(LexError::InvalidChar);
                        tok = Token::Error;
                        break;
                    }
                }
            }
        }
        if tok == Token::Eof || self.buf_in_use {
            todo!()
        }
        tok
    }
}

const CHAR_LOOKUP_TABLE: [u8; 256] = [
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0x2,
    0,
    0,
    (0x8 | 0x1 | 0x2),
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x1,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0x4,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    (0x8 | 0x1 | 0x2),
    0,
    0,
    0,
    0,
    0x4,
    (0x1 | 0x4),
    0x4,
    0x4,
    0x4,
    (0x1 | 0x4),
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x1,
    0,
    0,
    0,
    0x1,
    0,
    0x1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
    0x10,
];
