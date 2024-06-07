pub struct ErrorHandler {
    errors: Vec<Error>
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler {
            errors: Vec::new()
        }
    }
    pub fn report(&mut self, error: Error) {
        self.errors.push(error);
    }
    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }
    pub fn throw_errors(&self) {
        for err in &self.errors {
            print!("\n{}",err.to_string());
        }
        print!("\n");
        panic!();
    }
}

pub struct Error {
    pub error_type: ErrorType,
    pub line: u32,
    pub position: usize
}

impl Error {
    pub fn new(error_type: ErrorType,line: &u32, position: &usize) -> Self {
        Error {
            error_type,
            line: *line,
            position: *position
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}[{}:{}]",self.error_type
        .get_type(),self.line,self.position)
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        if *self.error_type.to_string() != *other.error_type.to_string() {
            return false
        }
        &self.line == &other.line && &self.position == &other.position
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self.to_string())
    }
}
#[derive(Clone)]
pub enum ErrorType {
    DivideByZero,
    InvalidOperands,
    NotImplemented,
    UnknownToken,
    MissingToken,
    InvalidTokenValue,
    UnexpectedToken,
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.get_type())
    }
}

impl std::fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self.get_type())
    }
}

impl ErrorType {
    fn get_type(&self) -> String {
        match &self {
            Self::DivideByZero => "Divide by Zero".to_string(),
            Self::InvalidOperands => "Invalid Operands".to_string(),
            Self::NotImplemented => "Not Implemented".to_string(),
            Self::UnknownToken => "Unknown Token".to_string(),
            Self::MissingToken => "Missing Token".to_string(),
            Self::InvalidTokenValue => "Invalid Token Value".to_string(),
            Self::UnexpectedToken => "Unexpected Token".to_string(),
        }
    }
}