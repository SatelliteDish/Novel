pub struct ErrorHandler {
    errors: Vec<NvlError>
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler {
            errors: Vec::new()
        }
    }

    pub fn report(&mut self,error: NvlError) {
        self.errors.push(error)
    }
    
    pub fn has_errors(&mut self) -> bool {
        !self.errors.is_empty()
    }

    pub fn throw_errors(&mut self) {
        for err in &self.errors {
            eprintln!("\n{}",err);
        }
        std::process::exit(1);
    }
}


#[derive(PartialEq, Clone)]
pub struct NvlError {
    pub error_type: ErrorType,
    pub line: u32,
    pub position: usize,
    pub message: Box<String>,
}

impl NvlError {
    pub fn new(error_type: ErrorType,line: &u32, position: &usize, message: String) -> Self {
        NvlError {
            error_type,
            line: *line,
            position: *position,
            message: Box::new(message)
        }
    }
}

impl std::fmt::Display for NvlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}[{}:{}] {}",
            &self.error_type.get_type(),
            &self.line,
            &self.position,
            &self.message
        )
    }

}

impl std::fmt::Debug for NvlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self.to_string())
    }
}

impl std::error::Error for NvlError {}

    #[allow(dead_code)]
#[derive(Clone,Copy,PartialEq)]
pub enum ErrorType {
    DivideByZero,
    InvalidOperands,
    NotImplemented,
    UnknownToken,
    MissingToken,
    InvalidTokenValue,
    InvalidTokenLength,
    UnexpectedToken,
    UnexpectedEof,
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
            Self::InvalidTokenLength => "Invalid Token Length".to_string(),
            Self::UnexpectedEof => "Unexpected End Of File".to_string(),
        }
    }
}
