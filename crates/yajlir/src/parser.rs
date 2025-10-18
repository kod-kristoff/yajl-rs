use std::fmt;

use crate::parser::{lexer::Lexer, parser_impl::ParseState};

mod lexer;
mod parser_impl;

#[derive(Clone, Debug, Default)]
pub struct ParserOptions {
    pub allow_comments: bool,
    pub dont_validate_strings: bool,
    pub allow_multiple_values: bool,
    pub allow_partial_values: bool,
}

impl ParserOptions {
    pub fn allow_comments(&mut self, yes: bool) {
        self.allow_comments = yes;
    }
    pub fn dont_validate_strings(&mut self, yes: bool) {
        self.dont_validate_strings = yes;
    }
    pub fn allow_multiple_values(&mut self, yes: bool) {
        self.allow_multiple_values = yes;
    }
}
#[derive(Clone, Debug)]
pub struct Parser {
    lexer: Lexer,
    state_stack: Vec<ParseState>,
    options: ParserOptions,
    error: Option<ParseError>,
}

impl Parser {
    pub fn new(options: ParserOptions) -> Self {
        let state_stack = vec![ParseState::Start];
        Self {
            lexer: Lexer::new(lexer::LexerOptions {
                allow_comments: options.allow_comments,
                validate_utf8: !options.dont_validate_strings,
            }),
            state_stack,
            options,
            error: None,
        }
    }
    pub fn parse(&mut self, text: &[u8]) -> Result<(), ParseError> {
        self.do_parse(text)
    }
    pub fn complete_parse(&mut self) -> Result<(), ParseError> {
        self.do_finish()
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidObjectKey,
    InvalidObjectSeparator,
    InvalidKeyValueSeparator,
    PrematureEof,
    UnallowedToken,
    Unknown(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidObjectKey => write!(f, "invalid object key (must be a string)"),
            Self::InvalidObjectSeparator => {
                f.write_str("after key and value, inside map, I expect ',' or '}'")
            }
            Self::InvalidKeyValueSeparator => {
                f.write_str("object key and value must be separated by a colon (':')")
            }
            Self::PrematureEof => write!(f, "premature EOF"),
            Self::UnallowedToken => f.write_str("unallowed token at this point in JSON text"),
            Self::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}
