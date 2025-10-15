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

impl Lexer {
    pub fn new(allow_comments: bool) -> Self {
        Self {
            allow_comments,
            validate_utf8: true,
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
                b't' => todo!("true"),
                b'f' => todo!("false"),
                b'n' => todo!("null"),
                b'"' => {
                    tok = self.string(text, offset);
                    break;
                }
                b'-' | b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    todo!("number")
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
        dbg!(&tok);
        // todo!()
        Ok(tok)
    }

    fn string(&mut self, text: &[u8], offset: &mut usize) -> Token {
        let mut tok = Token::Error;
        let mut has_escapes = false;
        loop {
            /* now jump into a faster scanning routine to skip as much
             * of the buffers as possible */
            if *offset < text.len() {
                *offset += dbg!(string_scan(&text[*offset..], self.validate_utf8));
            }

            if dbg!(*offset) >= text.len() {
                tok = Token::Eof;
                break;
            }
            let curr_char = dbg!(self.read_char(text, offset));

            if curr_char == b'"' {
                tok = Token::String;
                break;
            }
            todo!()
        }
        dbg!(tok)
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
#[derive(Clone, Debug)]
pub enum LexError {
    InvalidChar,
    UnallowedComment,
}
