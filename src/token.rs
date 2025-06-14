#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Integer(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret, // For power (^)
    LParen,
    RParen,
    EOF,
    Illegal,
}
