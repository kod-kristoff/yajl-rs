use std::ops::Range;

// #[derive(Clone, Debug)]
// pub struct Token<'t> {
//     pub text: &'t [u8],
//     pub kind: TokenKind,
//     pub span: Range<usize>,
// }
//
// #[derive(Clone, Copy, Debug)]
// pub enum TokenKind {
//     LeftCurlyBrace,
//     Comment,
//     Eof,
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Token {
    Bool = 0,
    Colon = 1,
    Comma = 2,
    Eof = 3,
    Error = 4,
    LeftSquareBracket = 5,
    LeftCurlyBracket = 6,
    Null = 7,
    RightSquareBracket = 8,
    RightCurlyBracket = 9,
    Integer = 10,
    Double = 11,
    String = 12,
    StringWithEscapes = 13,
    Comment = 14,
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
static charLookupTable: [u8;256] =
[
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

#[derive(Clone, Debug)]
pub struct Lexer {
    allow_comments: bool,
    validate_utf8: bool,
    error: Option<LexError>,
}

#[derive(Clone, Debug)]
pub struct LexerOptions {
    pub allow_comments: bool,
    pub validate_utf8: bool,
}

impl Default for LexerOptions {
    fn default() -> Self {
        Self {
            allow_comments: false,
            validate_utf8: true,
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new(LexerOptions::default())
    }
}
impl Lexer {
    pub fn new(
        LexerOptions {
            allow_comments,
            validate_utf8,
        }: LexerOptions,
    ) -> Self {
        Self {
            allow_comments,
            validate_utf8,
            error: None,
        }
    }

    fn read_char(&mut self, text: &[u8], offset: &mut usize) -> u8 {
        let c = text[*offset];
        *offset += 1;
        c
    }
    fn unread_char(&mut self, offset: &mut usize) {
        if *offset > 0 {
            *offset -= 1;
        } else {
            todo!()
        }
    }
    fn set_error(&mut self, error: LexError) {
        self.error = Some(error);
    }
    pub fn lex(&mut self, text: &[u8], offset: &mut usize) -> Result<Token, LexError> {
        // let mut kind = TokenKind::LeftCurlyBrace;
        let mut start_offset = *offset;
        let mut tok = Token::Error;
        'lex: loop {
            if *offset >= text.len() {
                // kind = TokenKind::Eof;
                tok = Token::Eof;
                break;
            }
            let c = self.read_char(text, offset);
            dbg!(&c);
            match c {
                b'{' => {
                    tok = Token::LeftCurlyBracket;
                    break;
                }
                b'}' => {
                    tok = Token::RightCurlyBracket;
                    break;
                }
                b'[' => {
                    tok = Token::LeftSquareBracket;
                    break;
                }
                b']' => {
                    tok = Token::RightSquareBracket;
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
                b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r' | b' ' => {
                    start_offset += 1;
                }
                b't' => {
                    let mut want = &b"rue"[..];
                    loop {
                        if *offset >= text.len() {
                            tok = Token::Eof;
                            break 'lex;
                        }
                        let c = self.read_char(text, offset);
                        if c != want[0] {
                            self.unread_char(offset);
                            self.set_error(LexError::InvalidString);
                            tok = Token::Error;
                            break 'lex;
                        }
                        want = &want[1..];
                        if want.is_empty() {
                            break;
                        }
                    }
                    tok = Token::Bool;
                    break;
                }
                b'f' => {
                    let mut want = &b"false"[..];
                    loop {
                        if *offset >= text.len() {
                            tok = Token::Eof;
                            break 'lex;
                        }
                        let c = self.read_char(text, offset);
                        if c != want[0] {
                            self.unread_char(offset);
                            self.set_error(LexError::InvalidString);
                            tok = Token::Error;
                            break 'lex;
                        }
                        want = &want[1..];
                        if want.is_empty() {
                            break;
                        }
                    }
                    tok = Token::Bool;
                    break;
                }
                b'n' => {
                    let mut want = &b"null"[..];
                    loop {
                        if *offset >= text.len() {
                            tok = Token::Eof;
                            break 'lex;
                        }
                        let c = self.read_char(text, offset);
                        if c != want[0] {
                            self.unread_char(offset);
                            self.set_error(LexError::InvalidString);
                            tok = Token::Error;
                            break 'lex;
                        }
                        want = &want[1..];
                        if want.is_empty() {
                            break;
                        }
                    }
                    tok = Token::Bool;
                    break;
                }
                b'"' => {
                    tok = self.string(text, offset);
                    break;
                }
                b'-' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    self.unread_char(offset);
                    tok = self.number(text, offset);
                    break;
                }
                b'/' => {
                    // kind = TokenKind::Comment;
                    if !self.allow_comments {
                        self.unread_char(offset);
                        self.error = Some(LexError::UnallowedComment);
                        tok = Token::Error;
                        break;
                    }
                    tok = self.comment(text, offset);
                    if tok != Token::Comment {
                        break;
                    }
                    // reset tok to inital value
                    tok = Token::Error;
                    start_offset = *offset;
                    continue;
                }
                _invalid_char => {
                    self.error = Some(LexError::InvalidChar);
                    tok = Token::Error;
                    // todo!(" handle c={}", ch)},
                    // TODO: return error here
                }
            }
        }
        dbg!(&tok);
        // todo!()
        Ok(tok)
    }

    fn number(&mut self, text: &[u8], offset: &mut usize) -> Token {
        let mut tok = Token::Integer;
        let mut c = self.read_char(text, offset);
        if c == b'-' {
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
        }

        if c == b'0' {
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
        } else if b'1' <= c && c <= b'9' {
            loop {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
                if !(b'0' <= c && c <= b'9') {
                    break;
                }
            }
        } else {
            self.unread_char(offset);
            self.set_error(LexError::MissingIntegerAfterMinus);
            return Token::Error;
        }
        if c == b'.' {
            if *offset >= text.len() {
                return Token::Eof;
            }
            let mut num_rd = 0;
            c = self.read_char(text, offset);
            while b'0' <= c && c <= b'9' {
                num_rd += 1;
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
            }
            if num_rd == 0 {
                self.unread_char(offset);
                self.set_error(LexError::MissingIntegerAfterDecimal);
                return Token::Error;
            }
            tok = Token::Double;
        }
        if c == b'e' || c == b'E' {
            if *offset >= text.len() {
                return Token::Eof;
            }
            c = self.read_char(text, offset);
            if c == b'+' || c == b'-' {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
            }
            if b'0' <= c && c <= b'9' {
                loop {
                    if *offset >= text.len() {
                        return Token::Eof;
                    }
                    c = self.read_char(text, offset);
                    if !(b'0' <= c && c <= b'9') {
                        break;
                    }
                }
            } else {
                self.unread_char(offset);
                self.set_error(LexError::MissingIntegerAfterExponent);
                return Token::Error;
            }
            tok = Token::Double;
        }
        self.unread_char(offset);
        tok
    }
    fn string(&mut self, text: &[u8], offset: &mut usize) -> Token {
        let mut tok = Token::Error;
        let mut has_escapes = false;
        'string: loop {
            /* now jump into a faster scanning routine to skip as much
             * of the buffers as possible */
            if *offset < text.len() {
                *offset += dbg!(string_scan(&text[*offset..], self.validate_utf8));
            }

            if dbg!(*offset) >= text.len() {
                tok = Token::Eof;
                break;
            }
            let mut curr_char = dbg!(self.read_char(text, offset));

            if curr_char == b'"' {
                tok = Token::String;
                break;
            } else if curr_char == b'\\' {
                has_escapes = true;
                if *offset >= text.len() {
                    tok = Token::Eof;
                    break;
                }
                dbg!(&has_escapes);
                curr_char = dbg!(self.read_char(text, offset));
                if curr_char == b'u' {
                    let mut i = 0;
                    while i < 4 {
                        if *offset >= text.len() {
                            tok = Token::Eof;
                            break 'string;
                        }
                        curr_char = dbg!(self.read_char(text, offset));
                        if dbg!(charLookupTable[curr_char as usize]) & VHC == 0 {
                            self.unread_char(offset);
                            self.set_error(LexError::StringInvalidHexChar);
                            tok = Token::Error;
                            break 'string;
                        }
                        i += 1;
                    }
                } else if charLookupTable[curr_char as usize] & VEC == 0 {
                    self.unread_char(offset);
                    self.set_error(LexError::StringInvalidEscapedChar);
                    tok = Token::Error;
                    break;
                }
            } else if charLookupTable[curr_char as usize] & IJC != 0 {
                self.unread_char(offset);
                self.set_error(LexError::StringInvalidJsonChar);
                tok = Token::Error;
                break;
            } else if self.validate_utf8 {
                let t = self.utf8_char(text, offset, curr_char);
                if t == Token::Eof {
                    tok = Token::Eof;
                    break;
                } else if t == Token::Error {
                    self.set_error(LexError::StringInvalidUtf8);
                    break;
                }
            }

            // accept it and move on
        }
        if has_escapes && tok == Token::String {
            tok = Token::StringWithEscapes;
        }
        dbg!(tok)
    }
    fn utf8_char(&mut self, text: &[u8], offset: &mut usize, mut curr_char: u8) -> Token {
        if curr_char <= 0x7f {
            // single byte
            return Token::String;
        } else if (curr_char >> 5) == 0x6 {
            // two bytes
            if *offset >= text.len() {
                return Token::Eof;
            }
            curr_char = self.read_char(text, offset);
            if (curr_char >> 6) == 0x2 {
                return Token::String;
            }
        } else if (curr_char >> 4) == 0x0e {
            // three bytes
            if *offset >= text.len() {
                return Token::Eof;
            }
            curr_char = self.read_char(text, offset);
            if (curr_char >> 6) == 0x2 {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                curr_char = self.read_char(text, offset);
                if (curr_char >> 6) == 0x2 {
                    return Token::String;
                }
            }
        } else if (curr_char >> 3) == 0x1e {
            // four bytes
            if *offset >= text.len() {
                return Token::Eof;
            }
            curr_char = self.read_char(text, offset);
            if (curr_char >> 6) == 0x2 {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                curr_char = self.read_char(text, offset);
                if (curr_char >> 6) == 0x2 {
                    if *offset >= text.len() {
                        return Token::Eof;
                    }
                    curr_char = self.read_char(text, offset);
                    if (curr_char >> 6) == 0x2 {
                        return Token::String;
                    }
                }
            }
        }
        Token::Error
    }
    fn comment(&mut self, text: &[u8], offset: &mut usize) -> Token {
        if *offset >= text.len() {
            return Token::Eof;
        }
        let mut c = self.read_char(text, offset);
        if c == b'/' {
            loop {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
                if c == b'\n' {
                    break;
                }
            }
            Token::Comment
        } else if c == b'*' {
            loop {
                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
                if c != b'*' {
                    continue;
                }

                if *offset >= text.len() {
                    return Token::Eof;
                }
                c = self.read_char(text, offset);
                if c == b'/' {
                    break;
                }
                self.unread_char(offset)
            }
            Token::Comment
        } else {
            self.error = Some(LexError::InvalidChar);
            Token::Error
        }
    }
}

