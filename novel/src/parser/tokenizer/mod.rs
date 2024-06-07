use regex::Regex;

mod token;
pub use token::{Token,TokenType};

mod literal_value;
pub use literal_value::LiteralValue;

use super::error_handler::{Error,ErrorType};

#[derive(PartialEq)]

pub struct Tokenizer {
    text: String,
    current: usize,
    line: u32,
    start: usize,
    token: Result<Token,Error>,
    next_token: Result<Token,Error>
}
impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            text: String::from(text),
            current: 0,
            line: 0,
            start: 0,
            next_token: Ok(Token::new_eof(LiteralValue::eof(),"\0",&0,&0).unwrap()),
            token: Ok(Token::new_eof(LiteralValue::eof(),"\0",&0,&0).unwrap())
        };
        tokenizer.scan();
        tokenizer
    }
    //Returns next token and moves current position
    //Advances self.token and self.next_token
    pub fn scan(&mut self) -> Result<Token,Error> {
        let result = match &self.token {
            Ok(tkn) => Ok(tkn.clone()),
            Err(e) => Err(Error {
                error_type: e.error_type.clone(),
                line: e.line,
                position: e.position
            })
        };
        self.token = self.get_next_token();
        if let Ok(tkn) = &self.token {
            self.increment(tkn.len());
        }
        self.next_token = self.get_next_token();
        result
    }

    //Returns a reference to the current token
    pub fn peek(&self) -> &Result<Token,Error> {
        &self.token
    }

    //Returns a reference to the next token
    pub fn peek_next(&self) -> &Result<Token,Error> {
        &self.next_token
    }

    //returns t
    fn get_next_token(&mut self) -> Result<Token,Error> {
        let current = &self.text[self.current..];
        //match regex
        let mut result = Err(Error::new(
            ErrorType::MissingToken,
            &self.line,
            &self.current
        ));
        if let Some(cap) = Regex::new(
        /**********************************************\
         *                  if                         * 
        \**********************************************/
            r"^[iI]f"
        ).unwrap().captures(current) {
            result = Ok(Token::new_if(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *              therefore                      * 
        \**********************************************/
            r"^; [tT]herefore"
        ).unwrap().captures(current) {
            result = Ok(Token::new_therefore(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *               equal to                      * 
        \**********************************************/
            r"^is equal to"
        ).unwrap().captures(current) {
            result = Ok(Token::new_eq_to (
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *             not equal to                    * 
        \**********************************************/
            r"^(is not|isn't) equal to"
        ).unwrap().captures(current) {
            result = Ok(Token::new_neq_to(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                  or                         * 
        \**********************************************/
            r"^or"
        ).unwrap().captures(current) {
            result = Ok(Token::new_or(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                  not                        * 
        \**********************************************/
            r"^(is not|isn't)"
        ).unwrap().captures(current) {
            result = Ok(Token::new_not(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                  and                        * 
        \**********************************************/
            r"^and"
        ).unwrap().captures(current) {
            result = Ok(Token::new_and(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *            less or equal to                 * 
        \**********************************************/
            r"^is less than or equal to"
        ).unwrap().captures(current) {
            result = Ok(Token::new_less_eq( 
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *               less than                     * 
        \**********************************************/
            r"^is less than"
        ).unwrap().captures(current) {
            result = Ok(Token::new_less(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *           greater or equal                  * 
        \**********************************************/
            r"^is greater than or equal to"
        ).unwrap().captures(current) {
            result = Ok(Token::new_greater_eq(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                greater                      * 
        \**********************************************/
            r"^is greater than"
        ).unwrap().captures(current) {
            result = Ok(Token::new_greater(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                false                        * 
        \**********************************************/
            r"^false"
        ).unwrap().captures(current) {
            result = Ok(Token::new_false(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                 true                        * 
        \**********************************************/
            r"^true"
        ).unwrap().captures(current) {
            result = Ok(Token::new_true(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                 none                        * 
        \**********************************************/
            r"^none"
        ).unwrap().captures(current) {
            result = Ok(Token::new_none(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *                  you                        * 
        \**********************************************/
            r"^You"
        ).unwrap().captures(current) {
            result = Ok(Token::new_you(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *              assignment                     * 
        \**********************************************/
            r"^((it|he|she) is | they are)"
        ).unwrap().captures(current) {
            result = Ok(Token::new_assignment(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *              declaration                    * 
        \**********************************************/
            r"^[tT]here is a"
        ).unwrap().captures(current) {
            result = Ok(Token::new_declaration(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
        /**********************************************\
         *               id keyword                    * 
        \**********************************************/
            r"^(called|named|labelled)"
        ).unwrap().captures(current) {
            result = Ok(Token::new_id_keyword(
                LiteralValue::new_keyword(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^[A-Z]\w+"
        ).unwrap().captures(current) {
            result = Ok(Token::new_identifier(
                LiteralValue::new_identifier(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r#"^"[^"]*""#
        ).unwrap().captures(current) {
            result = Ok(Token::new_string(
                LiteralValue::new_string(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r#"^'[^']*'"#
        ).unwrap().captures(current) {
            result = Ok(Token::new_string(
                LiteralValue::new_string(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^\d+(\.[\d]+)?"
        ).unwrap().captures(current) {
            result = Ok(Token::new_number(
                LiteralValue::new_number(&cap[0].parse::<f64>().expect("Non-number matched as numeric literal!!!")),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^,"
        ).unwrap().captures(current) {
                result = Ok(Token::new_comma(
                    LiteralValue::new_symbol(&cap[0]),
                    &cap[0],
                    &self.line,
                    &self.current
                ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^\."
        ).unwrap().captures(current) {
            result = Ok(Token::new_dot(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^!"
        ).unwrap().captures(current) {
            result = Ok(Token::new_bang(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(r"^\?").unwrap().captures(current) {
            result = Ok(Token::new_question(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^(‽|\?!|!\?)"
        ).unwrap().captures(current) {
            result = Ok(Token::new_interrobang(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^;"
        ).unwrap().captures(current) {
            result = Ok(Token::new_semicolon(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^:"
        ).unwrap().captures(current) {
            result = Ok(Token::new_colon(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^\("
        ).unwrap().captures(current) {
            result = Ok(Token::new_left_paren(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        }  else if let Some(cap) = Regex::new(
            r"^\)"
        ).unwrap().captures(current) {
            result = Ok(Token::new_right_paren(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^\+"
        ).unwrap().captures(current) {
            result = Ok(Token::new_plus(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^-"
        ).unwrap().captures(current) {
            result = Ok(Token::new_minus(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"\*"
        ).unwrap().captures(current) {
            result = Ok(Token::new_star(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^/"
        ).unwrap().captures(current) {
            result = Ok(Token::new_slash(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^\.\.\."
        ).unwrap().captures(current) {
            result = Ok(Token::new_ellipsis(
                LiteralValue::new_symbol(&cap[0]),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(cap) = Regex::new(
            r"^\z"
        ).unwrap().captures(current) {
            result = Ok(Token::new_eof(
                LiteralValue::eof(),
                &cap[0],
                &self.line,
                &self.current
            ).unwrap());
        } else if let Some(_) = Regex::new(
            r"^\s+"
        ).unwrap().captures(current) {
            self.increment(1);
            result = self.get_next_token()
        }

        result
    } 

    fn increment(&mut self, increase: usize) {
        self.current += increase
    }
}


