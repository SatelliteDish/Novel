use core::fmt;
use super::{LiteralValue,Error,ErrorType};

#[derive(PartialEq,Eq,Clone,Hash,Copy)]
pub enum TokenType {
    //Literals
    NumericLiteral, StringLiteral, Identifier,

    Comma, Dot, Bang, Question, Interrobang,
    
    Semicolon, Colon,

    //Parens
    LeftParen, RightParen,
    
    //Math
    Plus, Minus, Slash, Star, Mod,
    
    Ellipsis,

    //Control Flow
    If, Therefore, 
    
    //Comparison
    EqTo, NeqTo,Or, Not, And, Less, Greater, LessEq, GreaterEq,
    
    //Keywords
    False, True, None, You,
    Assignment, Declaration, IdKeyword,

    EOF, Invalid
}

impl TokenType {

    pub fn to_string(&self) -> String {
        match self {
            TokenType::NumericLiteral => "NumericLiteral".to_string(),
            TokenType::StringLiteral => "StringLiteral".to_string(),
            TokenType::Identifier => "Identifier".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::Bang => "!".to_string(),
            TokenType::Question => "?".to_string(),
            TokenType::Interrobang => "‽".to_string(),
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Star => "*".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::Mod => "%".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Ellipsis => "...".to_string(),
            TokenType::If => "If".to_string(),
            TokenType::Therefore => "Therefore".to_string(),
            TokenType::And => "And".to_string(),
            TokenType::Or => "Or".to_string(),
            TokenType::Not => "Not".to_string(),
            TokenType::EqTo => "==".to_string(),
            TokenType::NeqTo => "!=".to_string(),
            TokenType::Less => "<".to_string(),
            TokenType::LessEq => "<=".to_string(),
            TokenType::Greater => ">".to_string(),
            TokenType::GreaterEq => ">=".to_string(),
            TokenType::True => "True".to_string(),
            TokenType::False => "False".to_string(),
            TokenType::You => "You".to_string(),
            TokenType::None => "None".to_string(),
            TokenType::IdKeyword => "Id Keyword".to_string(),
            TokenType::Declaration => "Declaration".to_string(),
            TokenType::Assignment => "Assignment".to_string(),
            TokenType::EOF => "EOF".to_string(),
            TokenType::Invalid => "Error".to_string(),
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", &self.to_string())
    }
}

#[derive(PartialEq,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: LiteralValue,
    raw: String,
    line: u32,
    start: usize
}

impl Token {

