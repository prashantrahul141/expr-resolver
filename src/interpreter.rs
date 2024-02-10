use crate::{ast::AST, token::Token, utils::factorial};

/// Tree walk interpreter.
pub struct Interpreter;

impl Interpreter {
    /// Public function to starting walking a AST.
    /// # Arguments
    /// * ast : Reference to the AST to walk.
    /// # Returns
    /// Result enum with the value of expression if interpretion was correct,
    /// otherwise string error.
    pub fn walk_ast(ast: &AST) -> Result<f64, String> {
        match ast {
            // if the entire ast is just one token.
            AST::Node(number) => Interpreter::walk_node(number),
            // walk the rest ast.
            AST::Con(operator, sub_tokens) => Interpreter::solve_expr(operator, sub_tokens),
        }
    }

    /// returns the inner valuue of a node, basically a Number Token.
    /// # Arguments
    /// * token - Reference to the token.
    /// # Returns
    /// Result with value of the number inside the token, otherwise error string.
    fn walk_node(token: &Token) -> Result<f64, String> {
        match token {
            Token::Number(f) => Ok(*f),
            _ => Err("Unrecognised node token.".to_string()),
        }
    }

    /// This is nothing but a wrapper which calls functions required
    /// by recognising the type of expression from the length of operands.
    /// # Arguments
    /// * token - Reference to the token.
    /// * sub_tokens - Reference to vector of ast inside the current node.
    /// # Returns
    /// The Result returned by the respective called function.
    fn solve_expr(operator: &Token, sub_tokens: &[AST]) -> Result<f64, String> {
        match sub_tokens.len() {
            // if there are two operands, the expression is binary.
            2 => Interpreter::solve_binary(operator, sub_tokens),
            // if there is only one operand, the expression is unary.
            1 => Interpreter::solve_unary(operator, sub_tokens),
            // everything else is unreal according to this interpreter.
            _ => Err("Unrecognised number of operands.".to_string()),
        }
    }

    /// Solves a binary expression.
    /// # Arguments
    /// * token - Reference to the token.
    /// * sub_tokens - Reference to vector of ast inside the current node.
    /// # Returns
    /// Result with value after solving the binary expresion, otherwise error string.
    fn solve_binary(operator: &Token, sub_tokens: &[AST]) -> Result<f64, String> {
        // the left operand.
        let left = match Interpreter::walk_ast(&sub_tokens[0]) {
            Ok(left) => left,
            Err(e) => return Err(e),
        };

        // the right operand..
        let right = match Interpreter::walk_ast(&sub_tokens[1]) {
            Ok(right) => right,
            Err(e) => return Err(e),
        };

        log::trace!("Solving binary left={left}  operator={operator} right={right}");
        // checking type of operator, and solving accordingly.
        match operator {
            Token::Plus => Ok(left + right),
            Token::Minus => Ok(left - right),
            Token::Star => Ok(left * right),
            Token::Slash => Ok(left / right),
            _ => Err("Unrecognised binary operator.".to_string()),
            //
        }
    }

    /// Solves a binary expression.
    /// # Arguments
    /// * token - Reference to the token.
    /// * sub_tokens - Reference to vector of ast inside the current node.
    /// # Returns
    /// Result with value after solving the binary expresion, otherwise error string.
    fn solve_unary(operator: &Token, sub_tokens: &[AST]) -> Result<f64, String> {
        // the only right operand.
        let right = match Interpreter::walk_ast(&sub_tokens[0]) {
            Ok(right) => right,
            Err(e) => return Err(e),
        };

        log::trace!("Solving binary operator={operator} right={right}");
        // checking type of operator and solving accordingly.
        match operator {
            Token::Minus => Ok(-right),
            Token::Bang => Ok(factorial(right)),
            _ => Err("Unrecognised binary operator.".to_string()),
        }
    }
}
