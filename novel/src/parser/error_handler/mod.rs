pub struct ErrorHandler {
    has_error: bool,
    errors: Vec<Error>
}

pub struct Error {
    error_type: Box<dyn NVLError>,
    line: u32,
    position: u32
}

impl Error {
    pub fn new<T: NVLError + 'static>(error_type: T,line: u32, position: u32) -> Self {
        Error {
            error_type: Box::new(error_type),
            line,
            position
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}[{}:{}]",self.error_type
        .get_type(),self.line,self.position)
    }
}

pub trait NVLError: std::fmt::Debug + std::fmt::Display {
    fn get_type(&self) -> String;
}

pub enum MathError {
    DivideByZero,
    InvalidOperands,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.get_type())
    }
}

impl std::fmt::Debug for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self.get_type())
    }
}

impl NVLError for MathError {
    fn get_type(&self) -> String {
        match &self {
            Self::DivideByZero => "Divide by Zero".to_string(),
            Self::InvalidOperands => "Invalid Operands".to_string(),
        }
    }
}
pub enum SyntaxError {
    NotImplemented,
    UnknownToken
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.get_type())
    }
}

impl std::fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",&self.get_type())
    }
}

impl NVLError for SyntaxError {
    fn get_type(&self) -> String {
        match &self {
            Self::NotImplemented => "Not Implemented".to_string(),
            Self::UnknownToken => "Unknown Token".to_string(),
        }
    }
}