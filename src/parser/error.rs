use thiserror::Error;
use derive_more::Display;



#[derive(Debug, Display, Clone)]
pub enum ParserErrType<'a> {
    UnexpectedToken(&'a str),
    UnexpectedChar(char),
}

#[derive(Debug, Display, Clone, Error)]
#[display("[line {line}] {err_type}: {message}")]
pub struct ParserErr<'a> {
    pub err_type: ParserErrType<'a>,
    pub line: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct ParserErrHandler<'a> {
    errors: Vec<ParserErr<'a>>,
}

impl<'a> ParserErrHandler<'a> {
    pub fn new() -> Self {
        Self {
            errors: vec![],
        }
    }

    pub fn is_empty(&self) -> bool { self.errors.is_empty() }
    pub fn error_count(&self) -> usize { self.errors.len() }

    pub fn report(&mut self, e: ParserErr<'a>) {
        self.errors.push(e);
    }

    pub fn dump_errors(&mut self) -> Vec<ParserErr> {
        let errs = self.errors.clone();
        self.errors = vec![];
        errs
    }

    pub fn into_errors(self) -> Vec<ParserErr<'a>> {
        self.errors
    }
}

pub type Result<'a, T> = std::result::Result<T, ParserErr<'a>>;
