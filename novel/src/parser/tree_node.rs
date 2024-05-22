pub enum TreeNode {
    Addition{left: Box<TreeNode>,right: Box<TreeNode>},
    Subtraction{left: Box<TreeNode>,right: Box<TreeNode>},
    Multiplication{left: Box<TreeNode>,right: Box<TreeNode>},
    Division{left: Box<TreeNode>,right: Box<TreeNode>},
    Negation(Box<TreeNode>),
    NumericLiteral(f64),
    Identifier(String),
    Keyword(String),
    StringLiteral(String),
    Empty
}
impl TreeNode {
    pub fn new_addition(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode::Addition {
            left:Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_subtraction(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode::Subtraction{
            left: Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_multiplication(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode::Multiplication{
            left: Box::new(left),
            right: Box::new(right)
        }
    }
    pub fn new_division(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode::Division {
            left: Box::new(left),
            right: Box::new(right)
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
    pub fn eval(&self) -> Result<String,String> {
        match &self {
            TreeNode::Addition{left,right} => {
                let left_val = match left.eval() {
                    Ok(val) => match val.parse::<f64>() {
                        Ok(num) => num,
                        Err(e) => panic!("{}",e.to_string())
                    },
                    Err(e) => {
                        panic!("{}",e.to_string())
                    }
                };
                let right_val = match right.eval() {
                    Ok(val) => match val.parse::<f64>() {
                        Ok(num) => num,
                        Err(e) => panic!("{}",e.to_string())
                    },
                    Err(e) => {
                        panic!("{}",e.to_string())
                    }
                };
                Ok({left_val + right_val}.to_string())
            },
            TreeNode::Negation(arg) => {
                match arg.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => Ok({num * -1.0}.to_string()),
                            Err(e) => Err(e.to_string())
                        }
                    },
                    Err(e) => Err(e)
                }
            },
            TreeNode::Subtraction{left,right} => {
                let left_val = match left.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => num,
                            Err(e) => panic!("{}", e.to_string())
                        }
                    },
                    Err(e) => panic!("{}",e)
                };
                let right_val = match right.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => num,
                            Err(e) => panic!("{}",e.to_string())
                        }
                    },
                    Err(e) => panic!("{}",e)
                };
                Ok({left_val - right_val}.to_string())
            },
            TreeNode::Multiplication{left,right} => {
                let left_val = match left.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => num,
                            Err(e) => panic!("{}", e.to_string())
                        }
                    },
                    Err(e) => panic!("{}",e)
                };
                let right_val = match right.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => num,
                            Err(e) => panic!("{}",e.to_string())
                        }
                    },
                    Err(e) => panic!("{}",e)
                };
                Ok({left_val * right_val}.to_string())
            }
            TreeNode::Division {left, right} => {
                let left_val = match left.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => num,
                            Err(e) => panic!("{}", e.to_string())
                        }
                    },
                    Err(e) => panic!("{}",e)
                };
                let right_val = match right.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                if num == 0.0 {
                                    panic!("Error, cannot divide by zero!")
                                } else {
                                    num
                                }
                            },
                            Err(e) => panic!("{}",e.to_string())
                        }
                    },
                    Err(e) => panic!("{}",e)
                };
                Ok({left_val / right_val}.to_string())
            }
            TreeNode::NumericLiteral(num) => Ok(num.to_string()),
            TreeNode::Empty => Ok("Empty!".to_string()),
            _ => Err("Not implemented".to_string())
        }
    }
}

mod tests {
    use std::borrow::Borrow;

    use super::TreeNode;
    #[test]
    fn new_add() {
        for i in 0..100 {
            let add = TreeNode::new_addition(TreeNode::NumericLiteral(i as f64), TreeNode::NumericLiteral({100-i} as f64));
            if let TreeNode::Addition{ left, right } = add {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral(num) => *num,
                    _ => i as f64*2.0
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral(num) => *num,
                    _ => i as f64
                };
                assert_eq!(left_val, i as f64);
                assert_eq!(right_val, 100.0-i as f64);
                return;
            }
            assert_eq!(true,false);
        }
    }
    #[test]
    fn addition() {
        for i in -100..100 {
            let i_f = i as f64;
            let add = TreeNode::new_addition(
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f));
            if let TreeNode::Addition{ .. } = add {
                assert_eq!(match &add.eval() {
                    Ok(val) => val,
                    Err(e) => panic!("{}",e)
                },"100");
                break;
            }
            assert_eq!(true,false);
        }
    }

    #[test]
    fn new_subtraction() {
        for i in -100..100 {
            let i_f = i as f64;
            let sub = TreeNode::new_subtraction(
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
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
                assert_eq!(*left_val,i_f,
                    "\nLeft side of subtraction should be {} but is {}!\n",i_f,*left_val);
                assert_eq!(*right_val,100.0-i_f,
                    "\nRight side of subtraction should be {} but is {}!\n",i_f,*right_val);
            }

        }
    }

    #[test]
    fn subtraction() {
        for i in -100..100 {
            let i_f = i as f64;
            let sub = TreeNode::new_subtraction(
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
            );
            if let TreeNode::Subtraction { .. } = sub {
                match &sub.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                assert_eq!(num,i_f-(100.0-i_f),"Subtraction not following math correctly!");
                            },
                            Err(_e) => {
                                assert_eq!(true,false,"Subtraction value cannot be parsed to f64!");
                                return;
                            }
                        }
                    },
                    Err(_e) => {
                        assert_eq!(true,false,"Failed to evaluate subtraction");
                        return;
                    }
                };
                
            }
        }
    }

}