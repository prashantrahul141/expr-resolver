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
/// Result<f_64, String>
/// # Examples
/// ```
/// use expr_resolver::resolve;
/// // gives syntax error.
/// assert!(matches!(resolve("2)2".to_string()), Err(String)));
/// ```
pub fn resolve(input_string: String) -> Result<f64, String> {
    // create a new lexer
    let mut lexer = Lexer::new(input_string);
    // and parse input string into tokens.
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
            // we create a new ast walk interpreter.
            let interpreter = Interpreter::new();
            // walk the ast and return the resulting Result<f64, String>
            return interpreter.walk_ast(&ast);
        }
        // otherwise we return the error we got from the parser.
        Err(err) => return Err(err),
    };
}
