use expr_resolver::{ast::AST, interpreter::Interpreter, token::Token};

#[test]
fn basic_walking() {
    let asts = vec![&AST::Node(Token::Number(2.0))];

    assert!(Interpreter::walk_ast(asts[0]) == Ok(2.0));
}
