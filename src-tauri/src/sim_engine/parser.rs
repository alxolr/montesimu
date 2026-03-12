use std::collections::HashSet;
use std::fmt;

/// Represents a token in the expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}

/// Tokenizes an expression string into a vector of tokens
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Operators and parentheses
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            // Numbers (including decimals)
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                let mut has_dot = false;

                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        chars.next();
                    } else if c == '.' && !has_dot {
                        has_dot = true;
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match num_str.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => return Err(format!("Invalid number: {}", num_str)),
                }
            }
            // Identifiers (alphanumeric, starting with letter or underscore)
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut id = String::new();

                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        id.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Identifier(id));
            }
            _ => {
                return Err(format!("Unexpected character: '{}'", ch));
            }
        }
    }

    Ok(tokens)
}

/// Represents a mathematical operator
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
        }
    }
}

/// Represents an expression node in the abstract syntax tree
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    BinaryOp {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::BinaryOp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

/// Extracts all unique identifiers from an expression
pub fn extract_identifiers(expr: &Expr) -> HashSet<String> {
    let mut identifiers = HashSet::new();
    extract_identifiers_recursive(expr, &mut identifiers);
    identifiers
}

fn extract_identifiers_recursive(expr: &Expr, identifiers: &mut HashSet<String>) {
    match expr {
        Expr::Number(_) => {}
        Expr::Identifier(id) => {
            identifiers.insert(id.clone());
        }
        Expr::BinaryOp { left, right, .. } => {
            extract_identifiers_recursive(left, identifiers);
            extract_identifiers_recursive(right, identifiers);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_display() {
        assert_eq!(format!("{}", Operator::Add), "+");
        assert_eq!(format!("{}", Operator::Subtract), "-");
        assert_eq!(format!("{}", Operator::Multiply), "*");
        assert_eq!(format!("{}", Operator::Divide), "/");
    }

    #[test]
    fn test_expr_display() {
        let expr = Expr::Number(42.0);
        assert_eq!(format!("{}", expr), "42");

        let expr = Expr::Identifier("x".to_string());
        assert_eq!(format!("{}", expr), "x");

        let expr = Expr::BinaryOp {
            op: Operator::Add,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        };
        assert_eq!(format!("{}", expr), "(1 + 2)");
    }

    #[test]
    fn test_extract_identifiers_empty() {
        let expr = Expr::Number(42.0);
        let ids = extract_identifiers(&expr);
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_extract_identifiers_single() {
        let expr = Expr::Identifier("x".to_string());
        let ids = extract_identifiers(&expr);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("x"));
    }

    #[test]
    fn test_extract_identifiers_multiple() {
        let expr = Expr::BinaryOp {
            op: Operator::Add,
            left: Box::new(Expr::Identifier("x".to_string())),
            right: Box::new(Expr::Identifier("y".to_string())),
        };
        let ids = extract_identifiers(&expr);
        assert_eq!(ids.len(), 2);
        assert!(ids.contains("x"));
        assert!(ids.contains("y"));
    }

    #[test]
    fn test_extract_identifiers_duplicates() {
        let expr = Expr::BinaryOp {
            op: Operator::Add,
            left: Box::new(Expr::Identifier("x".to_string())),
            right: Box::new(Expr::Identifier("x".to_string())),
        };
        let ids = extract_identifiers(&expr);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("x"));
    }

    // Tokenizer tests
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
}
