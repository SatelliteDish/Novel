mod error;
mod token;
mod scanner;

use error::ParserErrHandler;

#[derive(Debug)]
pub struct Parser<'a> {
    text: &'a str,
    err_handler: ParserErrHandler<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            err_handler: ParserErrHandler::new(),
        }
    }

    pub fn parse(&self) {

    }
}
