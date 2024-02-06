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

// top level lexer.
pub struct Lexer {
    // source string.
    pub source_string: String,
    // source string as vec of chars.
    pub source_chars: Vec<char>,
    // start of current token.
    pub start: usize,
    // current position of the character in consideration.
    pub current: usize,
    // Vec of tokens.
    pub tokens: Vec<Token>,
    // len of the input string.
    pub len: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            source_chars: input.chars().collect(),
            source_string: input.clone(),
            start: 0,
            current: 0,
            len: input.len(),
            tokens: vec![],
        }
    }

}
