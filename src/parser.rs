use std::vec;

use crate::ast::AST;
use crate::lexer::Lexer;
use crate::token::Token;

// Top level parser.
// parses linear array of tokens into AST.
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    // constructor for parser.
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self { lexer }
    }

    // public parse method.
    pub fn parse(&mut self) -> Result<AST, String> {
        // we start with binding power of 0.
        self.expr(0)
    }

    /// Parses an expression using Operator-Precedence parse (Pratt Parsing)
    /// ref : https://en.wikipedia.org/wiki/Operator-precedence_parser
    /// # Arguments
    /// * min_binding_power - minimum binding power till recursivel parse the expression.
    /// # Returns
    /// * AST - ast of the expression.
    fn expr(&mut self, min_binding_power: u8) -> Result<AST, String> {
        // Parsing left hand side of the expression.
        let mut left_hand_side = match self.lexer.next_token() {
            // if the token is a number we simply create a node out of it.
            Token::Number(f) => AST::Node(Token::Number(f)),

            // if we reached the end we panic.
            Token::Eof => return Err("Unexpected token : EOF".to_string()),

            // if its a operator, then it means the operator is a unary.
            operator => {
                // we get the right binding power of the unary operator.
                let ((), right_binding_power) = Parser::prefix_binding_power(operator);

                // then recursively parse it.
                match self.expr(right_binding_power) {
                    // save if its safe.
                    Ok(right_hand_side) => AST::Con(operator, vec![right_hand_side]),
                    // return if err.
                    Err(err) => return Err(err),
                }
            }
        };

        loop {
            // Operator: Infix operator.
            let operator = match self.lexer.peek() {
                // shouldn't be a number, obviously.
                Token::Number(n) => return Err(format!("Expected operator recieved number : {n}")),

                // also shouldn't end.
                Token::Eof => break,

                // take the operator.
                op => op,
            };

            // get the left binding power of the postfix operator.
            if let Some((left_bp, ())) = Parser::postfix_binding_power(operator) {
                // we break the loop when precendence of the current left binding
                // power of the postfix operator is less than minimum binding power.
                if left_bp < min_binding_power {
                    break;
                }
                self.lexer.next_token();

                left_hand_side = AST::Con(operator, vec![left_hand_side]);

                // we need to skip the current iteration.
                continue;
            }

            // get the left binding power and right binding power of this infix operator.
            let (left_bp, right_bp) = Parser::infix_binding_power(operator);

            // ends recursion when the minimum binding power for this
            // expr function call is less then left binding power of the current operator.
            if left_bp < min_binding_power {
                break;
            }

            // consume operator token.
            self.lexer.next_token();

            // recurisvely call expr to parse right hand side of the expression.
            match self.expr(right_bp) {
                Ok(right_hand_side) => {
                    // create ast.
                    left_hand_side = AST::Con(operator, vec![left_hand_side, right_hand_side]);
                }
                Err(err) => return Err(err),
            }
        }

        // return the created AST.
        Ok(left_hand_side)
    }

    /// Gets the infix binding power of a operator.
    /// # Arguments
    /// * token - the operator token.
    /// # Returns
    /// * (left, right) - left and right infix binding power of the operator.
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

    /// Gets the postfix binding power of a operator.
    /// # Arguments
    /// * token - the operator token.
    /// # Returns
    /// * (left, ())) - left postfix binding power of the operator.
    fn postfix_binding_power(token: Token) -> Option<(u8, ())> {
        let power = match token {
            Token::Bang => (6, ()),

            // basically unreachable.
            _ => return None,
        };

        Some(power)
    }

    /// Gets the prefix binding power of a unary operators.
    /// # Arguments
    /// * token - the operator token.
    /// # Returns
    /// * ((), right) - right prefix binding power of the operator.
    fn prefix_binding_power(token: Token) -> ((), u8) {
        match token {
            Token::Minus | Token::Plus => ((), 5),

            // basically unreachable.
            t => panic!("Cannot get infix binding power of {t}"),
        }
    }
}
