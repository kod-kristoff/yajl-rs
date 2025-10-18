use crate::parser::lexer::Token;

use super::{ParseError, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParseState {
    Start = 0,
    ParseComplete = 1,
    ParseError = 2,
    LexicalError = 3,
    MapStart = 4,
    MapSep = 5,
    MapNeedVal = 6,
    MapGotVal = 7,
    MapNeedKey = 8,
    ArrayStart = 9,
    ArrayGotVal = 10,
    ArrayNeedVal = 11,
    GotValue = 12,
}

impl Parser {
    fn set_error(&mut self, error: ParseError) {
        self.error = Some(error);
        *self.state_stack.last_mut().unwrap() = ParseState::ParseError;
    }
    fn set_stack_top(&mut self, top: ParseState) {
        *self
            .state_stack
            .last_mut()
            .expect("state should always have at least one elem") = top;
    }
    fn get_stack_top(&mut self) -> ParseState {
        self.state_stack
            .last()
            .copied()
            .expect("state should always have at least one elem")
    }
    pub(crate) fn do_parse(&mut self, text: &[u8]) -> Result<(), ParseError> {
        let mut offset = 0;
        loop {
            dbg!(&self.state_stack);
            let top = self.get_stack_top();
            match top {
                ParseState::ParseError => {
                    let error = self.error.clone().unwrap();
                    return Err(error);
                }
                ParseState::Start
                | ParseState::MapNeedVal
                | ParseState::ArrayNeedVal
                | ParseState::ArrayStart => {
                    let mut state_to_push = ParseState::Start;
                    let tok = self.lexer.lex(text, &mut offset);
                    let mut valid_token = false;
                    match tok {
                        Ok(Token::String) => {
                            dbg!("callback string");
                            valid_token = true;
                        }
                        Ok(Token::LeftCurlyBracket) => {
                            dbg!("callback start_map");
                            state_to_push = ParseState::MapStart;
                            valid_token = true;
                        }
                        Ok(Token::LeftSquareBracket) => {
                            dbg!("callback start_array");
                            state_to_push = ParseState::ArrayStart;
                            valid_token = true;
                        }
                        t => todo!("handle tok={:?}", t),
                    }
                    dbg!(&state_to_push);
                    if valid_token {
                        match self.get_stack_top() {
                            ParseState::MapNeedVal => {
                                self.set_stack_top(ParseState::MapGotVal);
                            }
                            _ => {
                                self.set_stack_top(ParseState::ArrayGotVal);
                            }
                        }
                        if state_to_push != ParseState::Start {
                            self.state_stack.push(state_to_push);
                        }
                    } else {
                        self.set_error(ParseError::UnallowedToken);
                    }
                }
                ParseState::MapStart | ParseState::MapNeedKey => {
                    let tok = self.lexer.lex(text, &mut offset);
                    let mut found_key = false;
                    match tok {
                        Ok(Token::Eof) => return Ok(()),
                        Ok(Token::String) => {
                            found_key = true;
                        }
                        _ => {
                            self.set_error(ParseError::InvalidObjectKey);
                        }
                    }

                    if found_key {
                        self.set_stack_top(ParseState::MapSep);
                    }
                }
                ParseState::MapSep => {
                    let tok = self.lexer.lex(text, &mut offset);
                    match tok {
                        Ok(Token::Eof) => return Ok(()),
                        Ok(Token::Colon) => {
                            self.set_stack_top(ParseState::MapNeedVal);
                        }
                        _ => {
                            self.set_error(ParseError::InvalidKeyValueSeparator);
                        }
                    }
                }
                ParseState::MapGotVal => {
                    let tok = self.lexer.lex(text, &mut offset);
                    dbg!(&tok);
                    match tok {
                        Ok(Token::Comma) => {
                            self.set_stack_top(ParseState::MapNeedKey);
                        }
                        _ => {
                            self.set_error(ParseError::InvalidObjectSeparator);
                            // offset -= bufLen
                        }
                    }
                }
                ParseState::ArrayGotVal => {
                    let tok = self.lexer.lex(text, &mut offset);
                    dbg!(&tok);
                    match tok {
                        Ok(Token::Comma) => self.set_stack_top(ParseState::ArrayNeedVal),
                        Ok(Token::RightSquareBracket) => {
                            dbg!("callback end_array");
                            self.state_stack.pop();
                        }
                        _ => todo!("handle tok={:?}", tok),
                    }
                }
                state => todo!("handle top={:?}", state),
            }
        }
        todo!()
    }

    pub(crate) fn do_finish(&mut self) -> Result<(), ParseError> {
        self.do_parse(b" ")?;

        match self.state_stack.last().unwrap() {
            ParseState::ParseError => {
                let error = self.error.clone().unwrap();
                Err(error)
            }
            _ => {
                if self.options.allow_partial_values {
                    Ok(())
                } else {
                    let error = ParseError::PrematureEof;
                    self.set_error(error.clone());
                    Err(error)
                }
            }
        }
    }
}
