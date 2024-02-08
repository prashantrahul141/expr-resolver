use expr_resolver::{lexer::Lexer, token::Token};

#[test]
fn basic() {
    let mut lexer = Lexer::new("2 + 2".to_string());
    lexer.scan();

    assert_eq!(lexer.len, 5);
    assert_eq!(lexer.tokens.len(), 4);
}

#[test]
fn basic_tokenization() {
    let mut lexer = Lexer::new("2 ++  2".to_string());
    lexer.scan();

    assert_eq!(lexer.tokens.len(), 5);

    match lexer.tokens[4] {
        Token::Number(n) => assert!(n == 2_f64, "Token is not 2."),
        _ => {
            assert!(!true, "Token is not number.");
        }
    }
    assert!(matches!(lexer.tokens[2], Token::Plus));
    assert!(matches!(lexer.tokens[3], Token::Plus));
}

#[test]
fn basic_number_scanning() {
    let mut lexer = Lexer::new("1414141 141.141 141".to_string());
    lexer.scan();

    match lexer.tokens[3] {
        Token::Number(n) => assert!(n == 1414141_f64, "Token is not 1414141."),
        _ => {
            assert!(!true, "Token is not number.");
        }
    }

    match lexer.tokens[2] {
        Token::Number(n) => assert!(n == 141.141_f64, "Token is not 1414141."),
        _ => {
            assert!(!true, "Token is not number.");
        }
    }
}

#[test]
fn error_flag() {
    let mut lexer = Lexer::new("2(2".to_string());
    lexer.scan();

    assert!(lexer.has_errors);
}
