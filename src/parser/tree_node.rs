use super::{error_handler::{Error, ErrorType}, tokenizer::{Token,TokenType}};
use super::LiteralValue;

macro_rules! node_constructor {
    (UNARY, $type: tt, $name: ident) => {
        pub fn $name(val: LiteralValue<'a>, token: Token<'a>) -> Self {
            TreeNode::$type {
                val,
                token
            }
        }
    };
    (BINARY, $type: tt, $name: ident) => {
        pub fn $name(left: Self, right: Self, token: Token<'a>) -> Self {
            TreeNode::$type {
                left:Box::new(left),
                right: Box::new(right),
                token
            }
        }
    }
}

#[allow(dead_code)]
pub enum TreeNode<'a> {
    NumericLiteral{val: LiteralValue<'a>,token: Token<'a>},
    StringLiteral{val: LiteralValue<'a>,token: Token<'a>},
    Identifier{val: LiteralValue<'a>,token: Token<'a>},

    Comma{val: LiteralValue<'a>,token: Token<'a>},
    Dot{val: LiteralValue<'a>,token: Token<'a>},
    Bang{val: LiteralValue<'a>,token: Token<'a>},
    Question{val: LiteralValue<'a>,token: Token<'a>},
    Interrobang{val: LiteralValue<'a>,token: Token<'a>},

    Semicolon{val: LiteralValue<'a>,token: Token<'a>},
    Colon{val: LiteralValue<'a>,token: Token<'a>},

    Parens{val: Box<TreeNode<'a>>, left_token: Token<'a>, right_token: Token<'a>},

    Addition{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Subtraction{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Multiplication{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Division{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Modulo{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>, token: Token<'a>},
    
    
    Negation{arg: Box<TreeNode<'a>>,token: Token<'a>},
    Keyword{val: LiteralValue<'a>,token: Token<'a>},
    
    Ellipsis{val: LiteralValue<'a>,token: Token<'a>},

    If{condition: Box<TreeNode<'a>>,expression: Box<TreeNode<'a>>,token: Token<'a>},
    Therefore{condition: Box<TreeNode<'a>>,expression: Box<TreeNode<'a>>,token: Token<'a>},
    
    EqTo{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    NeqTo{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Or{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Not{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    And{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Less{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    Greater{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    LessEq{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    GreaterEq{left: Box<TreeNode<'a>>,right: Box<TreeNode<'a>>,token: Token<'a>},
    
    BooleanLiteral{val: LiteralValue<'a>,token: Token<'a>},
    None{val: LiteralValue<'a>,token: Token<'a>},
    You{val: LiteralValue<'a>,token: Token<'a>},
    Assignment{identifier: LiteralValue<'a>, val: LiteralValue<'a>,token: Token<'a>},
    Declaration{identifier: LiteralValue<'a>, val: Box<TreeNode<'a>>,token: Token<'a>},
    
    Eof{ val: LiteralValue<'a>, token: Token<'a> },
    Empty{ val: LiteralValue<'a>, token: Token<'a> },
}

impl std::fmt::Display for TreeNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",match &self {
            TreeNode::NumericLiteral{val,..} |
            TreeNode::StringLiteral{val,..} |
            TreeNode::Identifier{val,..} |

            TreeNode::Comma{val,..} |
            TreeNode::Dot{val,..} |
            TreeNode::Bang{val,..} |
            TreeNode::Question{val,..} |
            TreeNode::Interrobang{val,..} |
        
            TreeNode::Semicolon{val,..} |
            TreeNode::Colon{val,..} => val.to_string(),
        
            TreeNode::Parens{val,..} => {
                format!("
                    \"{}\": {{
                        {}
                    }}
                ",self.get_type(),val)
            },
        
            TreeNode::Addition{left,right,..} |
            TreeNode::Subtraction{left,right,..} |
            TreeNode::Multiplication{left,right,..} |
            TreeNode::Division{left,right,..} |
            TreeNode::Modulo{left,right,..} => {
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
            }
            
            
            TreeNode::Negation{arg,..} => {
                format!("
                    \"-\" : {{
                        \"arg\": {{
                            {}
                        }} 
                    }}
                ",arg
                )
            },
            TreeNode::Keyword{val,..} => val.to_string(),

            TreeNode::Ellipsis{val,..} => val.to_string(),

            TreeNode::If{condition,expression,..} |
            TreeNode::Therefore{condition,expression,..} => {
                format!("
                    \"{}\": {{
                        \"condition\": {{
                            {}
                        }},
                        \"expression\": {{
                            {}
                        }}
                    }}
                ",self.get_type(),condition,expression)
            }
        
            TreeNode::EqTo{left,right,..} |
            TreeNode::NeqTo{left,right,..} |
            TreeNode::Or{left,right,..} |
            TreeNode::Not{left,right,..} |
            TreeNode::And{left,right,..} |
            TreeNode::Less{left,right,..} |
            TreeNode::Greater{left,right,..} |
            TreeNode::LessEq{left,right,..} |
            TreeNode::GreaterEq{left,right,..} => {
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
            }

            TreeNode::BooleanLiteral{val,..} => {
                format!("
                    \"{}\": {{
                        {}
                    }}
                ",self.get_type(),val)
            },
            TreeNode::None{..} => "None".to_string(),
            TreeNode::You{..} => "You".to_string(),
            TreeNode::Assignment{identifier,val,..}  => {
                format!("
                    \"{}\": {{
                        \"{}\": {{
                            {}
                        }}
                    }}
                ",self.get_type(),identifier,val)
            },
            TreeNode::Declaration{identifier,val,..} => {
                format!("
                    \"{}\": {{
                        \"{}\": {{
                            {}
                        }}
                    }}
                ",self.get_type(),identifier,val)
            },

            TreeNode::Eof{..} => "EOF".to_string(),
            TreeNode::Empty{..} => "null".to_string(),
        })
    }
}

