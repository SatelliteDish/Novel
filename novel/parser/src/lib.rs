use tokenizer::{self, Tokenizer};
use std::rc::*;

enum TreeNode {
    Addition(char,Weak<TreeNode>,Weak<TreeNode>),
    Subtraction(char,Weak<TreeNode>,Weak<TreeNode>),
    Multiplication(char,Weak<TreeNode>,Weak<TreeNode>),
    Division(char,Weak<TreeNode>,Weak<TreeNode>),
    Negation(char,Weak<TreeNode>),
    NumericLiteral(f64)
}

impl TreeNode {
    pub fn to_string(&self) -> String {
        match self {
            TreeNode::Addition(symbol,left,right) |
            TreeNode::Subtraction(symbol,left,right) |
            TreeNode::Multiplication(symbol,left,right) |
            TreeNode::Division(symbol,left,right) => {
                let left_text = match left.upgrade() {
                    Some(node) => node.to_string(),
                    None => "null".to_string()
                };
                let right_text = match right.upgrade() {
                    Some(node) => node.to_string(),
                    None => "null".to_string()
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
                ",symbol,left_text,right_text)
            },
            TreeNode::Negation(symbol,arg) => {
                let arg_text = match arg.upgrade() {
                    Some(node) => node.to_string(),
                    None => {"null".to_string()}
                };
                format!("
                    {} : {{
                        'arg': {{
                            {}
                        }} 
                    }}
                ",symbol,arg_text)
            },
            TreeNode::NumericLiteral(val) => {
                val.to_string()
            }
        }
    }
}

pub fn parse(text: &str) -> String {
    let mut tokenizer = Tokenizer::new(text);
    let mut more_tokens = true;
    let mut result = String::new();
    while more_tokens {
        match tokenizer.get_next_token() {
            Some(token) => {
                result = result + "\n" + &token.to_string();
            },
            None => {
                more_tokens = false;
            }
        } 
    }
    result
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
