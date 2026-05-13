use std::{any::Any, fmt, sync::WaitTimeoutResult};

use crate::parser::{
    lexer::{LexError, Lexer},
    parser_impl::ParseState,
};

mod lexer;
mod parser_impl;

#[derive(Clone, Debug, Default)]
pub struct ParserOptions {
    pub allow_comments: bool,
    pub dont_validate_strings: bool,
    pub allow_multiple_values: bool,
    pub allow_partial_values: bool,
    pub allow_trailing_garbage: bool,
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
    pub fn allow_partial_values(&mut self, yes: bool) {
        self.allow_partial_values = yes;
    }
    pub fn allow_trailing_garbage(&mut self, yes: bool) {
        self.allow_trailing_garbage = yes;
    }
}

#[derive(Debug)]
pub struct Parser<'ctx> {
    callbacks: Option<&'ctx ParserCallbacks>,
    ctx: &'ctx mut dyn Any,
    lexer: Lexer,
    state_stack: Vec<ParseState>,
    options: ParserOptions,
    error: Option<ParseError>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CallbackStatus {
    Continue,
    Cancel,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParserCallbacks {
    pub null: Option<fn(&mut dyn Any) -> CallbackStatus>,
    pub boolean: Option<fn(&mut dyn Any, bool) -> CallbackStatus>,
    pub integer: Option<fn(&mut dyn Any, i64) -> CallbackStatus>,
    pub double: Option<fn(&mut dyn Any, f64) -> CallbackStatus>,
    pub number: Option<fn(&mut dyn Any, &[u8]) -> CallbackStatus>,
    pub string: Option<fn(&mut dyn Any, &[u8]) -> CallbackStatus>,
    pub start_map: Option<fn(&mut dyn Any) -> CallbackStatus>,
    pub map_key: Option<fn(&mut dyn Any, &[u8]) -> CallbackStatus>,
    pub end_map: Option<fn(&mut dyn Any) -> CallbackStatus>,
    pub start_array: Option<fn(&mut dyn Any) -> CallbackStatus>,
    pub end_array: Option<fn(&mut dyn Any) -> CallbackStatus>,
}

impl<'ctx> Parser<'ctx> {
    pub fn new(
        callbacks: Option<&'ctx ParserCallbacks>,
        ctx: &'ctx mut dyn Any,
        options: ParserOptions,
    ) -> Self {
        let state_stack = vec![ParseState::Start];
        Self {
            callbacks,
            ctx,
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

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    ClientCancelled,
    FloatingPointOverflow,
    IntegerOverflow,
    InvalidArraySeparator,
    InvalidKeyValueSeparator,
    InvalidObjectKey,
    InvalidObjectSeparator,
    InvalidToken,
    PrematureEof,
    TrailingGarbage,
    UnallowedToken,
    LexicalError(LexError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("parse error: ")?;
        match self {
            Self::ClientCancelled => {
                f.write_str("client cancelled parse via callback return value")
            }
            Self::FloatingPointOverflow => f.write_str("numeric (floating point) overflow"),
            Self::IntegerOverflow => f.write_str("integer overflow"),
            Self::InvalidArraySeparator => f.write_str("after array element, I expect ',' or ']'"),
            Self::InvalidObjectKey => write!(f, "invalid object key (must be a string)"),
            Self::InvalidObjectSeparator => {
                f.write_str("after key and value, inside map, I expect ',' or '}'")
            }
            Self::InvalidKeyValueSeparator => {
                f.write_str("object key and value must be separated by a colon (':')")
            }
            Self::InvalidToken => f.write_str("invalid token, internal error"),
            Self::PrematureEof => write!(f, "premature EOF"),
            Self::TrailingGarbage => f.write_str("trailing garbage"),
            Self::UnallowedToken => f.write_str("unallowed token at this point in JSON text"),
            Self::LexicalError(err) => f.write_fmt(format_args!("lexical error: {}", err)),
        }
    }
}

impl std::error::Error for ParseError {}
