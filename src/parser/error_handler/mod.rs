static mut ERRORS: Vec<Error> = Vec::new();


pub unsafe fn report(error: Error) {
    ERRORS.push(error)
}

pub unsafe fn has_errors() -> bool {
    ERRORS.len() > 0
}
pub unsafe fn throw_errors() {
    for err in ERRORS.clone() {
        eprintln!("\n{}",err.to_string());
    }
    std::process::exit(1);
}



#[derive(PartialEq,Clone,Copy)]
pub struct Error {
    pub error_type: ErrorType,
    pub line: u32,
    pub position: usize
}

impl Error {

    pub fn new(error_type: ErrorType,line: u32, position: usize) -> Self {
        Error {
            error_type,
            line,
            position
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}[{}:{}]",self.error_type
        .get_type(),self.line,self.position)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self.to_string())
    }
}

#[derive(Clone,Copy,PartialEq)]
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