#[allow(dead_code)]
impl<'a> TreeNode<'a> {
    pub fn from_token(tkn: Token<'a>) -> Self {
        let val = tkn.val;
        match &tkn.token_type {
            TokenType::NumericLiteral => TreeNode::new_number(val, tkn),
            TokenType::StringLiteral => TreeNode::new_string(val, tkn),
            _ => {
                TreeNode::Empty{val,token: tkn}
            }
        }
    }

    pub fn new_if(condition: TreeNode<'a>, expression: TreeNode<'a>, token: Token<'a>) -> TreeNode<'a> {
        TreeNode::If {
            condition: Box::new(condition),
            expression: Box::new(expression),
            token
        }
    }

    pub fn new_negation(arg: TreeNode<'a>, token: Token<'a>) -> TreeNode<'a> {
        TreeNode::Negation {
            arg: Box::new(arg),
            token
        }
    }

    node_constructor!(BINARY, Addition, new_addition);
    node_constructor!(BINARY, Subtraction, new_subtraction);
    node_constructor!(BINARY, Multiplication, new_multiplication);
    node_constructor!(BINARY, Division, new_division);
    node_constructor!(BINARY, Modulo, new_modulus);
    node_constructor!(UNARY, NumericLiteral, new_number);
    node_constructor!(UNARY, StringLiteral, new_string);
    node_constructor!(UNARY, Identifier, new_identifier);
    node_constructor!(UNARY, Keyword, new_keyword);
    node_constructor!(UNARY, Empty, new_empty);
    node_constructor!(UNARY, Comma, new_comma);
    node_constructor!(UNARY, Dot, new_dot);
    node_constructor!(UNARY, Bang, new_bang);
    node_constructor!(UNARY, Question, new_question); 
    node_constructor!(UNARY, Interrobang, new_interrobang);
    node_constructor!(UNARY, Semicolon, new_semicolon);
    node_constructor!(UNARY, Colon, new_colon);
    node_constructor!(UNARY, BooleanLiteral, new_bool);
    node_constructor!(UNARY, None, new_none);
    node_constructor!(UNARY, Eof, new_eof);
    node_constructor!(UNARY, You, new_you);

