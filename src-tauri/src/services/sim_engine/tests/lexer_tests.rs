use crate::services::sim_engine::lexer::{Token, tokenize};

#[test]
fn test_tokenize_number() {
    let tokens = tokenize("42").unwrap();
    assert_eq!(tokens, vec![Token::Number(42.0)]);
}

#[test]
fn test_tokenize_decimal() {
    let tokens = tokenize("3.14").unwrap();
    assert_eq!(tokens, vec![Token::Number(3.14)]);
}

#[test]
fn test_tokenize_identifier() {
    let tokens = tokenize("x").unwrap();
    assert_eq!(tokens, vec![Token::Identifier("x".to_string())]);
}

#[test]
fn test_tokenize_identifier_with_numbers() {
    let tokens = tokenize("var123").unwrap();
    assert_eq!(tokens, vec![Token::Identifier("var123".to_string())]);
}

#[test]
fn test_tokenize_operators() {
    let tokens = tokenize("+ - * /").unwrap();
    assert_eq!(
        tokens,
        vec![Token::Plus, Token::Minus, Token::Star, Token::Slash]
    );
}

#[test]
fn test_tokenize_parentheses() {
    let tokens = tokenize("( )").unwrap();
    assert_eq!(tokens, vec![Token::LeftParen, Token::RightParen]);
}

#[test]
fn test_tokenize_simple_expression() {
    let tokens = tokenize("x + 5").unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Number(5.0)
        ]
    );
}

#[test]
fn test_tokenize_complex_expression() {
    let tokens = tokenize("(a + b) * 2.5").unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::LeftParen,
            Token::Identifier("a".to_string()),
            Token::Plus,
            Token::Identifier("b".to_string()),
            Token::RightParen,
            Token::Star,
            Token::Number(2.5)
        ]
    );
}

#[test]
fn test_tokenize_no_spaces() {
    let tokens = tokenize("x+y*2").unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Star,
            Token::Number(2.0)
        ]
    );
}

#[test]
fn test_tokenize_invalid_character() {
    let result = tokenize("x + @");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character"));
}

#[test]
fn test_tokenize_empty_string() {
    let tokens = tokenize("").unwrap();
    assert_eq!(tokens, vec![]);
}

#[test]
fn test_tokenize_whitespace_only() {
    let tokens = tokenize("   \t\n  ").unwrap();
    assert_eq!(tokens, vec![]);
}

#[test]
fn test_tokenize_invalid_char() {
    let result = tokenize("1 + @ + 2");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character"));
}
