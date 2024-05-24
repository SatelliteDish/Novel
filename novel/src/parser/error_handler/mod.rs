pub struct ErrorHandler {
    hasError: bool,
    errors: Vec<SyntaxError>
}

use super::tree_node::TreeNode;

pub enum SyntaxError {
    DivideByZero(String),
    InvalidOperands(String),
    NotImplemented(String)
}

impl SyntaxError {
    pub fn invalid_operands(node: &TreeNode) -> SyntaxError {
        Self::InvalidOperands(format!(
            "\nInvalid operands to {}",
            node.get_type()
        ))
    }
}