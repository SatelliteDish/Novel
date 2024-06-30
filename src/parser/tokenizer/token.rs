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

    Whitespace, EOF, Invalid, Empty
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
            TokenType::Empty => "Empty".to_string(),
            TokenType::Whitespace => "Whitespace".to_string(),
            TokenType::Invalid => "Error".to_string(),
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", &self.to_string())
    }
}

#[derive(PartialEq,Clone,Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub val: LiteralValue<'a>,
    raw: &'a str,
    line: u32,
    start: usize
}

impl<'a> Token<'a> {

    pub fn new(tkn_type: TokenType, val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Token<'a> {
        Token {
            token_type: tkn_type,
            val,
            raw,
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

    //========================================
    //TODO: Make a macro for these constructors
    //=========================================
    
    pub fn empty() -> Self {
        Token {
            token_type: TokenType::Empty,
            val: LiteralValue::none(),
            raw: "",
            line: 0,
            start: 0
        }
    }

    pub fn new_whitespace(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Token<'a> {
        Token {
            token_type: TokenType::Whitespace,
            val,
            raw,
            line,
            start
        }
    }

    pub fn new_eof(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::EOF = &val {
            Ok(Token {
                token_type: TokenType::EOF,
                val,
                raw,
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
            raw: "\0",
            line: 0,
            start: 0
        }
    }

    pub fn new_identifier(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Identifer(_) = val {
            Ok(Token {
                token_type: TokenType::Identifier,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_string(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::String(_) = val {
            Ok(Token {
                token_type: TokenType::StringLiteral,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_comma(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "," {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Comma,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_dot(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "." {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Dot,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_bang(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "!" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Bang,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_question(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "?" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Question,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_interrobang(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "‽" || *sym != "?!" || *sym != "!?" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Interrobang,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_semicolon(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != ";" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Semicolon,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_colon(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != ":" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Colon,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_left_paren(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "(" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::LeftParen,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_right_paren(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != ")" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::RightParen,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_plus(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "+" {
                return Err(Error::new(
                        ErrorType::InvalidTokenValue,
                        line,
                        start
                    ))
                }
            Ok(Token {
                token_type: TokenType::Plus,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_minus(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "-" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Minus,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_star(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "*" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Star,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_slash(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "/" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Slash,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_mod(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "%" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Mod,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_ellipsis(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Symbol(sym) = &val {
            if *sym != "..." {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Ellipsis,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_if(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "If" || *key != "if" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::If,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_eq_to(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::EqTo,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_neq_to(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is not equal to" || *key != "isn't equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::NeqTo,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_or(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "or" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Or,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_not(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is not" || *key != "isn't" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Not,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_and(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "and" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::And,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_less_eq(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is less than or equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::LessEq,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_less(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is less than" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Less,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_greater_eq(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is greater than or equal to" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::GreaterEq,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_greater(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "is greater than" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Greater,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_therefore(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "; Therefore" || *key != "; therefore" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_false(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "false" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::False,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_true(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "true" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::True,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_none (val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "none" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::None,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_you(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "You" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::You,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_assignment(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "it is" ||
               *key != "he is" ||
               *key != "she is" ||
               *key != "they are" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_declaration(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "; Therefore" || *key != "; therefore" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
    
    pub fn new_id_keyword(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Keyword(key) = &val {
            if *key != "called" ||
               *key != "named" ||
               *key != "labelled" {
                return Err(Error::new(
                    ErrorType::InvalidTokenValue,
                    line,
                    start
                ))
            }
            Ok(Token {
                token_type: TokenType::Therefore,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }

    pub fn new_number(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
        if let LiteralValue::Number(_) = val {
            Ok(Token {
                token_type: TokenType::NumericLiteral,
                val,
                raw,
                line,
                start
            })
        } else {
            Err(Error::new(ErrorType::InvalidTokenValue, line, start))
        }
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",&self.to_string())
    }
}

impl std::fmt::Debug for Token<'_> {
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