    pub fn new(tkn_type: TokenType, val: LiteralValue, raw: &str, line: u32, start: usize) -> Token {
        Token {
            token_type: tkn_type,
            val,
            raw: String::from(raw),
            line,
            start
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
        ", &self.token_type, &self.val, &self.raw, self.line, self.start)
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn end(&self) -> usize {
        self.start + self.len()
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn start(&self) -> usize {
        self.start
    }
    
    pub fn new_eof(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::EOF = &val {
            Ok(Token {
                token_type: TokenType::EOF,
                val,
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err( Error::new( ErrorType::InvalidTokenValue, line, start ))
        }
    }
    
    pub fn invalid() -> Self {
        Token {
            token_type: TokenType::Invalid,
            val: LiteralValue::none(),
            raw: "\0".to_string(),
            line: 0,
            start: 0
        }
    }

    pub fn new_identifier(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Identifer(_) = val {
            Ok(Token {
                token_type: TokenType::Identifier,
                val,
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_string(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::String(_) = val {
            Ok(Token {
                token_type: TokenType::StringLiteral,
                val,
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_comma(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "," {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Comma,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_dot(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "." {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Dot,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_bang(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "!" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Bang,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_question(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "?" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Question,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_interrobang(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "‽" || sym != "?!" || sym != "!?" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Interrobang,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_semicolon(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != ";" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Semicolon,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_colon(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != ":" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Colon,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_left_paren(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "(" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::LeftParen,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_right_paren(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != ")" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::RightParen,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_plus(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "+" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                    ))
                    }
                    Ok(Token {
                        token_type: TokenType::Plus,
                    val: val.clone(),
                    raw: raw.to_string(),
                    line,
                    start
                }
            )
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_minus(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "-" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Minus,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_star(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "*" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Star,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_slash(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "/" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Slash,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_mod(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "%" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Mod,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_ellipsis(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if sym != "..." {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Ellipsis,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_if(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "If" || key != "if" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::If,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_eq_to(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::EqTo,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_neq_to(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is not equal to" || key != "isn't equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::NeqTo,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_or(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "or" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Or,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_not(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is not" || key != "isn't" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Not,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_and(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "and" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::And,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_less_eq(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is less than or equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::LessEq,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_less(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is less than" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Less,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_greater_eq(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is greater than or equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::GreaterEq,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_greater(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "is greater than" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Greater,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_therefore(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "; Therefore" || key != "; therefore" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_false(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "false" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::False,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_true(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "true" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::True,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_none (val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "none" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::None,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_you(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "You" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::You,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_assignment(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "it is" ||
               key != "he is" ||
               key != "she is" ||
               key != "they are" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_declaration(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "; Therefore" || key != "; therefore" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_id_keyword(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if key != "called" ||
               key != "named" ||
               key != "labelled" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val: val.clone(),
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_number(val: LiteralValue, raw: &str, line: u32, start: usize) -> Result<Self,Error> {
        if let LiteralValue::Number(_) = val {
            Ok(Token {
                token_type: TokenType::NumericLiteral,
                val,
                raw: raw.to_string(),
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
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
        assert_eq!(TokenType::NumericLiteral.to_string(), "NumericLiteral");
        assert_eq!(TokenType::StringLiteral.to_string(), "StringLiteral");
        assert_eq!(TokenType::NumericLiteral.to_string(),"NumericLiteral");
        assert_eq!(TokenType::StringLiteral.to_string(), "StringLiteral");
        assert_eq!(TokenType::Identifier.to_string(), "Identifier");
        assert_eq!(TokenType::Comma.to_string(), ",");
        assert_eq!(TokenType::Dot.to_string(), ".");
        assert_eq!(TokenType::Bang.to_string(), "!");
        assert_eq!(TokenType::Question.to_string(), "?");
        assert_eq!(TokenType::Interrobang.to_string(), "‽");
        assert_eq!(TokenType::LeftParen.to_string(), "(");
        assert_eq!(TokenType::RightParen.to_string(), ")");
        assert_eq!(TokenType::Plus.to_string(), "+");
        assert_eq!(TokenType::Minus.to_string(), "-");
        assert_eq!(TokenType::Star.to_string(), "*");
        assert_eq!(TokenType::Slash.to_string(), "/");
        assert_eq!(TokenType::Colon.to_string(), ":");
        assert_eq!(TokenType::Semicolon.to_string(), ";");
        assert_eq!(TokenType::Ellipsis.to_string(), "...");
        assert_eq!(TokenType::If.to_string(), "If");
        assert_eq!(TokenType::Therefore.to_string(), "Therefore");
        assert_eq!(TokenType::And.to_string(), "And");
        assert_eq!(TokenType::Or.to_string(), "Or");
        assert_eq!(TokenType::Not.to_string(), "Not");
        assert_eq!(TokenType::EqTo.to_string(), "==");
        assert_eq!(TokenType::NeqTo.to_string(), "!=");
        assert_eq!(TokenType::Less.to_string(), "<");
        assert_eq!(TokenType::LessEq.to_string(), "<=");
        assert_eq!(TokenType::Greater.to_string(), ">");
        assert_eq!(TokenType::GreaterEq.to_string(), ">=");
        assert_eq!(TokenType::True.to_string(), "True");
        assert_eq!(TokenType::False.to_string(), "False");
        assert_eq!(TokenType::You.to_string(), "You");
        assert_eq!(TokenType::None.to_string(), "None");
        assert_eq!(TokenType::IdKeyword.to_string(), "Id Keyword");
        assert_eq!(TokenType::Declaration.to_string(), "Declaration");
        assert_eq!(TokenType::Assignment.to_string(), "Assignment");
        assert_eq!(TokenType::EOF.to_string(), "EOF");
    }

}
