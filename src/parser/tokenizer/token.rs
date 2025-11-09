use core::fmt;
use super::{LiteralValue,NvlError,ErrorType};
use std::error;


macro_rules! token_constructor {
    (Symbol, $type: tt, $sym: expr, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, line: u32, start: usize, length: usize) -> Result<Token<'a>,Box<dyn error::Error>> {
            if let LiteralValue::Symbol(sym) = &val {
                if *sym != $sym {
                    return Err(Box::new(
                        NvlError::new(
                            ErrorType::InvalidTokenValue,
                            &line,
                            &start,
                            "Error".to_string()
                        )
                    ))
                }
                Ok(Token {
                    token_type: TokenType::$type,
                    val,
                    line,
                    start,
                    length
                })
            } else {
                Err(
                    Box::new(
                        NvlError::new(
                            ErrorType::InvalidTokenValue,
                            &line,
                            &start,
                            "Error".to_string()
                        )
                    )
                )
            }
        }
    };

    (Keyword, $type: tt, $name: ident, $($lit: literal);+) => {
        pub fn $name(val: LiteralValue<'a>, line: u32, start: usize, length: usize) -> Result<Token<'a>,Box<dyn error::Error>> {
            if let LiteralValue::Keyword(key) = &val {
                if $(*key != $lit)||+ {
                    return Err(Box::new(
                            NvlError::new(
                                ErrorType::InvalidTokenValue,
                                &line,
                                &start,
                                "Error".to_string()
                            )
                    ))
                }
                Ok(Token {
                    token_type: TokenType::$type,
                    val,
                    line,
                    start,
                    length
                })
            } else {
                Err(Box::new(
                    NvlError::new(
                        ErrorType::InvalidTokenValue,
                        &line,
                        &start,
                        "Error".to_string()
                    )
                ))
            }
        }
    };

    ($lit_type: tt, $type: tt, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, line: u32, start: usize, length: usize) -> Result<Token<'a>,Box<dyn error::Error>> {
            if let LiteralValue::$lit_type(_) = &val {
                Ok(Token {
                    token_type: TokenType::$type,
                    val,
                    line,
                    start,
                    length
                })
            } else {
                Err(Box::new(
                    NvlError::new(
                        ErrorType::InvalidTokenValue,
                        &line,
                        &start,
                        "Error".to_string()
                    )  
                ))
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

    Whitespace, Eof, Invalid, Empty, NewLine
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
            TokenType::NewLine => "New Line".to_string(),
        })
    }
}

pub struct TokenBuilder<'a> {
    token_type: Option<TokenType>,
    val: Option<LiteralValue<'a>>,
    line: Option<u32>,
    start: Option<usize>,
    length: Option<usize>,
}

impl<'a> TokenBuilder<'a> {
    pub fn new() -> Self {
        TokenBuilder {
            token_type: None,
            val: None,
            line: None,
            start: None,
            length: None,
        }
    }

    pub fn with_token_type(mut self, token_type: TokenType) -> Self {
        self.token_type = Some(token_type);
        self
    }

    pub fn with_value(mut self, value: LiteralValue<'a> ) -> Self {
        self.val = Some(value);
        self
    }

    pub fn with_line(mut self, line: u32 ) -> Self {
        self.line = Some(line);
        self
    }

    pub fn with_start(mut self, start: usize ) -> Self {
        self.start = Some(start);
        self
    }

    pub fn with_length(mut self, length: usize ) -> Self {
        self.length = Some(length);
        self
    }

    pub fn build(self) -> Result<Token<'a>, Box::<dyn error::Error>> {
        let token_type = self.token_type.ok_or("Tried to build token with unset type!")?;
        let val = self.val.ok_or("Tried to build token with unset value!")?;
        let line = self.line.ok_or("Tried to build token with unset line!")?;
        let start = self.start.ok_or("Tried to build token with unset start position!")?;
        let length = self.length.ok_or("Tried to build token with unset length!")?;
        
        Ok( Token {
            token_type,
            val,
            line,
            start,
            length,
        } )
    }
}

#[derive(PartialEq,Clone,Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub val: LiteralValue<'a>,
    line: u32,
    start: usize,
    length: usize,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
            \"{}\": {{
                \"val\": {},
                \"start\": {},
                \"line\": {},
                \"len\": {}
            }}
        ", &self.token_type, &self.val, &self.start, &self.line, &self.len())
    }
}

#[allow(dead_code)]
impl<'a> Token<'a> {
    
