use core::fmt;
use super::{LiteralValue,Error,ErrorType};


macro_rules! token_constructor {
    ($type: tt, $lit_type: tt, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
            if let LiteralValue::$lit_type(_) = &val {
                Ok(Token {
                    token_type: TokenType::$type,
                    val,
                    raw,
                    line,
                    start
                })
            } else {
                Err( Error::new( ErrorType::InvalidTokenValue, line, start ))
            }
        }
    }
}

macro_rules! symbol_token_constructor {
    ($type: tt, $sym: expr, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
            if let LiteralValue::Symbol(sym) = &val {
                if *sym != $sym {
                    return Err(Error::new(
                        ErrorType::InvalidTokenValue,
                        line,
                        start
                    ))
                }
                Ok(Token {
                    token_type: TokenType::$type,
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
}

macro_rules! dbl_keyword_token_constructor {
    ($type: tt, $sym1: expr, $sym2: expr, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
            if let LiteralValue::Keyword(key) = &val {
                if *key != $sym1 || *key != $sym2 {
                    return Err(Error::new(
                        ErrorType::InvalidTokenValue,
                        line,
                        start
                    ))
                }
                Ok(Token {
                    token_type: TokenType::$type,
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
}

macro_rules! sng_keyword_token_constructor {
    ($type: tt, $sym: expr, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, raw: &'a str, line: u32, start: usize) -> Result<Token<'a>,Error> {
            if let LiteralValue::Keyword(key) = &val {
                if *key != $sym {
                    return Err(Error::new(
                        ErrorType::InvalidTokenValue,
                        line,
                        start
                    ))
                }
                Ok(Token {
                    token_type: TokenType::$type,
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
}

#[allow(dead_code)]
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

    Whitespace, Eof, Invalid, Empty
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",match self {
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
            TokenType::Eof => "EOF".to_string(),
            TokenType::Empty => "Empty".to_string(),
            TokenType::Whitespace => "Whitespace".to_string(),
            TokenType::Invalid => "Error".to_string(),
        })
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

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
            \"{}\": {{
                \"val\": {},
                \"raw\": {},
                \"line\": {},
                \"start\": {}
            }}
        ", &self.token_type, &self.val, &self.raw, self.line, self.start)
    }
}

#[allow(dead_code)]
impl<'a> Token<'a> {

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
        if let LiteralValue::Eof = &val {
            Ok(Token {
                token_type: TokenType::Eof,
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

    token_constructor!(Identifier, Identifier, new_identifier);
    token_constructor!(StringLiteral, String, new_string); 
    token_constructor!(NumericLiteral, Number, new_number);
    symbol_token_constructor!(Comma, ",", new_comma);
    symbol_token_constructor!(Dot, ".", new_dot);
    symbol_token_constructor!(Bang, "!", new_bang);
    symbol_token_constructor!(Question, "?", new_question);
    symbol_token_constructor!(Semicolon, ";", new_semicolon); 
    symbol_token_constructor!(Colon, ":", new_colon); 
    symbol_token_constructor!(LeftParen, "(",new_left_paren);
    symbol_token_constructor!(RightParen, ")", new_right_paren); 
    symbol_token_constructor!(Plus, "+", new_plus);
    symbol_token_constructor!(Minus, "-", new_minus);
    symbol_token_constructor!(Star,"*", new_star);
    symbol_token_constructor!(Slash, "/", new_slash);
    symbol_token_constructor!(Mod, "%", new_mod);
    symbol_token_constructor!(Ellipsis, "...", new_ellipsis);
    dbl_keyword_token_constructor!(If, "If", "if", new_if);
    sng_keyword_token_constructor!(EqTo, "is equal to", new_eq_to); 
    dbl_keyword_token_constructor!(NeqTo, "is not equal to", "isn't equal to", new_neq_to); 
    sng_keyword_token_constructor!(Or, "or", new_or); 
    dbl_keyword_token_constructor!(Not, "is not", "isn't", new_not); 
    sng_keyword_token_constructor!(And, "and", new_and); 
    sng_keyword_token_constructor!(LessEq, "is less than or equal to", new_less_eq); 
    sng_keyword_token_constructor!(Less, "is less than", new_less);
    sng_keyword_token_constructor!(GreaterEq, "is greater than or equal to", new_greater_eq);
    sng_keyword_token_constructor!(Greater, "is greater than", new_greater);
    dbl_keyword_token_constructor!(Therefore, "Therefore", "therefore", new_therefore);
    sng_keyword_token_constructor!(False, "false", new_false);
    sng_keyword_token_constructor!(True, "true", new_true);
    sng_keyword_token_constructor!(None, "none", new_none);
    sng_keyword_token_constructor!(You, "You", new_you); 
    dbl_keyword_token_constructor!(Declaration, "; Therefore", "; therefore", new_declaration);

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
        assert_eq!(TokenType::Eof.to_string(), "EOF");
    }

}
