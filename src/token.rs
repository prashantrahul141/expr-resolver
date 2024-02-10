// List of all the tokens possible.
#[derive(Debug, Clone, Copy)]
pub enum Token {
    // we store numbers as rust's f64.
    Number(f64),
    // operators.
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    // Grouping
    LeftParen,
    RightParen,
    // end of file for ease of use.
    Eof,
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Token::Number(n) => write!(f, " {n} "),
            Token::Plus => write!(f, " + "),
            Token::Minus => write!(f, " - "),
            Token::Star => write!(f, " * "),
            Token::Slash => write!(f, " / "),
            Token::Bang => write!(f, " ! "),
            Token::Eof => write!(f, " EOF "),
            Token::LeftParen => write!(f, " ( "),
            Token::RightParen => write!(f, " ) "),
        }
    }
}
