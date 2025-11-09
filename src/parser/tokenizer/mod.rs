use regex::Regex;

mod token;
pub use token::{Token,TokenType};

use std::error;

mod literal_value;
pub use literal_value::LiteralValue;

use super::error_handler::{NvlError,ErrorType};

#[allow(dead_code)]
pub struct Tokenizer<'a> {
    text: &'a str,
    current: usize,
    line: u32,
    start: usize,
    token: Option<Token<'a>>,
    lines: Vec<u32>,
}
impl<'a> Tokenizer<'a> {

    pub fn new(text: &'a str) -> Tokenizer<'a> {
        let tkn: Token<'a> = get_first_token(text,0,0).unwrap();
        Tokenizer {
            text,
            current: *tkn.len(),
            line: 1,
            start: 0,
            token: match tkn.token_type {
                TokenType::Invalid => {
                    None
                },
                _ => Some(tkn)
            },
            lines: Vec::new(),
        }
    }

    /********************************************
    *Returns next token and moves current position
    *Advances self.token
    *********************************************/
    pub fn scan(&mut self) -> Result<Token<'a>,Box<dyn error::Error>> {
        let result;
        (result, self.token) = (self.token.ok_or(
                    Box::new(NvlError::new(
                        ErrorType::MissingToken,
                        &0,
                        &0,
                        "Error".to_string()
                    ))), Some(self.get_next_token()?));
        Ok(result?)
    }

    //Returns a reference to the current token
    pub fn peek(&self) -> &Option<Token> {
        &self.token
    }
 
    fn increment(&mut self, increase: usize) {
        self.current += increase
    }
    fn get_next_token(&mut self) -> Result<Token<'a>,Box<dyn error::Error>> {
        let tkn = get_first_token(
            &self.text[self.current..],
            self.line,
            self.current
        )?;
        match tkn.token_type {
            TokenType::Invalid => { Err(Box::new(NvlError::new(
                    ErrorType::MissingToken,
                    &self.line,
                    &self.current,
                    "Error".to_string()
                )))
            },
            TokenType::Whitespace => {
                self.increment(*tkn.len());
                self.get_next_token()
            },
            _ => {
                self.increment(*tkn.len());
                Ok(tkn)
            }
        }
    }
}

macro_rules! keyword_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::Keyword(text))
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! identifier_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::Identifier(text))
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! string_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::String(text))
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! number_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::Number(text.parse::<f64>()?))
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! symbol_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::Symbol(text))
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! eof_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::Eof)
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! newline_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::NewLine(text))
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}

macro_rules! whitespace_builder {
    ($token_type: tt) => (
     Box::new(| text: &'a str, line: u32, current: usize |  -> Result<Token<'a>, Box<dyn std::error::Error>> {
            Token::begin()
                .with_token_type(TokenType::$token_type)
                .with_value(LiteralValue::None)
                .with_line(line)
                .with_start(current)
                .with_length(text.len())
                .build()
        })
    )
}


type TokenBuilder<'a> = fn(&'a str, u32, usize) -> Result<Token<'a>, Box<dyn std::error::Error>>;

#[derive(std::fmt::Debug)]
struct TempError {
    message: String
}

impl std::fmt::Display for TempError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TempError { }

