use calculator::lexer::{Lexer, Token};

#[test]
fn test_empty() {
    let source = "";
    let mut lexer = Lexer::new(&source);
    assert!(lexer.next().is_none());
}

#[test]
fn test_empty_whitespace() {
    let source = " \t\r\n ";
    let mut lexer = Lexer::new(&source);
    assert!(lexer.next().is_none());
}

#[test]
fn test_simple_token() {
    let source = "!";
    let mut lexer = Lexer::new(&source);
    assert_eq!(lexer.next().unwrap().0, Token::Not);
    assert!(lexer.next().is_none());
}

#[test]
fn test_combined_token() {
    let source = "!=";
    let mut lexer = Lexer::new(&source);
    assert_eq!(lexer.next().unwrap().0, Token::Ne);
    assert!(lexer.next().is_none());
}

#[test]
fn test_number_token() {
    let source = "234";
    let mut lexer = Lexer::new(&source);
    assert_eq!(lexer.next().unwrap().0, Token::Int(234));
    assert!(lexer.next().is_none());
}

#[test]
fn test_multiple_tokens() {
    let source = "(1 + 2) - 3";
    let mut lexer = Lexer::new(&source);
    assert_eq!(lexer.next().unwrap().0, Token::LParen);
    assert_eq!(lexer.next().unwrap().0, Token::Int(1));
    assert_eq!(lexer.next().unwrap().0, Token::Add);
    assert_eq!(lexer.next().unwrap().0, Token::Int(2));
    assert_eq!(lexer.next().unwrap().0, Token::RParen);
    assert_eq!(lexer.next().unwrap().0, Token::Sub);
    assert_eq!(lexer.next().unwrap().0, Token::Int(3));
    assert!(lexer.next().is_none());
}