fn string_scan(buf: &[u8], utf8check: bool) -> usize {
    let mask = IJC | NFP | if utf8check { NUC } else { 0 };
    let mut skip = 0;
    while skip < buf.len() && charLookupTable[buf[skip] as usize] & mask == 0 {
        skip += 1;
    }
    skip
}
#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum LexError {
    Ok = 0,
    StringInvalidUtf8 = 1,
    StringInvalidEscapedChar = 2,
    StringInvalidJsonChar = 3,
    StringInvalidHexChar = 4,
    InvalidChar = 5,
    InvalidString = 6,
    MissingIntegerAfterDecimal = 7,
    MissingIntegerAfterExponent = 8,
    MissingIntegerAfterMinus = 9,
    UnallowedComment = 10,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // valid strings
    #[case("\"simple\"".as_bytes())]
    #[case(r#""nära""#.as_bytes())]
    #[case(r#""sࠉ""#.as_bytes())]
    #[case(&[34, 67, 78, 92, 0x72, 34])]
    // invalid strings
    #[case(&[34, 67, 10, 34])]
    #[case(&[34, 92, 117, 48, 48, 43, 43, 34])]
    #[case(&[34, 92, 20, 40, 42, 43, 44, 34])]
    #[case(&[34, 250, 20, 40, 42, 43, 44, 34])]
    // string EOFs
    #[case(b"\"s")]
    #[case(&[34, 117, 92])]
    #[case(&[34, 92, 117, 48])]
    #[case(&[34, 96, 0xe0])]
    #[case(&[34, 97, 0xe0, 0xa0])]
    #[case(&[34, 98, 0xe0, 0xa0, 0x80])]
    #[case(&[34, 99, 0xf0])]
    #[case(&[34, 100, 0xf0, 0x90])]
    #[case(&[34, 101, 0xf0, 0x90, 0x80])]
    #[case(&[34, 102, 0xf0, 0x90, 0x80, 0x80])]
    fn lex(#[case] text: &[u8]) {
        let mut lexer = Lexer::default();
        let mut offset = 0;
        let token = lexer.lex(text, &mut offset);
        insta::assert_debug_snapshot!(
            format!("lex_case_{}", String::from_utf8_lossy(text)),
            (token, offset, lexer)
        );
    }
}
