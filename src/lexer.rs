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

    // consumes next character and returns it.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_chars[self.current - 1]
    }

    // returns next char but doesnt consume it.
    fn look_ahead(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source_chars[self.current]
    }

    // returns next + 1 but doesnt consume it.
    fn look_ahead_twice(&mut self) -> char {
        if self.current + 1 >= self.len {
            return '\0';
        }
        self.source_chars[self.current + 1]
    }

    // returns the next token, and also consumes it.
    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    // returns the next token, but doesnt consume it.
    fn peek(&self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }

    // checks if reached the end of the input string.
    fn is_at_end(&self) -> bool {
        self.current >= self.len
    }

    // helper function to add tokens.
    fn add_token(&mut self, token_type: Token) {
        self.tokens.push(token_type);
    }
}
