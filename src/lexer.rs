use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            position: 0,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn consume_char(&mut self) -> Option<char> {
        self.position += 1;
        self.chars.next()
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_ascii_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    pub fn read_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(c) = self.peek() {
            if predicate(c) {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    pub fn read_integer(&mut self) -> i64 {
        let start = self.position;
        self.read_while(|c| c.is_ascii_digit());
        self.source[start..self.position]
            .parse()
            .expect("Failed to parse integer")
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(ch) = self.consume_char() {
            match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '^' => Token::Caret,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '0'..='9' => {
                    // Go back one char because consume_char already took the first digit
                    // This is a common pattern for "read_number" in lexers.
                    self.position -= 1;
                    self.chars = self.source[self.position..].chars().peekable();
                    Token::Integer(self.read_integer())
                }
                _ => Token::Illegal,
            }
        } else {
            Token::EOF
        }
    }
}

#[test]
fn next_token_test() {
    let source = "
    5+5*5
    -5+5^2/25
    ";
    let mut lexer = Lexer::new(source);
    let tokens = vec![
        Token::Integer(5),
        Token::Plus,
        Token::Integer(5),
        Token::Asterisk,
        Token::Integer(5),
        Token::Minus,
        Token::Integer(5),
        Token::Plus,
        Token::Integer(5),
        Token::Caret,
        Token::Integer(2),
        Token::Slash,
        Token::Integer(25),
        Token::EOF,
    ];

    for t in tokens {
        assert_eq!(t, lexer.next_token())
    }
}
