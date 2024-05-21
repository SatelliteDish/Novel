use regex::Regex;
#[derive(PartialEq,Eq,Clone,Hash,Copy)]
pub enum TokenType {
    NumericLiteral,
    StringLiteral,
    Whitespace,
    Keyword,
    Identifier,
    Symbol
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::NumericLiteral => "NumericLiteral".to_string(),
            TokenType::StringLiteral => "StringLiteral".to_string(),
            TokenType::Whitespace => "Whitespace".to_string(),
            TokenType::Keyword => "Keyword".to_string(),
            TokenType::Identifier => "Identifier".to_string(),
            TokenType::Symbol => "Symbol".to_string()
        }
    }
}
#[derive(PartialEq,Eq,Hash,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: String,
    len: usize,
}
impl Token {
    pub fn to_string(&self) -> String {
        format!("
            '{}': {{
                val: {}
            }}
        ",self.token_type.to_string(),self.val)
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
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
        loop {
            self.token = match self.get_next_token() {
                Some(tkn) => {
                    self.text = self.text[tkn.len()..].to_string();
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
                result = Some(Token{
                    token_type: *key,
                    len: val.len(),
                    val: match key {
                        TokenType::StringLiteral => val[1..val.len()-1].to_string(),
                        _ => val
                    }
                });
                break;
            }
        }
        result
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
