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

    // public function which starts parsing.
    pub fn parse(&mut self) {
        while !self.is_at_end() {
            self.parse_token();
            self.start = self.current;
        }

        self.add_token(Token::Eof);
    }

    // parses one token at at time.
    fn parse_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '+' => self.add_token(Token::Add),
            '-' => self.add_token(Token::Subtract),
            '*' => self.add_token(Token::Multiply),
            '/' => self.add_token(Token::Divide),
            _ => {
                if current_char.is_ascii_digit() {
                    self.parser_number();
                } else {
                    panic!("unsupported character in input.");
                }
            }
        }
    }

    // parses a number.
    fn parser_number(&mut self) {
        while self.look_ahead().is_ascii_digit() {
            self.advance();
        }

        if self.look_ahead() == '.' && self.look_ahead_twice().is_ascii_digit() {
            self.advance();
            while self.look_ahead().is_ascii_digit() {
                self.advance();
            }
        }

        let number_literal = self.source_string[self.start..self.current]
            .to_string()
            .parse::<f64>();

        match number_literal {
            Ok(number_literal) => self.add_token(Token::Number(number_literal)),
            Err(_) => panic!("Failed to parse number literal as f64"),
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
