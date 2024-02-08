use crate::token::Token;

/// Enum for modeling AST nodes.
pub enum AST {
    // each node.
    Node(Token),
    // connections.
    Con(Token, Vec<AST>),
}

/// fmt display for ast
/// recursively prints AST.
impl core::fmt::Display for AST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AST::Node(token) => write!(f, " {}", token),
            AST::Con(head, rest) => {
                write!(f, "({}", head)?;
                for node in rest {
                    write!(f, "{}", node)?;
                }

                write!(f, ")")
            }
        }
    }
}