    pub fn get_type(&self) -> String {
        match &self {
            TreeNode::NumericLiteral{..} => "NumericLiteral".to_string(),
            TreeNode::StringLiteral{..} => "StringLiteral".to_string(),
            TreeNode::Identifier{..} => "Identifier".to_string(),
            TreeNode::Comma{..} => "Comma".to_string(),
            TreeNode::Dot{..} => "Dot".to_string(),
            TreeNode::Bang{..} => "Bang".to_string(),
            TreeNode::Question{..} => "Question".to_string(),
            TreeNode::Interrobang{..} => "Interrobang".to_string(),
            TreeNode::Semicolon{..} => "Semicolon".to_string(),
            TreeNode::Colon{..} => "Colon".to_string(),
            TreeNode::Parens{..} => "Parenthesis".to_string(),
            TreeNode::Addition{..} => "Addition".to_string(),
            TreeNode::Subtraction{..} => "Subtraction".to_string(),
            TreeNode::Multiplication{..} => "Multiplication".to_string(),
            TreeNode::Division{..} => "Division".to_string(),
            TreeNode::Modulo{..} => "Modulo".to_string(),
            TreeNode::Negation{..}  => "Negation".to_string(),
            TreeNode::Keyword{..} => "Keyword".to_string(),
            TreeNode::Ellipsis{..} => "Ellipsis".to_string(),
            TreeNode::If{..} => "If".to_string(),
            TreeNode::Therefore{..} => "Therefore".to_string(),
            TreeNode::EqTo{..} => "Equal To".to_string(),
            TreeNode::NeqTo{..} => "Not Equal To".to_string(),
            TreeNode::Or{..} => "Or".to_string(),
            TreeNode::Not{..} => "Not".to_string(),
            TreeNode::And{..} => "And".to_string(),
            TreeNode::Less{..} => "Less Than".to_string(),
            TreeNode::Greater{..} => "Greater Than".to_string(),
            TreeNode::LessEq{..} => "Less Than Or Equal To".to_string(),
            TreeNode::GreaterEq{..} => "Greater Than Or Equal To".to_string(),
            TreeNode::BooleanLiteral{..} => "BooleanLiteral".to_string(),
            TreeNode::None{..} => "None".to_string(),
            TreeNode::You{..} => "You".to_string(),
            TreeNode::Assignment{..} => "Assignment".to_string(),
            TreeNode::Declaration{..} => "Declaration".to_string(),
            TreeNode::Eof{..} => "EOF".to_string(),
            TreeNode::Empty{..} => "null".to_string(),
        }
    }

    pub fn eval(&self) -> Result<LiteralValue,Error> {
        match &self {   
            TreeNode::Addition{left,right,token} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        Ok(LiteralValue::new_number(left_num + right_num))
                    } else {
                        Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                    }
                } else {
                    Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                }
            }
            TreeNode::Subtraction{left,right,token} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        Ok(LiteralValue::new_number(left_num - right_num))
                    } else {
                        Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                    }
                } else {
                    Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                }
            }
            TreeNode::Multiplication{left,right,token} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        Ok(LiteralValue::new_number(left_num * right_num))
                    } else {
                        Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                    }
                } else {
                    Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                }
            }
            TreeNode::Division {left, right,token} => {
                if let (Ok(left_val),Ok(right_val)) = (&left.eval(),&right.eval()) {
                    if let (LiteralValue::Number(left_num),LiteralValue::Number(right_num)) = (left_val,right_val) {
                        if *right_num == 0.0 { return Err(Error::new(ErrorType::DivideByZero,token.line(), token.start())) }
                        Ok(LiteralValue::new_number(left_num / right_num))
                    } else {
                        Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                    }
                } else {
                    Err(Error::new(ErrorType::InvalidOperands, token.line(), token.start()))
                }
            },
            TreeNode::NumericLiteral{val,..} => Ok(*val),
            TreeNode::Empty{..} => Ok(LiteralValue::none()),
            _ => Err(Error::new(ErrorType::NotImplemented, 0, 0))

        }
    }

}

