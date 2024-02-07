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

    fn infix_binding_power(token: Token) -> (u8, u8) {
        match token {
            Token::Plus => (1, 2),
            Token::Minus => (1, 2),
            Token::Star => (3, 4),
            Token::Slash => (3, 4),

            // basically unreachable.
            t => panic!("Cannot get infix binding power of {t}"),
        }
    }

    fn prefix_binding_power(token: Token) -> ((), u8) {
        match token {
            Token::Minus | Token::Plus => ((), 5),

            // basically unreachable.
            t => panic!("Cannot get infix binding power of {t}"),
        }
    }
}
