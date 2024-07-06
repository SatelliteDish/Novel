use regex::Regex;

mod token;
pub use token::{Token,TokenType};

mod literal_value;
pub use literal_value::LiteralValue;

use super::error_handler::{Error,ErrorType};

#[allow(dead_code)]
pub struct Tokenizer<'a> {
    text: &'a str,
    current: usize,
    line: u32,
    start: usize,
    token: Result<Token<'a>,Error>,
    lines: Vec<u32>,
}
impl<'a> Tokenizer<'a> {

    pub fn new(text: &'a str) -> Tokenizer<'a> {
        let tkn: Token<'a> = get_first_token(text,0,0);
        Tokenizer {
            text,
            current: tkn.len(),
            line: 1,
            start: 0,
            token: match tkn.token_type {
                TokenType::Invalid => {
                    Err(Error::new(
                        ErrorType::MissingToken,
                        0,
                        0
                    ))
                },
                _ => Ok(tkn)
            },
            lines: Vec::new(),
        }
    }

    /********************************************
    *Returns next token and moves current position
    *Advances self.token
    *********************************************/
    pub fn scan(&mut self) -> Result<Token<'a>,Error> {
        let result: Result<Token<'a>,Error>;
        (result, self.token) = (self.token, self.get_next_token(),);
        result
    }

    //Returns a reference to the current token
    pub fn peek(&self) -> &Result<Token,Error> {
        &self.token
    }
 
    fn increment(&mut self, increase: usize) {
        self.current += increase
    }
    fn get_next_token(&mut self) -> Result<Token<'a>,Error> {
        let cl = |func: Box<dyn FnOnce(LiteralValue, &'a str, u32, usize) -> Result<Token<'a>, Error>>, lit: LiteralValue, raw: &'a str| -> Result<Token<'a>, Error> {
            func(lit, raw, self.line, self.current)
        };
        let tkn = get_first_token(
            &self.text[self.current..],
            Box::new(cl)
        );
        match tkn.token_type {
            TokenType::Invalid => { Err(Error::new(
                    ErrorType::MissingToken,
                    self.line,
                    self.current
                ))
            },
            TokenType::Whitespace => {
                self.increment(tkn.len());
                self.get_next_token()
            },
            _ => {
                self.increment(tkn.len());
                Ok(tkn)
            }
        }
    }
}

