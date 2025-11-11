use derive_more::Display;

pub type LineNumber = usize;

#[derive(Debug,Display,Clone,Copy)]
pub enum TokenType<'a> {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier(&'a str), String(&'a str), Number(f32),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}

#[derive(Debug,Display,Clone, Copy)]
#[display("{{ token: {token_type}, line: {line} }}")]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    line: LineNumber,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, line: LineNumber) -> Self {
        Self { token_type, line }
    }

    pub fn eof(line: LineNumber) -> Self {
        Self::new(TokenType::EOF, line)
    }

    pub fn left_paren(line: LineNumber) -> Self {
        Self::new(TokenType::LeftParen,line)
    }

    pub fn right_paren(line: LineNumber) -> Self {
        Self::new(TokenType::RightParen, line)
    }

    pub fn left_brace(line: LineNumber) -> Self {
        Self::new(TokenType::LeftBrace,line)
    }

    pub fn right_brace(line: LineNumber) -> Self {
        Self::new(TokenType::RightBrace, line)
    }

    pub fn comma(line: LineNumber) -> Self {
        Self::new(TokenType::Comma, line)
    }

    pub fn dot(line: LineNumber) -> Self {
        Self::new(TokenType::Dot, line)
    }

    pub fn minus(line: LineNumber) -> Self {
        Self::new(TokenType::Minus, line)
    }

    pub fn plus(line: LineNumber) -> Self {
        Self::new(TokenType::Plus, line)
    }

    pub fn semicolon(line: LineNumber) -> Self {
        Self::new(TokenType::SemiColon, line)
    }

    pub fn star(line: LineNumber) -> Self {
        Self::new(TokenType::Star, line)
    }

    pub fn bang(line: LineNumber) -> Self {
        Self::new(TokenType::Bang, line)
    }

    pub fn bang_equal(line: LineNumber) -> Self {
        Self::new(TokenType::BangEqual, line)
    }

    pub fn equal(line: LineNumber) -> Self {
        Self::new(TokenType::Equal, line)
    }

    pub fn equal_equal(line: LineNumber) -> Self {
        Self::new(TokenType::EqualEqual, line)
    }

    pub fn greater(line: LineNumber) -> Self {
        Self::new(TokenType::Greater, line)
    }

    pub fn greater_equal(line: LineNumber) -> Self {
        Self::new(TokenType::GreaterEqual, line)
    }

    pub fn less(line: LineNumber) -> Self {
        Self::new(TokenType::Less, line)
    }

    pub fn less_equal(line: LineNumber) -> Self {
        Self::new(TokenType::LessEqual, line)
    }

    pub fn slash(line: LineNumber) -> Self {
        Self::new(TokenType::Slash, line)
    }

    pub fn string(line: LineNumber, value: &'a str) -> Self {
        Self::new(TokenType::String(value), line)
    }

    pub fn number(line: LineNumber, value: f32) -> Self {
        Self::new(TokenType::Number(value), line)
    }
}
