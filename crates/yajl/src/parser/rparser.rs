use rlexer::RLexer;

use crate::{parser::lexer::Token, Status};

use super::{parser_impl::ParseState, ParserOptions};

mod rlexer;

pub struct RParser {
    flags: u32,
    lexer: RLexer,
    state_stack: Vec<ParseState>,
    bytes_consumed: usize,
    options: ParserOptions,
}

impl RParser {
    pub fn new() -> Self {
        Self::with_options(ParserOptions::default())
    }
    pub fn with_options(options: ParserOptions) -> Self {
        Self {
            flags: 0,
            lexer: RLexer::new(options.allow_comments(), !options.dont_validate_string()),
            state_stack: vec![ParseState::Start],
            bytes_consumed: 0,
            options,
        }
    }

    pub fn complete_parse(&self) -> Status {
        todo!()
    }

    pub fn config(&mut self, opt: super::ParserOption, arg: bool) {
        if arg {
            self.flags |= opt as u32;
        } else {
            self.flags &= !(opt as u32);
        }
    }

    fn do_parse(&mut self, json_text: &[u8]) -> Status {
        let mut tok = Token::Bool;
        let offset = &mut self.bytes_consumed;
        *offset = 0;
        loop {
            match self.state_stack.last().unwrap() {
                ParseState::Start => {
                    let mut state_to_push = ParseState::Start;
                    tok = self.lexer.lex(json_text, offset);
                }
                x => todo!("handle {:?}", x),
            }
        }
        todo!()
    }
}

impl RParser {
    pub fn parse(&mut self, json_text: &[u8]) -> Status {
        self.do_parse(json_text)
    }
}
