#[derive(Debug,PartialEq,Clone,Copy)]
pub enum LiteralValue<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Identifer(&'a str),
    Keyword(&'a str),
    Symbol(&'a str),
    EOF,
    None
}

impl<'a> LiteralValue<'a> {

    pub fn new_number(num: f64) -> Self {
        LiteralValue::Number(num)
    }

    pub fn new_string(str: &'a str) -> Self {
        LiteralValue::String(str)
    }

    pub fn new_bool(bool: &bool) -> Self {
        LiteralValue::Boolean(*bool)
    }

    pub fn new_identifier(id: &'a str) -> Self {
        LiteralValue::Identifer(id)
    }
    
    pub fn new_keyword(key: &'a str) -> Self {
        LiteralValue::Keyword(key)
    }

    pub fn new_symbol(sym: &'a str) -> Self {
        LiteralValue::Symbol(sym)
    }

    pub fn eof() -> Self {
        LiteralValue::EOF
    }

    pub fn none() -> Self {
        LiteralValue::None
    }
}

impl std::fmt::Display for LiteralValue<'_> {
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
