use regex::Regex;

pub enum TokenType {
    NumericLiteral,
    StringLiteral,
    Whitespace
}
impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::NumericLiteral => "NumericLiteral".to_string(),
            TokenType::StringLiteral => "StringLiteral".to_string(),
            TokenType::Whitespace => "Whitespace".to_string()
        }
    }
}
pub struct Token {
    pub token_type: TokenType,
    pub val: String
}
impl Token {
    pub fn to_string(&self) -> String {
        format!("
            '{}': {{
                val: {}
            }}
        ",self.token_type.to_string(),self.val)
    }
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Tokenizer {
    text: String,
    pointer: usize,
    pub next_token: Option<String>
}
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        Tokenizer {
            text: String::from(text),
            pointer: 0,
            next_token: None
        }
    }
    pub fn get_next_token(&mut self) -> Option<Token> {
        let mut result: Option<Token> = None;
        let regex = [
            (TokenType::NumericLiteral, Regex::new(r"^\d+(\.[\d]+)?").unwrap()),
            (TokenType::Whitespace, Regex::new("^ +").unwrap()),
            (TokenType::StringLiteral, Regex::new(r#"^"[^"\n]*""#).unwrap()),
            (TokenType::StringLiteral, Regex::new(r#"^'[^'\n]*'"#).unwrap())
        ];
        for (key, regex) in regex {
            if regex.is_match(&self.text) {
                let mut val = regex.captures(&self.text).unwrap()[0].to_string();
                self.text = String::from(&self.text[{val.len()}..]);
                if let TokenType::StringLiteral = key {
                    val = val[1..{val.len()-1}].to_string();
                }
                result = Some(Token{
                    token_type: key,
                    val: val,
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
