use regex::Regex;

mod token;
pub use token::{Token,TokenType};

use super::error_handler::SyntaxError;

#[derive(PartialEq, Eq, Hash, Clone)]

pub struct Tokenizer {
    text: String,
    current: usize,
    line: usize,
    start: usize,
    pub token: Option<Token>,
    pub next_token: Option<Token>
}
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            text: String::from(text),
            current: 0,
            line: 0,
            start: 0,
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
                    self.current += tkn_len;
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
                        self.current += tkn.len();
                    }
                },
                None => break
            }
        }
        print!("Current: {}, Next: {}\n",match &self.token {
            Some(tkn) => tkn.to_string(),
            None => "None".to_string()
        },match &self.next_token {
            Some(tkn) => tkn.to_string(),
            None => "None".to_string()
        })
    }
    fn get_next_token(&mut self) -> Option<Token>{
        let mut result = None;
        let text = &self.text[self.current..];
        let regex = [
            (TokenType::NumericLiteral, Regex::new(r"^\d+(\.[\d]+)?").unwrap()),
            //any number of digits, and maybe a period and one or more digits.
            (TokenType::Whitespace, Regex::new(r"^[\s\n\t]+").unwrap()),
            //matches any number of spaces
            (TokenType::StringLiteral, Regex::new(r#"^"[^"]*""#).unwrap()),
            //anything between two sets of double quotes, except double quotes
            (TokenType::StringLiteral, Regex::new(r#"^'[^']*'"#).unwrap()),
            //anything inside single quotes, except single quotes
            (TokenType::Symbol, Regex::new(r"^[+/*\-=^%]").unwrap()),
            //only + / * - = ^ %
        ];
        for (key, rgx) in &regex {
            if rgx.is_match(text) {
                let mut val = rgx.captures(text).unwrap()[0].to_string();
                if let TokenType::StringLiteral = key {
                    val = val[1..{val.len()-1}].to_string();
                } else if let TokenType::Whitespace = key {
                    if val == "\n" {
                        self.line += 1;
                        self.start = self.current;
                    }
                }
                result = Some(Token::new(*key,match key {
                        TokenType::StringLiteral => &val[1..val.len()-1],
                        _ => &val
                    },&val,&self.line,&self.current
                ));
                break;
            }
        }
        result
    }
}