mod tests {
    use super::{TreeNode,LiteralValue,Token};

    #[allow(dead_code)]
    fn get_test_number(num: f64) -> TreeNode<'static> {
        TreeNode::new_number(LiteralValue::new_number(num),Token::invalid())
    }

    #[test]
    fn new_add() {
        for i in -100..100 {
            let i_f = i as f64;
            let add = TreeNode::new_addition(
                get_test_number(i_f),
                get_test_number(100.0-i_f),
                Token::invalid()
            );
            if let TreeNode::Addition {left, right,..} = add {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral{val,..} => match &val {
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
                            left.as_ref());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of addition not a Numeric Literal, is {} instead.\n",
                            right.as_ref());
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
            add)
        }
    }
    #[test]
    fn addition() {
        for i in -100..100 {
            let i_f = i as f64;
            let add = TreeNode::new_addition(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
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
                add);
        }
    }

    #[test]
    fn new_subtraction() {
        for i in -100..100 {
            let i_f = i as f64;
            let sub = TreeNode::new_subtraction(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
            );
            if let TreeNode::Subtraction { left, right ,..} = sub {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of subtraction not a Numeric Literal, is {} instead.\n",
                            left.as_ref());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of subtraction not a Numeric Literal, is {} instead.\n",
                            right.as_ref());
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
            sub)
        }
    }

    #[test]
    fn subtraction() {
        for i in -100..100 {
            let i_f = i as f64;
            let sub = TreeNode::new_subtraction(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
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
                sub)
        }
    }

    #[test]
    fn new_multiplication() {
        for i in -100..100 {
            let i_f = i as f64;
            let mult = TreeNode::new_multiplication(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
            );
            if let TreeNode::Multiplication { left, right,.. } = mult {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of multiplication not a Numeric Literal, is {} instead.\n",
                            left.as_ref());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of multiplication not a Numeric Literal, is {} instead.\n",
                            right.as_ref());
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
            mult)
        }
    }

    #[test]
    fn multiplication() {
        for i in -100..100 {
            let i_f = i as f64;
            let mult = TreeNode::new_multiplication(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
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
                mult)
        }
    }

    #[test]
    fn new_division() {
        for i in -100..99 {
            let i_f = i as f64;
            let div = TreeNode::new_division(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
            );
            if let TreeNode::Division { left, right,.. } = div {
                let left_val = match left.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nLeft branch of division not a Numeric Literal, is {} instead.\n",
                            left.as_ref());
                        return
                    }
                };
                let right_val = match right.as_ref() {
                    TreeNode::NumericLiteral{val,..} => val,
                    _ => {
                        assert_eq!(true,false,
                            "\nRight branch of division not a Numeric Literal, is {} instead.\n",
                            right.as_ref());
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
            div)
        }
    }

    #[test]
    fn division() {
        for i in -100..99 {
            let i_f = i as f64;
            let div = TreeNode::new_division(
                get_test_number(i_f),
                get_test_number(100.0 - i_f),
                Token::invalid()
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
                div)
        }
    }

    #[test]
    fn cant_divide_by_zero() {
        let div = TreeNode::new_division(
            get_test_number(10.0),
            get_test_number(0.0),
            Token::invalid()
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
                get_test_number(val1),
                get_test_number(val2),
                Token::invalid()
            );
            let mult2 = TreeNode::new_multiplication(
                get_test_number(val2),
                get_test_number(val1),
                Token::invalid()
            );
            if let (Ok(product1),Ok(product2)) = (mult1.eval(), mult2.eval()) {
                assert_eq!(product1, product2,"Error, communitive property not correct on Multiplication.");
            } else {
                assert_eq!(true, false,"Cannot evaluate multiplaction for communitive property");
            }
        }
    }

}
