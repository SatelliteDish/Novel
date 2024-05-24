use regex::Regex;

mod token;
pub use token::{Token,TokenType};

use super::error_handler::SyntaxError;

#[derive(PartialEq, Eq, Hash, Clone)]

pub struct Tokenizer {
    text: String,
    pointer: usize,
    pub token: Option<Token>,
    pub next_token: Option<Token>
}
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            text: String::from(text),
            pointer: 0,
            next_token: None,
            token: None
        };
        tokenizer.scan();
        tokenizer
    }
    pub fn scan(&mut self) {
        //loop to get get next non-whitespace token for current token
        loop {
            self.token = match self.get_next_token() {
                Some(tkn) => {
                    let tkn_len = match tkn.token_type {
                        TokenType::StringLiteral => tkn.val.len() + 2,
                        _ => tkn.val.len()
                    };
                    self.text = self.text[tkn_len..].to_string();
                    Some(tkn)
                },
                None => None
            };
            match &self.token {
                Some(tkn) => {
                    if tkn.token_type != TokenType::Whitespace {
                        break
                    }
                },
                None => break
            }
        }
        loop {
            self.next_token = self.get_next_token();
            match &self.next_token {
                Some(tkn) => {
                    if tkn.token_type != TokenType::Whitespace {
                        break
                    } else {
                        self.text = self.text[tkn.len()..].to_string();
                    }
                },
                None => break
            }
        }
    }
    fn get_next_token(&self) -> Option<Token>{
        let mut result = None;
        let regex = [
            (TokenType::NumericLiteral, Regex::new(r"^\d+(\.[\d]+)?").unwrap()),
            //any number of digits, and maybe a period and one or more digits.
            (TokenType::Whitespace, Regex::new("^ +").unwrap()),
            //matches any number of spaces
            (TokenType::StringLiteral, Regex::new(r#"^"[^"]*""#).unwrap()),
            //anything between two sets of double quotes, except double quotes
            (TokenType::StringLiteral, Regex::new(r#"^'[^']*'"#).unwrap()),
            //anything inside single quotes, except single quotes
            (TokenType::Symbol, Regex::new(r"^[+/*\-=^%]").unwrap())
            //only + / * - = ^ %
        ];
        for (key, rgx) in &regex {
            if rgx.is_match(&self.text) {
                let mut val = rgx.captures(&self.text).unwrap()[0].to_string();
                if let TokenType::StringLiteral = key {
                    val = val[1..{val.len()-1}].to_string();
                }
                let length = val.len().clone();
                result = Some(Token::new(*key,match key {
                        TokenType::StringLiteral => val[1..val.len()-1].to_string(),
                        _ => val
                    },length
                ));
                break;
            }
        }
        result
    }
}


