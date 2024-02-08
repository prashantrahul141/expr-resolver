use crate::{ast::AST, token::Token};

/// Tree walk interpreter.
pub struct Interpreter;

impl Interpreter {
    // A simple constructor.
    pub fn new() -> Self {
        Self
    }

    /// Public function to starting walking a AST.
    /// # Arguments
    /// * ast : Reference to the AST to walk.
    /// # Returns
    /// Result enum with the value of expression if interpretion was correct,
    /// otherwise string error.
    pub fn walk_ast(&self, ast: &AST) -> Result<f64, String> {
        match ast {
            // if the entire ast is just one token.
            AST::Node(number) => self.walk_node(&number),
            // walk the rest ast.
            AST::Con(operator, sub_tokens) => self.solve_expr(operator, sub_tokens),
        }
    }

    /// returns the inner valuue of a node, basically a Number Token.
    /// # Arguments
    /// * token - Reference to the token.
    /// # Returns
    /// Result with value of the number inside the token, otherwise error string.
    fn walk_node(&self, token: &Token) -> Result<f64, String> {
        match token {
            Token::Number(f) => Ok(f.clone()),
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
    fn solve_expr(&self, operator: &Token, sub_tokens: &Vec<AST>) -> Result<f64, String> {
        match sub_tokens.len() {
            // if there are two operands, the expression is binary.
            2 => return self.solve_binary(operator, sub_tokens),
            // if there is only one operand, the expression is unary.
            1 => return self.solve_unary(operator, sub_tokens),
            // everything else is unreal according to this interpreter.
            _ => return Err("Unrecognised number of operands.".to_string()),
        }
    }

    /// Solves a binary expression.
    /// # Arguments
    /// * token - Reference to the token.
    /// * sub_tokens - Reference to vector of ast inside the current node.
    /// # Returns
    /// Result with value after solving the binary expresion, otherwise error string.
    fn solve_binary(&self, operator: &Token, sub_tokens: &Vec<AST>) -> Result<f64, String> {
        // the left operand.
        let left = match self.walk_ast(&sub_tokens[0]) {
            Ok(left) => left,
            Err(e) => return Err(e),
        };

        // the right operand..
        let right = match self.walk_ast(&sub_tokens[1]) {
            Ok(right) => right,
            Err(e) => return Err(e),
        };

        // checking type of operator, and solving accordingly.
        match operator {
            Token::Plus => Ok(left + right),
            Token::Minus => Ok(left - right),
            Token::Star => Ok(left * right),
            Token::Slash => Ok(left / right),
            _ => return Err("Unrecognised binary operator.".to_string()),
            //
        }
    }

    /// Solves a binary expression.
    /// # Arguments
    /// * token - Reference to the token.
    /// * sub_tokens - Reference to vector of ast inside the current node.
    /// # Returns
    /// Result with value after solving the binary expresion, otherwise error string.
    fn solve_unary(&self, token: &Token, sub_tokens: &Vec<AST>) -> Result<f64, String> {
        // the only right operand.
        let right = match self.walk_ast(&sub_tokens[0]) {
            Ok(right) => right,
            Err(e) => return Err(e),
        };

        // checking type of operator and solving accordingly.
        match token {
            Token::Minus => return Ok(-right),
            _ => return Err("Unrecognised binary operator.".to_string()),
        }
    }
}
