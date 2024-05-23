use std::fmt::write;

use super::syntax_errors::SyntaxError;

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifer(String),
    Keyword(String),
    None
}
impl LiteralValue {
    pub fn new_number(num: f64) -> Self {
        LiteralValue::Number(num)
    }

    pub fn new_string(str: String) -> Self {
        LiteralValue::String(str)
    }

    pub fn new_bool(bool: bool) -> Self {
        LiteralValue::Boolean(bool)
    }

    pub fn new_identifier(id: String) -> Self {
        LiteralValue::Identifer(id)
    }
    
    pub fn new_keyword(key: String) -> Self {
        LiteralValue::Keyword(key)
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
            LiteralValue::String(str) => LiteralValue::String(str.to_string())
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
            LiteralValue::None => "None".to_string()
            }
        )
    }
}

pub enum TreeNode {
    Addition{left: Box<TreeNode>,right: Box<TreeNode>},
    Subtraction{left: Box<TreeNode>,right: Box<TreeNode>},
    Multiplication{left: Box<TreeNode>,right: Box<TreeNode>},
    Division{left: Box<TreeNode>,right: Box<TreeNode>},
    Negation(Box<TreeNode>),
    NumericLiteral(LiteralValue),
    Identifier(LiteralValue),
    StringLiteral(LiteralValue),
    Keyword(LiteralValue),
    Empty
}
impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match &self {
            TreeNode::Addition{left,right} |
            TreeNode::Subtraction{left,right} |
            TreeNode::Multiplication{left,right} |
            TreeNode::Division{left,right} => {
                format!("
                    \"{}\": {{
                        \"left\": {{
                            {}
                        }},
                        \"right\": {{
                            {}
                        }}
                    }}
                ",self.get_type(),left,right)
            },
            TreeNode::Negation(arg) => {
                format!("
                    \"-\" : {{
                        \"arg\": {{
                            {}
                        }} 
                    }}
                ",arg.to_string())
            },
            TreeNode::NumericLiteral(val) |
            TreeNode::Identifier(val) |
            TreeNode::Keyword(val) |
            TreeNode::StringLiteral(val) => val.to_string(),
            TreeNode::Empty => "null".to_string()
        })
    }
}
impl TreeNode {
    pub fn new_addition(left: Self, right: Self) -> Self {
        TreeNode::Addition {
            left:Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_subtraction(left: Self, right: Self) -> Self {
        TreeNode::Subtraction{
            left: Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_multiplication(left: Self, right: Self) -> Self {
        TreeNode::Multiplication{
            left: Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_division(left: Self, right: Self) -> Self {
        TreeNode::Division {
            left: Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_number(num: f64) -> Self {
        TreeNode::NumericLiteral(LiteralValue::Number(num))
    }

    pub fn new_string(str: String) -> Self {
        TreeNode::StringLiteral(LiteralValue::String(str))
    }

    pub fn get_type(&self) -> String {
        match &self {
            TreeNode::Addition{..} => "Addition".to_string(),
            TreeNode::Subtraction{..} => "Subtraction".to_string(),
            TreeNode::Multiplication{..} => "Multiplication".to_string(),
            TreeNode::Division{..} => "Division".to_string(),
            TreeNode::Negation(_) => "Negation".to_string(),
            TreeNode::NumericLiteral(_) => "NumericLiteral".to_string(),
            TreeNode::Identifier(_) => "Identifer".to_string(),
            TreeNode::Keyword(_) => "Keyword".to_string(),
            TreeNode::StringLiteral(_) => "StringLiteral".to_string(),
            TreeNode::Empty => "Empty".to_string()
        }
    }
    pub fn to_string(&self) -> String {
        match &self {
            TreeNode::Addition{left,right} |
            TreeNode::Subtraction{left,right} |
            TreeNode::Multiplication{left,right} |
            TreeNode::Division{left,right} => {
                let symbol = match &self {
                    TreeNode::Addition{..} => "+",
                    TreeNode::Subtraction{..} => "-",
                    TreeNode::Multiplication{..} => "*",
                    TreeNode::Division{..} => "/",
                    _ => "Error"
                };
                format!("
                    '{}': {{
                        'left': {{
                            {}
                        }},
                        'right': {{
                            {}
                        }}
                    }}
                ",symbol,left.to_string(),right.to_string())
            },
            TreeNode::Negation(arg) => {
                format!("
                    '-' : {{
                        'arg': {{
                            {}
                        }} 
                    }}
                ",arg.to_string())
            },
            TreeNode::NumericLiteral(val) => {
                val.to_string()
            },
            TreeNode::Identifier(val) |
            TreeNode::Keyword(val) |
            TreeNode::StringLiteral(val)=> val.to_string(),
            TreeNode::Empty => "null".to_string()
        }
    }
    pub fn eval(&self) -> Result<LiteralValue,SyntaxError> {
        match &self {   
            TreeNode::Addition{left,right} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        Ok(LiteralValue::new_number(left_num + right_num))
                    } else {
                        Err(SyntaxError::InvalidOperands(format!(
                            "\nInvalid Operands to {}. \nLeft: {},\nRight: {}\n",
                        &self.get_type(),left_val,right_val)))
                    }
                } else {
                    Err(SyntaxError::InvalidOperands(format!(
                        "\nCannot evaluate operands to {} equation",
                        &self.get_type()
                    )))
                }
            }
            TreeNode::Subtraction{left,right} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        Ok(LiteralValue::new_number(left_num - right_num))
                    } else {
                        Err(SyntaxError::InvalidOperands(format!(
                            "\nInvalid Operands to {}. \nLeft: {},\nRight: {}\n",
                        &self.get_type(),left_val,right_val)))
                    }
                } else {
                    Err(SyntaxError::InvalidOperands(format!(
                        "\nCannot evaluate operands to {} equation",
                        &self.get_type()
                    )))
                }
            }
            TreeNode::Multiplication{left,right} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        Ok(LiteralValue::new_number(left_num * right_num))
                    } else {
                        Err(SyntaxError::InvalidOperands(format!(
                            "\nInvalid Operands to {}. \nLeft: {},\nRight: {}\n",
                        &self.get_type(),left_val,right_val)))
                    }
                } else {
                    Err(SyntaxError::InvalidOperands(format!(
                        "\nCannot evaluate operands to {} equation",
                        &self.get_type()
                    )))
                }
            }
            TreeNode::Division {left, right} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        if *right_num == 0.0 { return Err(SyntaxError::DivideByZero("Cannot divide by zero!".to_string())) }
                        Ok(LiteralValue::new_number(left_num / right_num))
                    } else {
                        Err(SyntaxError::InvalidOperands(format!(
                            "\nInvalid Operands to {}. \nLeft: {},\nRight: {}\n",
                        &self.get_type(),left_val,right_val)))
                    }
                } else {
                    Err(SyntaxError::InvalidOperands(format!(
                        "\nCannot evaluate operands to {} equation",
                        &self.get_type()
                    )))
                }
            },
            TreeNode::NumericLiteral(num) => Ok(num.clone()),
            TreeNode::Empty => Ok(LiteralValue::none()),
            _ => Err(SyntaxError::NotImplemented("Not implemented".to_string()))
        }
    }
}