    pub fn begin() -> TokenBuilder<'a> {
        TokenBuilder::new()
    }

    pub fn len(&self) -> &usize {
        &self.length
    }
    
    pub fn line(&self) -> &u32 {
        &self.line
    }

    pub fn end(&self) -> usize {
        self.start + self.length
    }

    pub fn start(&self) -> &usize {
        &self.start
    }

    pub fn new_eof(val: LiteralValue<'a>, line: u32, start: usize, length: usize) -> Result<Token<'a>,Box<dyn error::Error>> {
        if let LiteralValue::Eof = &val {
            Ok( Token{
                token_type: TokenType::Eof,
                val,
                line,
                start,
                length
            })
        } else {
            Err( Box::new(
                NvlError::new(
                    ErrorType::InvalidTokenValue,
                    &line,
                    &start,
                    "Error".to_string()
                ))
            )
        }
    }

    pub fn invalid() -> Self {
        Token {
            token_type: TokenType::Invalid,
            val: LiteralValue::none(),
            line: 0,
            start: 0,
            length: 0
        }
    }


    token_constructor!(Identifier,     Identifier, new_identifier);
    //                 LiteralType     TokenType    Function Name 
    token_constructor!(    String,  StringLiteral,     new_string); 
    token_constructor!(    Number, NumericLiteral,     new_number);
    token_constructor!(   NewLine,        NewLine,       new_line);
 
    //              LiteralType TokenType  Char      Function Name
    token_constructor!(Symbol,      Comma,  ",",       new_comma);
    token_constructor!(Symbol,        Dot,  ".",         new_dot);
    token_constructor!(Symbol,       Bang,  "!",        new_bang);
    token_constructor!(Symbol,   Question,  "?",    new_question);
    token_constructor!(Symbol,  Semicolon,  ";",   new_semicolon); 
    token_constructor!(Symbol,      Colon,  ":",       new_colon); 
    token_constructor!(Symbol,  LeftParen,  "(",  new_left_paren);
    token_constructor!(Symbol, RightParen,  ")", new_right_paren); 
    token_constructor!(Symbol,       Plus,  "+",        new_plus);
    token_constructor!(Symbol,      Minus,  "-",       new_minus);
    token_constructor!(Symbol,      Star,   "*",        new_star);
    token_constructor!(Symbol,     Slash,   "/",       new_slash);
    token_constructor!(Symbol,       Mod,   "%",         new_mod);
    token_constructor!(Symbol,  Ellipsis, "...",    new_ellipsis);

    //                LiteralType  TokenType   Function Name   Match(es)
    token_constructor!(Keyword,         And,         new_and, "and"); 
    token_constructor!(Keyword,          Or,          new_or, "or"); 
    token_constructor!(Keyword,        EqTo,       new_eq_to, "is equal to"); 
    token_constructor!(Keyword,        Less,        new_less, "is less than");
    token_constructor!(Keyword,      LessEq,     new_less_eq, "is less than or equal to"); 
    token_constructor!(Keyword,     Greater,     new_greater, "is greater than");
    token_constructor!(Keyword,   GreaterEq,  new_greater_eq, "is greater than or equal to");
    token_constructor!(Keyword,        True,        new_true, "true");
    token_constructor!(Keyword,       False,       new_false, "false");
    token_constructor!(Keyword,        None,        new_none, "none");
    token_constructor!(Keyword,         You,         new_you, "You"); 
    token_constructor!(Keyword,         Not,         new_not, "is not";"isn't"); 
    token_constructor!(Keyword,       NeqTo,      new_neq_to, "is not equal to" ; "isn't equal to"); 
    token_constructor!(Keyword,          If,          new_if, "If" ; "if");
    token_constructor!(Keyword,   Therefore,   new_therefore, "Therefore" ; "therefore");
    token_constructor!(Keyword, Declaration, new_declaration, "; Therefore" ; "; therefore");
    token_constructor!(Keyword,   IdKeyword,  new_id_keyword, "called";"named";"labelled");
    token_constructor!(Keyword, Interrobang, new_interrobang, "!?";"?!";"‽");
    token_constructor!(Keyword,  Assignment,  new_assignment, "it is";"he is";"she is";"they are");

}

impl std::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
        \"{}\": {{
            \"val\": {},
            \"line\": {},
            \"start\": {},
            \"len\": {},
            \"end\": {}
        }}
    ",&self.token_type,&self.val,&self.line,&self.start,&self.len(),&self.end())
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
