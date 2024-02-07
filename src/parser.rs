use crate::lexer::Lexer;
use crate::lexer::Token;

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

    pub fn parse(&mut self) -> AST {
        self.expr(0)
    }

    fn expr(&mut self, min_binding_power: u8) -> AST {
        let mut left_hand_side = match self.lexer.next_token() {
            Token::Number(f) => AST::Node(Token::Number(f)),
            Token::Eof => panic!("Unexpected token : EOF"),
            operator => {
                let ((), right_binding_power) = Parser::prefix_binding_power(operator);
                let right_hand_side = self.expr(right_binding_power);
                AST::Con(operator, vec![right_hand_side])
            }
        };

        loop {
            let operator = match self.lexer.peek() {
                Token::Number(n) => panic!("Expected operator recieved number : {n}"),
                Token::Eof => break,
                op => op,
            };

            let (left_bp, right_bp) = Parser::infix_binding_power(operator);

            if left_bp < min_binding_power {
                break;
            }

            self.lexer.next_token();

            let right_hand_side = self.expr(right_bp);

            left_hand_side = AST::Con(operator, vec![left_hand_side, right_hand_side]);
        }

        left_hand_side
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
