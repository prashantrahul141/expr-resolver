pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

/// Takes mathematical expression as string, resolves it.
/// # Arguments
/// * input_string
/// # Returns
/// Result enum with solved value or incase of an error, error string.
/// # Examples
/// ```
/// use expr_solver::resolve;
///
/// // simple binary expression.
/// assert_eq!(resolve("2+2".to_string()), Ok(4.0));
///
/// // follows precendence, 2 + (2 * 2) and NOT (2 + 2) * 2
/// assert_eq!(resolve("2+2*2".to_string()), Ok(6.0));
///
/// // unary expression.
/// assert_eq!(resolve("-2".to_string()), Ok(-2.0));
///
/// // even chain them. -(-2)
/// assert_eq!(resolve("--2".to_string()), Ok(2.0));
///
/// // binary and unary in one expression.
/// assert_eq!(resolve("2+-2".to_string()), Ok(0.0));
///
/// // gives syntax error.
/// assert!(matches!(resolve("2)2".to_string()), Err(String)));
/// ```
pub fn resolve(input_string: String) -> Result<f64, String> {
    // create a new lexer
    // and parse input string into tokens.
    let mut lexer = Lexer::new(input_string);
    lexer.scan();

    if lexer.has_errors {
        return Err("Found lexical error(s) in the expression.".to_string());
    }

    // create a new parser
    let mut parser = Parser::new(&mut lexer);
    // and parse tokens into AST.
    let ast = parser.parse();

    // if the ast is correct.
    match ast {
        Ok(ast) => {
            // we walk the ast with our interpreter.
            Interpreter::walk_ast(&ast)
        }
        // otherwise we return the error we got from the parser.
        Err(err) => Err(err),
    }
}