fn get_first_token<'a>(
    text: &'a str,
    func: Box<dyn FnOnce(
        Box<dyn FnOnce(
            LiteralValue,
            &'a str,
            u32,
            usize
        ) -> Result<Token<'a>, Error>>, LiteralValue, &str
    ) -> Result<Token<'a>,Error>>) -> Result<Token, Error> {
    //match regex
    let mut result = Token::invalid();
    if let Some(cap) = Regex::new(

    /**********************************************\
    *                   if                         * 
    \**********************************************/
        r"^[iI]f"
    ).unwrap().find(text) {
        result =
            func(//TODO make Token::new that takes in a TokenType, raw, line, and position and then
                //use that in the closure instead
                Box::new(Token::new_if),
                LiteralValue::new_keyword(&text[..cap.len()]),
                &text[cap.len()..],
            )?;
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *               therefore                      * 
    \**********************************************/
        r"^; [tT]herefore"
    ).unwrap().find(text) {
        result = Token::new_therefore(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                equal to                      * 
    \**********************************************/
        r"^is equal to"
    ).unwrap().find(text) {
        result = Token::new_eq_to (
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *              not equal to                    * 
    \**********************************************/
        r"^(is not|isn't) equal to"
    ).unwrap().find(text) {
        result = Token::new_neq_to(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                   or                         * 
    \**********************************************/
        r"^or"
    ).unwrap().find(text) {
        result = Token::new_or(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                   not                        * 
    \**********************************************/
        r"^(is not|isn't)"
    ).unwrap().find(text) {
        result = Token::new_not(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                   and                        * 
    \**********************************************/
        r"^and"
    ).unwrap().find(text) {
        result = Token::new_and(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *             less or equal to                 * 
    \**********************************************/
        r"^is less than or equal to"
    ).unwrap().find(text) {
        result = Token::new_less_eq( 
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                less than                     * 
    \**********************************************/
        r"^is less than"
    ).unwrap().find(text) {
        result = Token::new_less(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *            greater or equal                  * 
    \**********************************************/
        r"^is greater than or equal to"
    ).unwrap().find(text) {
        result = Token::new_greater_eq(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                 greater                      * 
    \**********************************************/
        r"^is greater than"
    ).unwrap().find(text) {
        result = Token::new_greater(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
    
    /**********************************************\
    *                 false                        * 
    \**********************************************/
        r"^false"
    ).unwrap().find(text) {
        result = Token::new_false(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                  true                        * 
    \**********************************************/
        r"^true"
    ).unwrap().find(text) {
        result = Token::new_true(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                  none                        * 
    \**********************************************/
        r"^none"
    ).unwrap().find(text) {
        result = Token::new_none(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                   you                        * 
    \**********************************************/
        r"^You"
    ).unwrap().find(text) {
        result = Token::new_you(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *               assignment                     * 
    \**********************************************/
        r"^((it|he|she) is | they are)"
    ).unwrap().find(text) {
        result = Token::new_assignment(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *               declaration                    * 
    \**********************************************/
        r"^[tT]here is a"
    ).unwrap().find(text) {
        result = Token::new_declaration(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                id keyword                    * 
    \**********************************************/
        r"^(called|named|labelled)"
    ).unwrap().find(text) {
        result = Token::new_id_keyword(
            LiteralValue::new_keyword(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
        
    /**********************************************\
    *                identifier                    * 
    \**********************************************/
        r"^[A-Z]\w+"
    ).unwrap().find(text) {
        result = Token::new_identifier(
            LiteralValue::new_identifier(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                  string                      * 
    \**********************************************/
        r#"^"[^"]*""#
    ).unwrap().find(text) {
        result = Token::new_string(
            LiteralValue::new_string(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    
    } else if let Some(cap) = Regex::new(
        r#"^'[^']*'"#
    ).unwrap().find(text) {
        result = Token::new_string(
            LiteralValue::new_string(&text[..cap.len()]),
                &text[..cap.len()],
                line,
                pos
            ).unwrap();
        } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                  number                      * 
    \**********************************************/
        r"^\d+(\.[\d]+)?"
    ).unwrap().find(text) {
        result = Token::new_number(
            LiteralValue::new_number(cap.as_str().parse::<f64>().expect("Non-number matched as numeric literal!!!")),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
     /**********************************************\
    *                   comma                      * 
    \**********************************************/
        r"^,"
    ).unwrap().find(text) {
            result = Token::new_comma(
                LiteralValue::new_symbol(&text[..cap.len()]),
                &text[..cap.len()],
                line,
                pos
            ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                    dot                       * 
    \**********************************************/
        r"^\."
    ).unwrap().find(text) {
        result = Token::new_dot(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                   bang                       * 
    \**********************************************/
        r"^!"
    ).unwrap().find(text) {
        result = Token::new_bang(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                 question                     * 
    \**********************************************/
        r"^\?"
    ).unwrap().find(text) {
        result = Token::new_question(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                 interrobang                  * 
    \**********************************************/
        r"^(‽|\?!|!\?)"
    ).unwrap().find(text) {
        result = Token::new_interrobang(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                semicolon                     * 
    \**********************************************/
        r"^;"
    ).unwrap().find(text) {
        result = Token::new_semicolon(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                    colon                     * 
    \**********************************************/
        r"^:"
    ).unwrap().find(text) {
        result = Token::new_colon(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                left paren                    * 
    \**********************************************/
        r"^\("
    ).unwrap().find(text) {
        result = Token::new_left_paren(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    }  else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                 right paren                  * 
    \**********************************************/
        r"^\)"
    ).unwrap().find(text) {
        result = Token::new_right_paren(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                    plus                      * 
    \**********************************************/
        r"^\+"
    ).unwrap().find(text) {
        result = Token::new_plus(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                   minus                      * 
    \**********************************************/
        r"^-"
    ).unwrap().find(text) {
        result = Token::new_minus(
        LiteralValue::new_symbol(&text[..cap.len()]),
        &text[..cap.len()],
        line,
        pos
    ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                    star                      * 
    \**********************************************/
        r"\*"
    ).unwrap().find(text) {
        result = Token::new_star(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                   slash                      * 
    \**********************************************/
        r"^/"
    ).unwrap().find(text) {
        result = Token::new_slash(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos 
        ).unwrap();
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                 ellipsis                     * 
    \**********************************************/
        r"^\.\.\."
    ).unwrap().find(text) {
        result = Token::new_ellipsis(
            LiteralValue::new_symbol(&text[..cap.len()]),
            &text[..cap.len()],
            line,
            pos
        ).unwrap();
    } else if Regex::new(
       
    /**********************************************\
    *                    eof                       * 
    \**********************************************/
        r"^\z"
    ).unwrap().is_match(text) {
        result = Token::new_eof(
            LiteralValue::eof(),
            "\0",
            line,
            pos
        ).unwrap();
     } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                  new line                    * 
    \**********************************************/
        r"^\s*\n"
    ).unwrap().find(text) {
        result = Token::new_whitespace(
            LiteralValue::none(),
            &text[..cap.len()],
            line,
            pos
        )
    } else if let Some(cap) = Regex::new(
       
    /**********************************************\
    *                whitespace                    * 
    \**********************************************/
        r"^\s+"
    ).unwrap().find(text) {
        result = Token::new_whitespace(
            LiteralValue::none(),
            &text[..cap.len()],
            line,
            pos
        )
    }

    result
} 


