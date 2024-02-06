// List of all the tokens possible.
#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Eof,
}

