use std::fmt;
use std::ops::Range;

use logos::Logos;

#[derive(Debug, Clone)]
pub struct Token<'text> {
    pub text: &'text str,
    pub kind: TokenKind,
    pub span: Range<usize>,
}

impl<'text> Token<'text> {
    pub fn new(text: &'text str, kind: TokenKind, span: Range<usize>) -> Self {
        Token { text, kind, span }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.kind, self.text)
    }
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    #[regex(r#""([^"\\])*""#)]
    String,
    #[token("{")]
    LeftCurly,
    #[token("}")]
    RightCurly,
    #[token(":")]
    Colon,
}

/// Zero-copy tokenization function
/// The key insight: we return tokens that reference the original input
/// No string copying happens here!
pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut lexer = TokenKind::lexer(input);

    while let Some(result) = lexer.next() {
        dbg!(&tokens);
        if let Ok(kind) = result {
            let span = lexer.span();
            let text = &input[span.clone()];
            tokens.push(Token::new(text, kind, span));
        }
    }

    // Add EOF token
    // let len = input.len();
    // tokens.push(Token::new("", TokenKind::Eof, len..len));

    tokens
}
