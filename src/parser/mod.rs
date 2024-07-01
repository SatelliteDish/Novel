mod tokenizer;
use tokenizer::{LiteralValue, Token, TokenType, Tokenizer};

mod error_handler;
use error_handler::{Error, ErrorType, ErrorHandler};

mod tree_node;
use tree_node::TreeNode;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    error_handler: ErrorHandler
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(text),
            error_handler: ErrorHandler::new()
        }
    }

    pub fn parse(&'a mut self) -> String {
        let result_tree = &self.parse_e();
        if let Ok(val) =  result_tree.eval() {
            println!("Result = {}\n", val);
        }
        if self.error_handler.has_errors() {
            let _ = &self.error_handler.throw_errors();
        }
        result_tree.to_string()
    }
/*
    Parse Factor:
    F -> ID | f64 | -F
*/
    fn parse_f(&mut self) -> TreeNode<'a> {
        let node = match &self.tokenizer.peek() {
            Ok(_) => {
                let tkn = &self.tokenizer.scan().unwrap().clone();
            
                match tkn.token_type {
                    TokenType::NumericLiteral => {
                        TreeNode::new_number(tkn.val, *tkn)
                    },
                    TokenType::StringLiteral => {
                        TreeNode::new_string(tkn.val, *tkn)
                    },
                    TokenType::Identifier => {
                        TreeNode::new_identifier(tkn.val, *tkn)
                    },
                    TokenType::Comma => {
                        TreeNode::new_comma(tkn.val, *tkn)
                    },
                    TokenType::Dot => {
                        TreeNode::new_dot(tkn.val, *tkn)
                    },
                    TokenType::Bang => {
                        TreeNode::new_bang(tkn.val, *tkn)
                    },
                    TokenType::Question => {
                        TreeNode::new_question(tkn.val, *tkn)
                    },
                    TokenType::Interrobang => {
                        TreeNode::new_interrobang(tkn.val, *tkn)
                    },
                    TokenType::Semicolon => {
                        TreeNode::new_semicolon(tkn.val, *tkn)
                    },
                    TokenType::Colon => {
                        TreeNode::new_colon(tkn.val, *tkn)
                    },
                    TokenType::LeftParen => {
                        //If right paren found before left then it should be an unexpected token error 
                        todo!()
                    },
                    TokenType::Minus => {
                        TreeNode::new_negation(self.parse_f(),*tkn)
                    },
                    TokenType::If => {
                        TreeNode::new_if(
                            self.parse_e(),
                            self.parse_e(),
                            *tkn
                        )
                    },
                    TokenType::Therefore => {
                        todo!()
                    },
                    TokenType::False |
                    TokenType::True => {
                        TreeNode::new_bool(tkn.val, *tkn)
                    },
                    TokenType::None => {
                        TreeNode::new_none(tkn.val, *tkn)
                    },
                    TokenType::You => {
                        TreeNode::new_you(tkn.val, *tkn)
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
                    TokenType::Eof => {
                        TreeNode::new_eof(tkn.val, *tkn)
                    },
                    _ => {
                        let _ = &self.error_handler.report(
                            Error::new(
                                ErrorType::UnexpectedToken,
                                tkn.line(),
                                tkn.start()
                            ));
                        TreeNode::new_empty(tkn.val, *tkn)
                    }
                }
            },
            Err(e) => {
                let _ = &self.error_handler.report(
                    Error::new(
                        ErrorType::UnexpectedToken,
                        e.line,
                        e.position
                    )
                );
                TreeNode::new_empty(
                    LiteralValue::none(),
                    Token::invalid()
                    )
                }
            };
        print!("{}",node);
        node
    }
    /*
        Parse Expression:
        E = T {+|-T}
    */
    fn parse_e(&mut self) -> TreeNode<'a> {
        let mut e = self.parse_f();
        while self.is_more_tokens() {
            let token = self.tokenizer.scan();
            e = match token {
                Ok(tkn) => {
                    match tkn.token_type {
                        TokenType::Plus => {
                            TreeNode::new_addition(
                                e,
                                self.parse_f(),
                                tkn
                            )
                        },
                        TokenType::Minus => {
                            TreeNode::new_subtraction(
                                e,
                                self.parse_f(),
                                tkn
                            )
                        },
                        TokenType::Star => {
                            TreeNode::new_multiplication(
                                e,
                                self.parse_f(),
                                tkn
                             )
                        },
                        TokenType::Slash => {
                            TreeNode::new_division(
                                e,
                                self.parse_f(),
                                tkn
                            )
                        },
                        TokenType::Mod => {
                            TreeNode::new_modulus(
                                e,
                                self.parse_f(),
                                tkn
                            )
                        },
                        _ => {
                            TreeNode::new_empty(LiteralValue::none(), Token::invalid()) 
                        }
                    }
                },
                Err(e) => {
                    let _ = &self.error_handler.report(Error {
                        error_type: e.error_type,
                        line: e.line,
                        position: e.position
                    });
                    TreeNode::new_none(LiteralValue::none(), Token::invalid())
                }
            }
        }
        e
    }
    pub fn is_more_tokens(&mut self) -> bool {
            match &self.tokenizer.peek() {
                Ok(tkn) => !matches!(tkn.token_type, TokenType::Eof),
                Err(e) => {
                    let _ = &self.error_handler.report(Error {
                        error_type: e.error_type,
                        line: e.line,
                        position: e.position 
                    });
                    false
                }
            }
    }
}
