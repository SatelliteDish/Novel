mod tokenizer;
use tokenizer::{LiteralValue, Token, TokenType, Tokenizer};

mod error_handler;
use error_handler::{Error, ErrorHandler, ErrorType};

mod tree_node;
use tree_node::TreeNode;




pub fn parse(text: &str) -> String {
    let  mut tokenizer = Tokenizer::new(text);
    let mut error_handler = ErrorHandler::new();
    let result_tree = parse_e(&mut tokenizer,&mut error_handler);
    if error_handler.has_errors() {
        error_handler.throw_errors();
    }
    if let Ok(val) =  result_tree.eval() {
        print!("Result = {}\n", val);
    }
    result_tree.to_string()
}
/*
    Parse Factor:
    F -> ID | f64 | -F
*/
fn parse_f(tokenizer: &mut Tokenizer, error_handler: &mut ErrorHandler) -> TreeNode {
    let node = match &tokenizer.peek() {
        Ok(_) => {
            let tkn = tokenizer.scan().unwrap();
            match tkn.token_type {
                TokenType::NumericLiteral => {
                    TreeNode::new_number(tkn.clone().val, tkn)
                },
                TokenType::StringLiteral => {
                    TreeNode::new_string(tkn.clone().val, tkn)
                },
                TokenType::Identifier => {
                    TreeNode::new_identifier(tkn.clone().val, tkn)
                },
                TokenType::Comma => {
                    TreeNode::new_comma(tkn.clone().val, tkn)
                },
                TokenType::Dot => {
                    TreeNode::new_dot(tkn.clone().val, tkn)
                },
                TokenType::Bang => {
                    TreeNode::new_bang(tkn.clone().val, tkn)
                },
                TokenType::Question => {
                    TreeNode::new_question(tkn.clone().val, tkn)
                },
                TokenType::Interrobang => {
                    TreeNode::new_interrobang(tkn.clone().val, tkn)
                },
                TokenType::Semicolon => {
                    TreeNode::new_semicolon(tkn.clone().val, tkn)
                },
                TokenType::Colon => {
                    TreeNode::new_colon(tkn.clone().val, tkn)
                },
                TokenType::LeftParen => {
                    //If right paren found before left then it should be an unexpected token error 
                    todo!()
                },
                TokenType::Minus => {
                    TreeNode::new_negation(parse_f(tokenizer, error_handler),tkn)
                },
                TokenType::If => {
                    TreeNode::new_if(
                        parse_e(tokenizer, error_handler),
                    parse_e(tokenizer, error_handler),
                    tkn)
                },
                TokenType::Therefore => {
                    todo!()
                },
                TokenType::False |
                TokenType::True => {
                    TreeNode::new_bool(tkn.clone().val, tkn).unwrap()
                },
                TokenType::None => {
                    TreeNode::new_none(tkn.clone().val,tkn)
                },
                TokenType::You => {
                    TreeNode::new_you(tkn.clone().val,tkn)
                },
                TokenType::Assignment => {
                    todo!()
                },
                TokenType::Declaration => {
                    todo!()
                },
                TokenType::IdKeyword => {
                    todo!()
                },
                TokenType::EOF => {
                    TreeNode::new_eof(tkn.clone().val,tkn)
                },
                _ => {
                    error_handler.report(
                         Error::new(
                            ErrorType::UnexpectedToken,
                            tkn.line(),
                            tkn.start()
                        ));
                    TreeNode::new_empty(tkn.clone().val, tkn)
                }
            }
        },
        Err(e) => {
            error_handler.report(
                Error::new(
                   ErrorType::UnexpectedToken,
                   &e.line,
                   &e.position
               ));
           TreeNode::new_empty(
            LiteralValue::none(),
            Token::invalid()
        )
        }
    };
    print!("{}",node.to_string());
    node
}
/*
    Parse Expression:
    E = T {+|-T}
*/
fn parse_e(tokenizer: &mut Tokenizer, error_handler: &mut ErrorHandler) -> TreeNode {
    let mut e = parse_f(tokenizer,error_handler);
    while {
        match &tokenizer.peek() {
            Ok(tkn) => {
                match tkn.token_type {
                    TokenType::Plus |
                    TokenType::Minus |
                    TokenType::Star |
                    TokenType::Slash |
                    TokenType::Mod => true,
                    _ => false
                }
            },
            Err(e) => {
                error_handler.report(Error {
                    error_type: e.error_type.clone(),
                    line: e.line,
                    position: e.position 
                });
                false
            }
        }
        } {
        let token = tokenizer.scan();
        e = match token {
            Ok(tkn) => {
                match tkn.token_type {
                    TokenType::Plus => {
                        TreeNode::new_addition(
                        e,
                        parse_f(tokenizer, error_handler),
                        tkn
                        )
                    },
                    TokenType::Minus => {
                        TreeNode::new_subtraction(
                        e,
                        parse_f(tokenizer, error_handler),
                        tkn
                        )
                    },
                    TokenType::Star => {
                        TreeNode::new_multiplication(
                        e,
                        parse_f(tokenizer, error_handler),
                        tkn
                        )
                    },
                    TokenType::Slash => {
                        TreeNode::new_division(
                        e,
                        parse_f(tokenizer, error_handler),
                        tkn
                        )
                    },
                    TokenType::Mod => {
                        TreeNode::new_modulus(
                        e,
                        parse_f(tokenizer, error_handler),
                        tkn
                        )
                    },
                    _ => {
                       TreeNode::new_empty(LiteralValue::none(), Token::invalid()) 
                    }
                }
            },
            Err(e) => {
                error_handler.report(Error {
                    error_type: e.error_type.clone(),
                    line: e.line,
                    position: e.position
                });
                TreeNode::new_none(LiteralValue::none(), Token::invalid())
            }
        }
    }
    e
}