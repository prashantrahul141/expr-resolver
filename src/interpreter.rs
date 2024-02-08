use crate::ast::AST;

pub struct Interpreter {
    pub ast: AST,
}

impl Interpreter {
    pub fn new(ast: AST) -> Self {
        Self { ast }
    }
}
