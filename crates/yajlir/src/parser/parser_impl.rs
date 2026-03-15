use crate::{parser::lexer::Token, CallbackStatus};

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

impl<'ctx> Parser<'ctx> {
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
                ParseState::ParseComplete => {
                    if self.options.allow_multiple_values {
                        self.set_stack_top(ParseState::GotValue);
                    } else {
                        if self.options.allow_trailing_garbage {
                            break;
                        }
                        if offset == text.len() {
                            break;
                        }
                        let tok = self.lexer.lex(text, &mut offset);
                        if tok != Ok(Token::Eof) {
                            self.set_error(ParseError::TrailingGarbage);
                        }
                    }
                }
                ParseState::ParseError | ParseState::LexicalError => {
                    let error = self.error.clone().unwrap();
                    return Err(error);
                }
                ParseState::Start
                | ParseState::GotValue
                | ParseState::MapNeedVal
                | ParseState::ArrayNeedVal
                | ParseState::ArrayStart => {
                    let mut state_to_push = ParseState::Start;
                    let tok = self.lexer.lex(text, &mut offset);
                    let mut valid_token = false;
                    match tok {
                        Ok(Token::Eof) => return Ok(()),
                        Ok(Token::Error) => {
                            self.set_stack_top(ParseState::LexicalError);
                            continue;
                        }
                        Ok(Token::String) => {
                            self.call_string(b"string")?;
                            valid_token = true;
                        }
                        Ok(Token::StringWithEscapes) => {
                            self.call_string(b"string with escapes")?;
                            valid_token = true;
                        }
                        Ok(Token::Bool) => {
                            self.call_boolean(false)?;
                            valid_token = true;
                        }
                        Ok(Token::Null) => {
                            self.call_null()?;
                            valid_token = true;
                        }
                        Ok(Token::LeftCurlyBracket) => {
                            self.call_start_map()?;
                            state_to_push = ParseState::MapStart;
                            valid_token = true;
                        }
                        Ok(Token::LeftSquareBracket) => {
                            self.call_start_array()?;
                            state_to_push = ParseState::ArrayStart;
                            valid_token = true;
                        }
                        Ok(Token::Integer) => {
                            let buf = b"0";
                            if !self.call_number(buf)? {
                                let Ok(val) = parse_integer(buf) else {
                                    self.set_error(ParseError::IntegerOverflow);
                                    // TODO rewind offset
                                    continue;
                                };
                                self.call_integer(val)?;
                            }
                        }
                        Ok(Token::RightSquareBracket) => {
                            if top == ParseState::ArrayStart {
                                self.call_end_array()?;
                                self.state_stack.pop();
                                continue;
                            } else {
                                valid_token = false;
                            }
                        }
                        Ok(Token::Integer) => {
                            dbg!("callback number");
                            dbg!("or callback integer");
                            valid_token = true;
                        }
                        t => todo!("handle tok={:?}", t),
                    }
                    dbg!(&state_to_push);
                    if valid_token {
                        match self.get_stack_top() {
                            ParseState::Start | ParseState::GotValue => {
                                self.set_stack_top(ParseState::ParseComplete);
                            }
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
                        Ok(Token::RightCurlyBracket) => {
                            self.call_end_map()?;
                            self.state_stack.pop();
                        }
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
                        Ok(Token::Eof) => return Ok(()),
                        Ok(Token::RightSquareBracket) => {
                            self.call_end_array()?;
                            self.state_stack.pop();
                        }
                        _ => todo!("handle tok={:?}", tok),
                    }
                }
                state => todo!("handle top={:?}", state),
            }
        }
        Ok(())
    }

    fn call_null(&mut self) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_null) = callbacks.null {
                if (callback_null)(self.ctx) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_boolean(&mut self, val: bool) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_boolean) = callbacks.boolean {
                if (callback_boolean)(self.ctx, val) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_integer(&mut self, val: i64) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_integer) = callbacks.integer {
                if (callback_integer)(self.ctx, val) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_double(&mut self, val: f64) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_double) = callbacks.double {
                if (callback_double)(self.ctx, val) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    /// Call number callback if set.
    /// Returns:
    /// - `Ok(true)` if the callback `number` exists
    /// - `Ok(false)` if the callback `number` doesn't exists
    /// - `Err(ParseError::ClientCancelled)` if the callback `number` returns cancel.
    fn call_number(&mut self, val: &[u8]) -> Result<bool, ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_number) = callbacks.number {
                if (callback_number)(self.ctx, val) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn call_string(&mut self, val: &[u8]) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_string) = callbacks.string {
                if (callback_string)(self.ctx, val) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_start_map(&mut self) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_start_map) = callbacks.start_map {
                if (callback_start_map)(self.ctx) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_map_key(&mut self, val: &[u8]) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_map_key) = callbacks.map_key {
                if (callback_map_key)(self.ctx, val) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_end_map(&mut self) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_end_map) = callbacks.end_map {
                if (callback_end_map)(self.ctx) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_start_array(&mut self) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_start_array) = callbacks.start_array {
                if (callback_start_array)(self.ctx) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    fn call_end_array(&mut self) -> Result<(), ParseError> {
        if let Some(callbacks) = self.callbacks {
            if let Some(callback_end_array) = callbacks.end_array {
                if (callback_end_array)(self.ctx) == CallbackStatus::Cancel {
                    self.set_error(ParseError::ClientCancelled);
                    return Err(ParseError::ClientCancelled);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn do_finish(&mut self) -> Result<(), ParseError> {
        self.do_parse(b" ")?;

        match self.state_stack.last().unwrap() {
            ParseState::ParseError => {
                let error = self.error.clone().unwrap();
                Err(error)
            }
            ParseState::GotValue | ParseState::ParseComplete => Ok(()),
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

fn parse_integer(number: &[u8]) -> Result<i64, ParseIntegerError> {
    let mut ret: i64 = 0;
    let mut sign: i8 = 1;
    let mut pos = number;
    if pos[0] == b'-' {
        sign = -1;
        pos = &pos[1..];
    } else if pos[0] == b'+' {
        pos = &pos[1..];
    }
    while !pos.is_empty() {
        if ret > MAX_VALUE_TO_MULTIPLY {
            return if sign == 1 {
                Err(ParseIntegerError::Overflow)
            } else {
                Err(ParseIntegerError::Underflow)
            };
        }
        ret *= 10;
        if i64::MAX - ret < (pos[0] - b'0') as i64 {
            return if sign == 1 {
                Err(ParseIntegerError::Overflow)
            } else {
                Err(ParseIntegerError::Underflow)
            };
        }
        if pos[0] < b'0' || pos[0] > b'9' {
            return Err(ParseIntegerError::NonNumerical(pos[0]));
        }
        ret += (pos[0] - b'0') as i64;
        pos = &pos[1..];
    }
    Ok(sign as i64 * ret)
}
