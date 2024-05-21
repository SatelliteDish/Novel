use std::fmt;
mod tokenizer;
use tokenizer::Tokenizer;

mod tree_node;
use tree_node::TreeNode;

enum ParserError {
    DivideByZero
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match &self {
            ParserError::DivideByZero => {
                format!("\nError: Cannot divide by zero!\n")
            }
        };
        write!(f,"{}",message)
    }
}

pub fn parse(text: &str) -> String {
    let  mut tokenizer = Tokenizer::new(text);
    let result_tree = parse_e(&mut tokenizer);
    if let Ok(val) =  result_tree.eval() {
        print!("Result = {}\n", val);
    }
    result_tree.to_string()
}
/*
    Parse Factor:
    F -> ID | f64 | -F
*/
fn parse_f(tokenizer: &mut Tokenizer) -> TreeNode {
    let token = match &tokenizer.token {
        Some(tkn) => {
            let val = tkn.val.clone();
            match tkn.token_type {
                tokenizer::TokenType::Identifier => {
                    tokenizer.scan();
                    TreeNode::Identifier(val.to_string())
                },
                tokenizer::TokenType::Keyword => {
                    tokenizer.scan();
                    TreeNode::Keyword(val.to_string())
                },
                tokenizer::TokenType::NumericLiteral => {
                    tokenizer.scan();
                    TreeNode::NumericLiteral(val.parse::<f64>().unwrap())
                },
                tokenizer::TokenType::StringLiteral => {
                    tokenizer.scan();
                    TreeNode::StringLiteral(val.to_string())
                },
                tokenizer::TokenType::Symbol => {
                    if tkn.val == "-" {
                        tokenizer.scan();
                        TreeNode::Negation(Box::new(parse_f(tokenizer)))
                    } else {
                        tokenizer.scan();
                        TreeNode::Empty
                    }
    
                }
                _ => {
                    tokenizer.scan();
                    TreeNode::Empty
                }
            }
        },
        None => {
            tokenizer.scan();
            TreeNode::Empty
        }
    };
    token
}
/*
    Parse Expression:
    E = T {+|-T}
*/
fn parse_e(tokenizer: &mut Tokenizer) -> Box<TreeNode> {
    let mut e = parse_f(tokenizer);
   while {
    match &tokenizer.token {
        Some(tkn) => {
            match tkn.token_type {
                tokenizer::TokenType::NumericLiteral => true,
                tokenizer::TokenType::Symbol => {
                    if tkn.val == "+" || tkn.val == "-" || tkn.val == "*" || tkn.val == "/" {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            }
        },
        None => false
    }
   } {
        if let Some(token) = &tokenizer.token {
            if token.token_type == tokenizer::TokenType::Symbol {
                if token.val == "+" {
                    tokenizer.scan();//To skip operator
                    let left = e;
                    let right = parse_f(tokenizer);
                    e = TreeNode::new_addition(left, right);
                } else if token.val == "-" {
                    tokenizer.scan();//to skip operator
                    let left = e;
                    let right = parse_f(tokenizer);
                    e = TreeNode::new_subtraction(left, right)
               } else if token.val == "*" {
                tokenizer.scan();//to skip operator
                let left = e;
                let right = parse_f(tokenizer);
                e = TreeNode::new_multiplication(left, right)
                } else if token.val == "/" {
                    tokenizer.scan();//to skip operator
                    let left = e;
                    let right = parse_f(tokenizer);
                    e = TreeNode::new_division(left, right)
                    }
                }
        }
   }
    Box::new(e)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
