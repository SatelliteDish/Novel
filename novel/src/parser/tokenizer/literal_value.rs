#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifer(String),
    Keyword(String),
    Symbol(String),
    EOF,
    None
}

impl LiteralValue {

    pub fn new_number(num: f64) -> Self {
        LiteralValue::Number(num)
    }

    pub fn new_string(str: &str) -> Self {
        LiteralValue::String(str.to_string())
    }

    pub fn new_bool(bool: &bool) -> Self {
        LiteralValue::Boolean(*bool)
    }

    pub fn new_identifier(id: &str) -> Self {
        LiteralValue::Identifer(id.to_string())
    }
    
    pub fn new_keyword(key: &str) -> Self {
        LiteralValue::Keyword(key.to_string())
    }

    pub fn new_symbol(sym: &str) -> Self {
        LiteralValue::Symbol(sym.to_string())
    }

    pub fn eof() -> Self {
        LiteralValue::EOF
    }

    pub fn none() -> Self {
        LiteralValue::None
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        if let (LiteralValue::Boolean(left), LiteralValue::Boolean(right)) =  (&self,other) {
            left == right
        } else if let (LiteralValue::Identifer(left), LiteralValue::Identifer(right)) =  (&self,other) {
            left == right
        } else if let (LiteralValue::Keyword(left), LiteralValue::Keyword(right)) =  (&self,other) {
            left == right
        } else if let (LiteralValue::None, LiteralValue::None) =  (&self,other) {
            true
        } else if let (LiteralValue::Number(left), LiteralValue::Number(right)) =  (&self,other) {
            left == right
        } else if let (LiteralValue::String(left), LiteralValue::String(right)) =  (&self,other) {
            left == right
        } else if let (LiteralValue::Symbol(left), LiteralValue::Symbol(right)) =  (&self,other) {
            left == right
        } else if let (LiteralValue::EOF, LiteralValue::EOF) =  (&self,other) {
            true
        } else {
            false
        }
    }
}

impl std::clone::Clone for LiteralValue{
    fn clone(&self) -> Self {
        match &self {
            LiteralValue::Boolean(bool) => LiteralValue::Boolean(*bool),
            LiteralValue::Identifer(id) => LiteralValue::Identifer(id.to_string()),
            LiteralValue::Keyword(key) => LiteralValue::Keyword(key.to_string()),
            LiteralValue::None => LiteralValue::None,
            LiteralValue::Number(num) => LiteralValue::Number(num.clone()),
            LiteralValue::String(str) => LiteralValue::String(str.to_string()),
            LiteralValue::Symbol(sym) => LiteralValue::Symbol(sym.to_string()),
            LiteralValue::EOF => LiteralValue::EOF,
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",
        match &self {
            LiteralValue::Number(num) => format!("Number({})",num),
            LiteralValue::String(str) => format!("String({})",str),
            LiteralValue::Boolean(bool) => format!("Bool({})",bool),
            LiteralValue::Identifer(id) => format!("Identifier({})",id),
            LiteralValue::Keyword(key) => format!("KeyWord({})",key),
            LiteralValue::Symbol(sym) => format!("Symbol({})",sym),
            LiteralValue::EOF => "EOF".to_string(),
            LiteralValue::None => "None".to_string()
            }
        )
    }
}
