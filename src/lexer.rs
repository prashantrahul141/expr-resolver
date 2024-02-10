use crate::token::Token;

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
    // flag for errors.
    pub has_errors: bool,
}

impl Lexer {
    /// Constructor for lexer
    ///
    /// # Arguments
    /// * input - Input string to tokenize.
    ///
    /// # Returns
    /// * Lexer - new instance of lexer with prepopulated fields.
    pub fn new(input: String) -> Self {
        Self {
            source_chars: input.chars().collect(),
            source_string: input.clone(),
            start: 0,
            current: 0,
            len: input.len(),
            tokens: vec![],
            has_errors: false,
        }
    }

    /// Top level public function to start tokenizing the input string.
    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.scan_token();
            self.start = self.current;
        }

        // we add a EOF token at the end, comes handy when parsing.
        self.add_token(Token::Eof);

        // we reverse the entire tokens vec, because we will be
        // using pop function to retrieve one token at time, which
        // takes tokens from the end.
        self.tokens.reverse();
    }

    /// Internal function which parses one token at a time.
    fn scan_token(&mut self) {
        let current_char = self.advance();

        match current_char {
            '+' => self.add_token(Token::Plus),
            '-' => self.add_token(Token::Minus),
            '*' => self.add_token(Token::Star),
            '/' => self.add_token(Token::Slash),
            '!' => self.add_token(Token::Bang),
            ' ' | '\t' | '\r' => {}
            _ => {
                if current_char.is_ascii_digit() {
                    self.scan_number();
                } else {
                    self.has_errors = true;
                }
            }
        }
    }

    /// Scans a number type of token.
    fn scan_number(&mut self) {
        while self.look_ahead().is_ascii_digit() {
            self.advance();
        }

        // for floating point numbers.
        if self.look_ahead() == '.' && self.look_ahead_twice().is_ascii_digit() {
            self.advance();
            while self.look_ahead().is_ascii_digit() {
                self.advance();
            }
        }

        // we take literal string of the number and parse it into rust's f64.
        let number_literal = self.source_string[self.start..self.current]
            .to_string()
            .parse::<f64>();

        match number_literal {
            Ok(number_literal) => self.add_token(Token::Number(number_literal)),
            // probably never going to happen but still you never know.
            Err(_) => panic!("Failed to parse number literal as f64"),
        }
    }

    /// consumes current character and returns it.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_chars[self.current - 1]
    }

    /// returns current character but doesn't consume it.
    fn look_ahead(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source_chars[self.current]
    }

    /// returns next character but doesn't consume it.
    fn look_ahead_twice(&mut self) -> char {
        if self.current + 1 >= self.len {
            return '\0';
        }
        self.source_chars[self.current + 1]
    }

    /// returns the next token, and also consumes it.
    pub fn next_token(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    /// returns the next token, but doesn't consume it.
    pub fn peek(&self) -> Token {
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
