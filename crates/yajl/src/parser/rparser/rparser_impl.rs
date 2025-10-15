use crate::parser::{ParseError, RParser};

#[derive(Debug, Clone)]
pub struct RByteStack {}

impl RByteStack {
    pub fn new() -> Self {
        Self {}
    }
}
impl RParser {
    pub fn do_parse(&mut self, text: &[u8]) -> Result<(), String> {
        Ok(())
    }
}