mod tests {
    use super::{TreeNode,LiteralValue};
    use rand::prelude::*;

    #[test]
    fn new_add() {
        for i in -100..100 {
            let i_f = i as f64;
            let add = TreeNode::new_addition(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Addition {left, right} = add {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral(val) => match &val {
                        LiteralValue::Number(num) => num,
                        _ => {
                            assert_eq!(true,false,
                                "\nNumeric literal {} set to {} instead of real number\n",
                                left,val);
                            return
                        }
                    },
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of addition not a Numeric Literal, is {} instead.\n",
                            left.as_ref().to_string());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of addition not a Numeric Literal, is {} instead.\n",
                            right.as_ref().to_string());
                        return;
                    }
                };
                assert_eq!(*left_val,i_f,
                    "\nLeft side of addition should be {} but is {}!\n",
                    i_f,*left_val);
                assert_eq!(*right_val,LiteralValue::new_number(100.0-i_f),
                    "\nRight side of addition should be {} but is {}!\n",
                    i_f,*right_val);
                continue;
            }
            assert_eq!(true,false,
            "\nCreated node is not of type addition, found {} instead!\n",
            add.to_string())
        }
    }
    #[test]
    fn addition() {
        for i in -100..100 {
            let i_f = i as f64;
            let add = TreeNode::new_addition(
                TreeNode::NumericLiteral(LiteralValue::new_number(i_f)),
                TreeNode::NumericLiteral(LiteralValue::new_number(100.0-i_f))
            );
            if let TreeNode::Addition { .. } = add {
                match &add.eval() {
                    Ok(val) => {
                        match val {
                            &LiteralValue::Number(num) => {
                                assert_eq!(num,100.0,
                                    "\nAddition not following math correctly! Found {} when it should be 100.\n",
                                    num);
                            },
                            _ => {
                                assert_eq!(true,false,
                                    "\nAddition value: '{}' cannot be parsed to f64!\n",
                                    val);
                                return;
                            }
                        }
                    },
                    Err(_e) => {
                        assert_eq!(true,false,"\nFailed to evaluate addition\n.");
                        return;
                    }
                };
                continue;
            }
            assert_eq!(true,false,
                "\nCreated node was not a addition node, found {} instead!\n",
                add.to_string());
        }
    }

    #[test]
    fn new_subtraction() {
        for i in -100..100 {
            let i_f = i as f64;
            let sub = TreeNode::new_subtraction(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Subtraction { left, right } = sub {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of subtraction not a Numeric Literal, is {} instead.\n",
                            left.as_ref().to_string());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of subtraction not a Numeric Literal, is {} instead.\n",
                            right.as_ref().to_string());
                        return;
                    }
                };
                assert_eq!(*left_val,LiteralValue::new_number(i_f),
                    "\nLeft side of subtraction should be {} but is {}!\n",i_f,*left_val);
                assert_eq!(*right_val,LiteralValue::new_number(100.0-i_f),
                    "\nRight side of subtraction should be {} but is {}!\n",i_f,*right_val);
                continue;
            }
            assert_eq!(true,false,
            "\nCreated node was not a subtraction node, created {} instead!\n",
            sub.to_string())
        }
    }

    #[test]
    fn subtraction() {
        for i in -100..100 {
            let i_f = i as f64;
            let sub = TreeNode::new_subtraction(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Subtraction { .. } = sub {
                match &sub.eval() {
                    Ok(val) => {
                        match &val{
                            LiteralValue::Number(num) => {
                                assert_eq!(*num,i_f-(100.0-i_f),
                                "\nSubtraction not following math correctly! Found {} when it should be {}.",
                                num, i_f-(100.0-i_f));
                            },
                            _ => {
                                assert_eq!(true,false,
                                    "Subtraction value: {} cannot be parsed to f64!",
                                    val);
                                return;
                            }
                        }
                    },
                    Err(_e) => {
                        assert_eq!(true,false,"Failed to evaluate subtraction");
                        return;
                    }
                };
                continue;
            }
            assert_eq!(true,false,
                "\nCreated node was not a subtraction node, found {} instead!\n",
                sub.to_string())
        }
    }

    #[test]
    fn new_multiplication() {
        for i in -100..100 {
            let i_f = i as f64;
            let mult = TreeNode::new_multiplication(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Multiplication { left, right } = mult {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of multiplication not a Numeric Literal, is {} instead.\n",
                            left.as_ref().to_string());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of multiplication not a Numeric Literal, is {} instead.\n",
                            right.as_ref().to_string());
                        return;
                    }
                };
                assert_eq!(*left_val,LiteralValue::new_number(i_f),
                    "\nLeft side of multiplication should be {} but is {}!\n",i_f,*left_val);
                assert_eq!(*right_val,LiteralValue::new_number(100.0-i_f),
                    "\nRight side of multiplication should be {} but is {}!\n",i_f,*right_val);
                continue;
            }
            assert_eq!(true,false,
            "\nCreated node was not a multiplication node, created {} instead!\n",
            mult.to_string())
        }
    }

    #[test]
    fn multiplication() {
        for i in -100..100 {
            let i_f = i as f64;
            let mult = TreeNode::new_multiplication(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Multiplication { .. } = mult {
                match &mult.eval() {
                    Ok(val) => {
                        match &val {
                            LiteralValue::Number(num) => {
                                assert_eq!(*num,i_f*(100.0-i_f),
                                "\nMultiplication not following math correctly! Found {} when it should be {}.",
                                num, i_f*(100.0-i_f));
                            },
                            _ => {
                                assert_eq!(true,false,
                                    "Multiplication value: {} cannot be parsed to f64!",
                                    val);
                                return;
                            }
                        }
                    },
                    Err(_e) => {
                        assert_eq!(true,false,"Failed to evaluate multiplication");
                        return;
                    }
                };
                continue;
            }
            assert_eq!(true,false,
                "\nCreated node was not a multiplication node, found {} instead!\n",
                mult.to_string())
        }
    }

    #[test]
    fn new_division() {
        for i in -100..99 {
            let i_f = i as f64;
            let div = TreeNode::new_division(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Division { left, right } = div {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of division not a Numeric Literal, is {} instead.\n",
                            left.as_ref().to_string());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of division not a Numeric Literal, is {} instead.\n",
                            right.as_ref().to_string());
                        return;
                    }
                };
                assert_eq!(*left_val,LiteralValue::new_number(i_f),
                    "\nLeft side of division should be {} but is {}!\n",i_f,*left_val);
                assert_eq!(*right_val,LiteralValue::new_number(100.0-i_f),
                    "\nRight side of division should be {} but is {}!\n",i_f,*right_val);
                continue;
            }
            assert_eq!(true,false,
            "\nCreated node was not a division node, created {} instead!\n",
            div.to_string())
        }
    }

    #[test]
    fn division() {
        for i in -100..99 {
            let i_f = i as f64;
            let div = TreeNode::new_division(
                TreeNode::new_number(i_f),
                TreeNode::new_number(100.0-i_f)
            );
            if let TreeNode::Division { .. } = div {
                match &div.eval() {
                    Ok(val) => {
                        match val {
                            LiteralValue::Number(num) => {
                                assert_eq!(*num,i_f/(100.0-i_f),
                                "\nDivision not following math correctly! Found {} when it should be {}.",
                                num, i_f/(100.0-i_f));
                            },
                            _ => {
                                assert_eq!(true,false,
                                    "Division value: {} cannot be parsed to f64!",
                                    val);
                                return;
                            }
                        }
                    },
                    Err(_e) => {
                        assert_eq!(true,false,"Failed to evaluate division");
                        return;
                    }
                };
                continue;
            }
            assert_eq!(true,false,
                "\nCreated node was not a division node, found {} instead!\n",
                div.to_string())
        }
    }

    #[test]
    fn cant_divide_by_zero() {
        let div = TreeNode::new_division(
          TreeNode::new_number(10.0),
          TreeNode::new_number(0.0)  
        );
        match &div.eval() {
            Ok(val) => assert_eq!(true,false, "Allowed division by zero!\n Val: {}",val),
            Err(_) => assert_eq!(true,true) 
        }
    }
    #[test]
    fn communitive_property() {
        for i in 0..100 {
            let i_f = i as f64;
            let val1 = rand::random::<f64>() * 10.0  * i_f;
            let val2 = rand::random::<f64>() * 10.0  * i_f;
            let mult1 = TreeNode::new_multiplication(
                TreeNode::new_number(val1),
                TreeNode::new_number(val2)
            );
            let mult2 = TreeNode::new_multiplication(
                TreeNode::new_number(val2),
                TreeNode::new_number(val1)
            );
            if let (Ok(product1),Ok(product2)) = (mult1.eval(), mult2.eval()) {
                assert_eq!(product1, product2,"Error, communitive property not correct on Multiplication.");
            } else {
                assert_eq!(true, false,"Cannot evaluate multiplaction for communitive property");
            }
        }
    }

}