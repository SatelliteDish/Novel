use core::fmt;
use std::fmt::write;

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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", &self.to_string())
    }
}

#[derive(PartialEq,Eq,Hash,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: String,
    raw: String,
    line: usize,
    start: usize
}
impl Token {
    pub fn new(tkn_type: TokenType, val: &str, raw: &str, line: &usize, start: &usize) -> Token {
        Token {
            token_type: tkn_type,
            val: String::from(val),
            raw: String::from(raw),
            line: *line,
            start: *start
        }
    }
    pub fn to_string(&self) -> String {
        format!("
            \"{}\": {{
                \"val\": {},
                \"raw\": {},
                \"line\": {},
                \"start\": {}
            }}
        ",&self.token_type,&self.val,&self.raw,&self.line,&self.start)
    }
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn end(&self) -> usize {
        self.start + self.len()
    }
    
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",&self.to_string())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
        \"{}\": {{
            \"val\": {},
            \"raw\": {},
            \"line\": {},
            \"start\": {},
            \"len\": {},
            \"end\": {}
        }}
    ",&self.token_type,&self.val,&self.raw,&self.line,&self.start,&self.len(),&self.end())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_type_to_string() {
        assert_eq!(TokenType::Identifier.to_string(), "Identifier");
        assert_eq!(TokenType::Keyword.to_string(), "Keyword");
        assert_eq!(TokenType::NumericLiteral.to_string(), "NumericLiteral");
        assert_eq!(TokenType::StringLiteral.to_string(), "StringLiteral");
        assert_eq!(TokenType::Symbol.to_string(), "Symbol");
        assert_eq!(TokenType::Whitespace.to_string(), "Whitespace");
    }
    fn len() {
        for index in 1..100 {
            //let tkn = Token::new()
        }
    }

}