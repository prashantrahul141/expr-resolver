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

// Top level parser.
// parses linear array of tokens into AST.
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self { lexer }
    }

}
