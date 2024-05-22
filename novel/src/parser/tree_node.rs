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
    use super::TreeNode;
    #[test]
    fn new_add() {
        for i in -100..100 {
            let i_f = i as f64;
            let add = TreeNode::new_addition(
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
            );
            if let TreeNode::Addition {left, right} = add {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral(num) => num,
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
                assert_eq!(*right_val,100.0-i_f,
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
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
            );
            if let TreeNode::Addition { .. } = add {
                match &add.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                assert_eq!(num,100.0,
                                    "\nAddition not following math correctly! Found {} when it should be 100.\n",
                                    num);
                            },
                            Err(_e) => {
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
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
            );
            if let TreeNode::Subtraction { .. } = sub {
                match &sub.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                assert_eq!(num,i_f-(100.0-i_f),
                                "\nSubtraction not following math correctly! Found {} when it should be {}.",
                                num, i_f-(100.0-i_f));
                            },
                            Err(_e) => {
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
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
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
                assert_eq!(*left_val,i_f,
                    "\nLeft side of multiplication should be {} but is {}!\n",i_f,*left_val);
                assert_eq!(*right_val,100.0-i_f,
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
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
            );
            if let TreeNode::Multiplication { .. } = mult {
                match &mult.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                assert_eq!(num,i_f*(100.0-i_f),
                                "\nMultiplication not following math correctly! Found {} when it should be {}.",
                                num, i_f*(100.0-i_f));
                            },
                            Err(_e) => {
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
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
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
                assert_eq!(*left_val,i_f,
                    "\nLeft side of division should be {} but is {}!\n",i_f,*left_val);
                assert_eq!(*right_val,100.0-i_f,
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
                TreeNode::NumericLiteral(i_f),
                TreeNode::NumericLiteral(100.0-i_f)
            );
            if let TreeNode::Division { .. } = div {
                match &div.eval() {
                    Ok(val) => {
                        match val.parse::<f64>() {
                            Ok(num) => {
                                assert_eq!(num,i_f/(100.0-i_f),
                                "\nDivision not following math correctly! Found {} when it should be {}.",
                                num, i_f/(100.0-i_f));
                            },
                            Err(_e) => {
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
    
}