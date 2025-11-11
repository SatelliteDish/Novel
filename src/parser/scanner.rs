use std::{
    iter::Iterator as _,
    collections::HashMap,
};
use regex::Regex;
use super::{
    token::{Token,TokenType},
    error::{Result,ParserErr,ParserErrType},
};


#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> std::result::Result<Vec<Token<'a>>, Vec<ParserErr<'a>>> {
        let mut errs = vec![];
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token(&mut tokens) {
                errs.push(e);
            }
        }

        tokens.push(Token::eof(self.line));

        if errs.is_empty() {
            Ok(tokens)
        } else {
            Err(errs)
        }
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token<'a>>) -> std::result::Result<(), ParserErr<'a>> {
        let line = self.line;
        match self.advance() {
            Some('(') => tokens.push(Token::left_paren(line)),
            Some(')') => tokens.push(Token::right_paren(line)),
            Some('{') => tokens.push(Token::left_brace(line)),
            Some('}') => tokens.push(Token::right_brace(line)),
            Some(',') => tokens.push(Token::comma(line)),
            Some('.') => tokens.push(Token::dot(line)),
            Some('-') => tokens.push(Token::minus(line)),
            Some('+') => tokens.push(Token::plus(line)),
            Some(';') => tokens.push(Token::semicolon(line)),
            Some('*') => tokens.push(Token::star(line)),
            Some('!') => {
                if let Some('=') = self.peek() {
                    let _ = self.advance();
                    tokens.push(Token::bang_equal(line));
                } else {
                    tokens.push(Token::bang(line))
                }
            },
            Some('=') => {
                if let Some('=') = self.peek() {
                    let _ = self.advance();
                    tokens.push(Token::equal_equal(line));
                } else {
                    tokens.push(Token::equal(line));
                }
            },
            Some('>') => {
                if let Some('=') = self.peek() {
                    let _ = self.advance();
                    tokens.push(Token::greater_equal(line));
                } else {
                    tokens.push(Token::greater(line));
                }
            },
            Some('<') => {
                if let Some('=') = self.peek() {
                    let _ = self.advance();
                    tokens.push(Token::less_equal(line));
                } else {
                    tokens.push(Token::less(line));
                }
            },
            Some('/') => {
                if let Some('/') = self.peek() { // Start comment
                    while let Some(c) = self.advance() { // Stop on EOF
                        if c == '\n' { // Stop comment on newline
                            self.line += 1;
                            break;
                        }
                    }
                }
            }
            Some('"') => {
                let str = self.parse_string()?;
                tokens.push(Token::string(line, str));
            },
            Some('\n') => {
                self.line += 1;
            },
            Some('\t') | Some('\r') | Some(' ') => {},
            Some(c) => {
                if Self::is_digit(c) {
                    let num = self.parse_number()?;
                    tokens.push(Token::number(line, num));
                } else if Self::is_alpha(c) {
                    tokens.push(self.parse_identifier()?);
                } else {
                    return Err(ParserErr{
                        err_type: ParserErrType::UnexpectedChar(c),
                        line: self.line,
                        message: "Unexpected Token".to_string(),
                    })
                }
            },
            None => {},
        }
        Ok(())
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        match self.source[self.current..].chars().next() {
            Some(c) => {
                self.current += c.len_utf8();
                Some(c)
            },
            None => {
                self.current += 1;
                None
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.current+1..].chars().next()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn parse_string(&mut self) -> Result<'a, &'a str> {
        let mut in_string = true;
        let start = self.current;
        while in_string {
            if let Some(c) = self.advance() {
                if c == '"' {
                    in_string = false;
                }
            } else {
                // Return with an unexpected EOF error
                in_string = false;
            }
        }
        Ok(&self.source[start..self.current-1])
    }

    fn parse_number(&mut self) -> Result<'a, f32> {
        let start = self.current-1; // 1 - to include the first digit
        let mut seen_decimal = false;

        loop {
            if let Some(c) = self.peek() {
                if Self::is_digit(c) {
                    let _ = self.advance();
                } else if let ('.', false, true) = (c, seen_decimal, Self::is_digit(self.peek_next().unwrap_or('b'))) {
                    let _ = self.advance(); // consume the decimal
                    let _ = self.advance(); // consume the already checked number
                    seen_decimal = true;
                } else {
                    break; // number ended, do not consume
                }
            } else {
                // Unexpected EOF
                break;
            }
        }

        Ok(self.source[start..self.current].parse().unwrap())
    }

    fn parse_identifier(&mut self) -> Result<'a, Token<'a>> {

        todo!()
    }

    fn is_digit(c: char) -> bool {
        c == '0' || c == '1' ||
        c == '2' || c == '3' ||
        c == '4' || c == '5' ||
        c == '6' || c == '7' ||
        c == '8' || c == '9'
    }

    fn is_alpha(c: char) -> bool {
        Regex::new("[A-Za-z]").unwrap().is_match(&c.to_string())
    }
}
