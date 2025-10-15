use crate::parser::ParseError;

use super::ParserOption;

use self::rlexer::RLexer;
use self::rparser_impl::RByteStack;

mod rlexer;
mod rparser_impl;

#[derive(Debug, Clone)]
pub struct RParser {
    flags: u32,
    bytes_consumed: usize,
    parse_error: Option<ParseError>,
    state_stack: RByteStack,
    lexer: RLexer,
}

impl RParser {
    pub fn new() -> Self {
        Self {
            flags: 0,
            bytes_consumed: 0,
            parse_error: None,
            state_stack: RByteStack::new(),
            lexer: RLexer::new(),
        }
    }

    pub fn config(&mut self, opt: ParserOption, arg: bool) -> bool {
        if arg {
            self.flags |= opt as u32;
        } else {
            self.flags &= !(opt as u32);
        }
        true
    }
}

impl RParser {
    pub fn parse(&mut self, json_text: &[u8]) -> Result<(), String> {
        self.do_parse(json_text)
    }

    pub fn complete_parse(&mut self) -> Result<(), String> {
        return Err("bad".into());
        Ok(())
    }
}
