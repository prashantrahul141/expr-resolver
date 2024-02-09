use expr_resolver::{ast::AST::*, interpreter::Interpreter, token::Token::*};

#[test]
fn basic_walking() {
    let asts = vec![
        Node(Number(2.0)),
        Con(Minus, vec![Node(Number(2_f64))]),
        Con(Minus, vec![Node(Number(2_f64)), Node(Number(2_f64))]),
        Con(Plus, vec![Node(Number(2_f64)), Node(Number(2_f64))]),
        Con(Slash, vec![Node(Number(2_f64)), Node(Number(2_f64))]),
        Con(Star, vec![Node(Number(3_f64)), Node(Number(2_f64))]),
    ];

    assert_eq!(Interpreter::walk_ast(&asts[0]), Ok(2.0));
    assert_eq!(Interpreter::walk_ast(&asts[1]), Ok(-2.0));
    assert_eq!(Interpreter::walk_ast(&asts[2]), Ok(0.0));
    assert_eq!(Interpreter::walk_ast(&asts[3]), Ok(4.0));
    assert_eq!(Interpreter::walk_ast(&asts[4]), Ok(1.0));
    assert_eq!(Interpreter::walk_ast(&asts[5]), Ok(6.0));
}