fn get_first_token<'a> (text: &'a str, line: u32, current: usize) -> Result<Token<'a>, Box<dyn std::error::Error>> {
    let patterns:
        Vec<(&str, Box<TokenBuilder<'a>> )> = vec![
            (r"^[iI]f", keyword_builder!(If)),
            (r"^; [tT]herefore", keyword_builder!(Therefore)),
            (r"^is equal to", keyword_builder!(EqTo)),
            (r"^(is not|isn't) equal to", keyword_builder!(NeqTo)),
            (r"^or", keyword_builder!(Or)),
            (r"^(is not|isn't)", keyword_builder!(Not)),
            (r"^and", keyword_builder!(And)),
            (r"^is less than or equal to", keyword_builder!(LessEq)),
            (r"^is less than", keyword_builder!(Less)),
            (r"^is greater than or equal to", keyword_builder!(GreaterEq)),
            (r"^is greater than", keyword_builder!(Greater)),
            (r"^false", keyword_builder!(False)),
            (r"^true", keyword_builder!(True)),
            (r"^none", keyword_builder!(None)),
            (r"^You", keyword_builder!(You)),
            (r"^((it|he|she) is | they are)", keyword_builder!(Assignment)),
            (r"^[tT]here is a", keyword_builder!(Declaration)),
            (r"^(called|named|labelled)", keyword_builder!(IdKeyword)),
            (r"^[A-Z]\w+", identifier_builder!(Identifier)),
            (r#"^"[^"]*""#, string_builder!(StringLiteral)),
            (r#"^'[^']*'"#, string_builder!(StringLiteral)),
            (r"^\d+(\.[\d]+)?", number_builder!(NumericLiteral)),
            (r"^,", symbol_builder!(Comma)),
            (r"^\.", symbol_builder!(Dot)),
            (r"^!", symbol_builder!(Bang)),
            (r"^\?", symbol_builder!(Question)),
            (r"^(‽|\?!|!\?)", symbol_builder!(Interrobang)),
            (r"^;", symbol_builder!(Semicolon)),
            (r"^:", symbol_builder!(Colon)),
            (r"^\(", symbol_builder!(LeftParen)),
            (r"^\)", symbol_builder!(RightParen)),
            (r"^\+", symbol_builder!(Plus)),
            (r"^-", symbol_builder!(Minus)),
            (r"\*", symbol_builder!(Star)),
            (r"^/", symbol_builder!(Slash)),
            (r"^\.\.\.", symbol_builder!(Ellipsis)),
            (r"^\z", eof_builder!(Eof)),
            (r"^\s*\n", newline_builder!(NewLine)),
            (r"^\s+", whitespace_builder!(None)),
    ];
    for (rgx, func) in patterns {
        if let Some(cap) = Regex::new(rgx).unwrap().find(text) {
            return Ok(func(&text[..cap.len()], line, current)?);
        }
    }
    Err(Box::new(TempError{ message: "This is temporary!".to_string() }))
    //let mut result = Ok(Token::invalid());
    //if let Some(cap) = Regex::new(

    // /**********************************************\
    //*                   if                         * 
    //\**********************************************/
    //    r"^[iI]f"
    //).unwrap().find(text) {
    //    result = Ok(Token::new_if(//TODO make Token::new that takes in a TokenType, raw, line, and position and then
    //            //use that in the closure instead
    //            // no make it a vec of (regex, () => Token) and iterate
    //            LiteralValue::new_keyword(&text[..cap.len()]),
    //            line,
    //            current,
    //            cap.len(),
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*               therefore                      * 
    //\**********************************************/
    //    r"^; [tT]herefore"
    //).unwrap().find(text) {
    //    result = Ok(Token::new_therefore(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                equal to                      * 
    //\**********************************************/
    //    r"^is equal to"
    //).unwrap().find(text) {
    //    result = Ok(Token::new_eq_to (
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?)
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*              not equal to                    * 
    //\**********************************************/
    //    r"^(is not|isn't) equal to"
    //).unwrap().find(text) {
    //    result = Ok(Token::new_neq_to(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                   or                         * 
    //\**********************************************/
    //    r"^or"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_or(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                   not                        * 
    //\**********************************************/
    //    r"^(is not|isn't)"
    //).unwrap().find(text) {
    //    result = Ok(Token::new_not(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                   and                        * 
    //\**********************************************/
    //    r"^and"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_and(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*             less or equal to                 * 
    //\**********************************************/
    //    r"^is less than or equal to"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_less_eq( 
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                less than                     * 
    //\**********************************************/
    //    r"^is less than"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_less(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*            greater or equal                  * 
    //\**********************************************/
    //    r"^is greater than or equal to"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_greater_eq(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                 greater                      * 
    //\**********************************************/
    //    r"^is greater than"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_greater(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
    
    // /**********************************************\
    //*                 false                        * 
    //\**********************************************/
    //    r"^false"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_false(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                  true                        * 
    //\**********************************************/
    //    r"^true"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_true(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                  none                        * 
    //\**********************************************/
    //    r"^none"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_none(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                   you                        * 
    //\**********************************************/
    //    r"^You"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_you(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*               assignment                     * 
    //\**********************************************/
    //    r"^((it|he|she) is | they are)"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_assignment(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*               declaration                    * 
    //\**********************************************/
    //    r"^[tT]here is a"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_declaration(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                id keyword                    * 
    //\**********************************************/
    //    r"^(called|named|labelled)"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_id_keyword(
    //        LiteralValue::new_keyword(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
        
    // /**********************************************\
    //*                identifier                    * 
    //\**********************************************/
    //    r"^[A-Z]\w+"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_identifier(
    //        LiteralValue::new_identifier(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                  string                      * 
    //\**********************************************/
    //    r#"^"[^"]*""#
    //).unwrap().find(text) {
    //    result = Ok( Token::new_string(
    //        LiteralValue::new_string(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    
    //} else if let Some(cap) = Regex::new(
    //    r#"^'[^']*'"#
    //).unwrap().find(text) {
    //    result = Ok( Token::new_string(
    //        LiteralValue::new_string(&text[..cap.len()]),
    //            line,
    //            current,
    //        cap.len()
    //    )?);
    //    } else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                  number                      * 
    //\**********************************************/
    //    r"^\d+(\.[\d]+)?"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_number(
    //        LiteralValue::new_number(cap.as_str().parse::<f64>().expect("Non-number matched as numeric literal!!!")),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                   comma                      * 
    //\**********************************************/
    //    r"^,"
    //).unwrap().find(text) {
    //        result = Ok( Token::new_comma(
    //            LiteralValue::new_symbol(&text[..cap.len()]),
    //            line,
    //            current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                    dot                       * 
    //\**********************************************/
    //    r"^\."
    //).unwrap().find(text) {
    //    result = Ok( Token::new_dot(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                   bang                       * 
    //\**********************************************/
    //    r"^!"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_bang(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                 question                     * 
    //\**********************************************/
    //    r"^\?"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_question(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                 interrobang                  * 
    //\**********************************************/
    //    r"^(‽|\?!|!\?)"
    //).unwrap().find(text) {
    //    result = Ok(Token::new_interrobang(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                semicolon                     * 
    //\**********************************************/
    //    r"^;"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_semicolon(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                    colon                     * 
    //\**********************************************/
    //    r"^:"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_colon(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                left paren                    * 
    //\**********************************************/
    //    r"^\("
    //).unwrap().find(text) {
    //    result = Ok( Token::new_left_paren(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //}  else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                 right paren                  * 
    //\**********************************************/
    //    r"^\)"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_right_paren(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                    plus                      * 
    //\**********************************************/
    //    r"^\+"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_plus(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                   minus                      * 
    //\**********************************************/
    //    r"^-"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_minus(
    //    LiteralValue::new_symbol(&text[..cap.len()]),
    //    line,
    //    current,
    //        cap.len()
    //)?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                    star                      * 
    //\**********************************************/
    //    r"\*"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_star(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                   slash                      * 
    //\**********************************************/
    //    r"^/"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_slash(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                 ellipsis                     * 
    //\**********************************************/
    //    r"^\.\.\."
    //).unwrap().find(text) {
    //    result = Ok( Token::new_ellipsis(
    //        LiteralValue::new_symbol(&text[..cap.len()]),
    //        line,
    //        current,
    //        cap.len()
    //    )?);
    //} else if Regex::new(
       
    // /**********************************************\
    //*                    eof                       * 
    //\**********************************************/
    //    r"^\z"
    //).unwrap().is_match(text) {
    //    result = Ok( Token::new_eof(
    //        LiteralValue::eof(),
    //        line,
    //        current,
    //        text.len()
    //    )?);
    // } else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                  new line                    * 
    //\**********************************************/
    //    r"^\s*\n"
    //).unwrap().find(text) {
    //    result = Ok( Token::new_line(
    //        LiteralValue::none(),
    //        line,
    //        current,
    //        cap.len()
    //        )?)
    //} else if let Some(cap) = Regex::new(
       
    // /**********************************************\
    //*                whitespace                    * 
    //\**********************************************/
    //    r"^\s+"
    //).unwrap().find(text) {
    //    result = get_first_token(&text[cap.len()-1..], line, current);
    //}

    //result
} 


