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

/// Parser for converting tokens into an AST
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse(&mut self) -> Result<Expr, String> {
        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        let expr = self.parse_additive()?;
        if self.position < self.tokens.len() {
            return Err(format!(
                "Unexpected token at position {}: {:?}",
                self.position,
                self.current()
            ));
        }
        Ok(expr)
    }

    // Parse addition and subtraction (lowest precedence)
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        while let Some(token) = self.current() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expr::BinaryOp {
                        op: Operator::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplicative()?;
                    left = Expr::BinaryOp {
                        op: Operator::Subtract,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse multiplication and division (higher precedence)
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.current() {
            match token {
                Token::Star => {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expr::BinaryOp {
                        op: Operator::Multiply,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_primary()?;
                    left = Expr::BinaryOp {
                        op: Operator::Divide,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Parse primary expressions (numbers, identifiers, parenthesized expressions)
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.current() {
            Some(Token::Number(n)) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            Some(Token::Identifier(id)) => {
                let identifier = id.clone();
                self.advance();
                Ok(Expr::Identifier(identifier))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_additive()?;
                match self.current() {
                    Some(Token::RightParen) => {
                        self.advance();
                        Ok(expr)
                    }
                    Some(token) => Err(format!(
                        "Expected closing parenthesis, found: {:?}",
                        token
                    )),
                    None => Err("Expected closing parenthesis, found end of expression".to_string()),
                }
            }
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Star) | Some(Token::Slash) => {
                Err(format!(
                    "Unexpected operator at position {}: {:?}",
                    self.position,
                    self.current()
                ))
            }
            Some(Token::RightParen) => {
                Err(format!("Unexpected closing parenthesis at position {}", self.position))
            }
            None => Err("Unexpected end of expression".to_string()),
        }
    }
}

/// Validates expression syntax and returns detailed error messages
pub fn validate_syntax(input: &str) -> Result<(), String> {
    // Check for balanced parentheses
    let mut paren_count = 0;
    let mut paren_positions = Vec::new();
    
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => {
                paren_count += 1;
                paren_positions.push((i, '('));
            }
            ')' => {
                paren_count -= 1;
                if paren_count < 0 {
                    return Err(format!("Unbalanced parentheses: unexpected ')' at position {}", i));
                }
                paren_positions.push((i, ')'));
            }
            _ => {}
        }
    }
    
    if paren_count > 0 {
        let unclosed = paren_positions.iter()
            .filter(|(_, ch)| *ch == '(')
            .map(|(pos, _)| pos.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!("Unbalanced parentheses: unclosed '(' at position(s): {}", unclosed));
    }
    
    // Try to parse the expression
    parse_expression(input)?;
    
    Ok(())
}

/// Parses an expression string into an AST
pub fn parse_expression(input: &str) -> Result<Expr, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    parser.parse()
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

    // Parser tests
    #[test]
    fn test_parse_number() {
        let expr = parse_expression("42").unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn test_parse_identifier() {
        let expr = parse_expression("x").unwrap();
        assert_eq!(expr, Expr::Identifier("x".to_string()));
    }

    #[test]
    fn test_parse_addition() {
        let expr = parse_expression("1 + 2").unwrap();
        match expr {
            Expr::BinaryOp { op, left, right } => {
                assert_eq!(op, Operator::Add);
                assert_eq!(*left, Expr::Number(1.0));
                assert_eq!(*right, Expr::Number(2.0));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_subtraction() {
        let expr = parse_expression("5 - 3").unwrap();
        match expr {
            Expr::BinaryOp { op, left, right } => {
                assert_eq!(op, Operator::Subtract);
                assert_eq!(*left, Expr::Number(5.0));
                assert_eq!(*right, Expr::Number(3.0));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication() {
        let expr = parse_expression("3 * 4").unwrap();
        match expr {
            Expr::BinaryOp { op, left, right } => {
                assert_eq!(op, Operator::Multiply);
                assert_eq!(*left, Expr::Number(3.0));
                assert_eq!(*right, Expr::Number(4.0));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division() {
        let expr = parse_expression("8 / 2").unwrap();
        match expr {
            Expr::BinaryOp { op, left, right } => {
                assert_eq!(op, Operator::Divide);
                assert_eq!(*left, Expr::Number(8.0));
                assert_eq!(*right, Expr::Number(2.0));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_operator_precedence() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let expr = parse_expression("2 + 3 * 4").unwrap();
        match expr {
            Expr::BinaryOp {
                op: Operator::Add,
                left,
                right,
            } => {
                assert_eq!(*left, Expr::Number(2.0));
                match *right {
                    Expr::BinaryOp {
                        op: Operator::Multiply,
                        left: mult_left,
                        right: mult_right,
                    } => {
                        assert_eq!(*mult_left, Expr::Number(3.0));
                        assert_eq!(*mult_right, Expr::Number(4.0));
                    }
                    _ => panic!("Expected multiplication on right side"),
                }
            }
            _ => panic!("Expected addition at top level"),
        }
    }

    #[test]
    fn test_parse_parentheses() {
        // (2 + 3) * 4 should parse as (2 + 3) * 4
        let expr = parse_expression("(2 + 3) * 4").unwrap();
        match expr {
            Expr::BinaryOp {
                op: Operator::Multiply,
                left,
                right,
            } => {
                match *left {
                    Expr::BinaryOp {
                        op: Operator::Add,
                        left: add_left,
                        right: add_right,
                    } => {
                        assert_eq!(*add_left, Expr::Number(2.0));
                        assert_eq!(*add_right, Expr::Number(3.0));
                    }
                    _ => panic!("Expected addition in parentheses"),
                }
                assert_eq!(*right, Expr::Number(4.0));
            }
            _ => panic!("Expected multiplication at top level"),
        }
    }

    #[test]
    fn test_parse_nested_parentheses() {
        let expr = parse_expression("((1 + 2) * 3)").unwrap();
        match expr {
            Expr::BinaryOp {
                op: Operator::Multiply,
                ..
            } => {}
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_parse_with_identifiers() {
        let expr = parse_expression("x + y * 2").unwrap();
        match expr {
            Expr::BinaryOp {
                op: Operator::Add,
                left,
                right,
            } => {
                assert_eq!(*left, Expr::Identifier("x".to_string()));
                match *right {
                    Expr::BinaryOp {
                        op: Operator::Multiply,
                        left: mult_left,
                        right: mult_right,
                    } => {
                        assert_eq!(*mult_left, Expr::Identifier("y".to_string()));
                        assert_eq!(*mult_right, Expr::Number(2.0));
                    }
                    _ => panic!("Expected multiplication"),
                }
            }
            _ => panic!("Expected addition"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let expr = parse_expression("(a + b) / (c - d)").unwrap();
        match expr {
            Expr::BinaryOp {
                op: Operator::Divide,
                ..
            } => {}
            _ => panic!("Expected division"),
        }
    }

    #[test]
    fn test_parse_empty_expression() {
        let result = parse_expression("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Empty expression"));
    }

    #[test]
    fn test_parse_unbalanced_parentheses() {
        let result = parse_expression("(1 + 2");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("closing parenthesis"));
    }

    #[test]
    fn test_parse_unexpected_token() {
        let result = parse_expression("1 + + 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_trailing_operator() {
        let result = parse_expression("1 + 2 +");
        assert!(result.is_err());
    }

    // Syntax error detection tests
    #[test]
    fn test_validate_syntax_valid() {
        assert!(validate_syntax("1 + 2").is_ok());
        assert!(validate_syntax("(a + b) * c").is_ok());
        assert!(validate_syntax("x / (y - 3)").is_ok());
    }

    #[test]
    fn test_validate_syntax_unbalanced_parens_missing_close() {
        let result = validate_syntax("(1 + 2");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unbalanced parentheses"));
        assert!(err.contains("unclosed"));
    }

    #[test]
    fn test_validate_syntax_unbalanced_parens_extra_close() {
        let result = validate_syntax("1 + 2)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unbalanced parentheses"));
        assert!(err.contains("unexpected ')'"));
    }

    #[test]
    fn test_validate_syntax_consecutive_operators() {
        let result = validate_syntax("1 + + 2");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unexpected operator"));
    }

    #[test]
    fn test_validate_syntax_operator_at_start() {
        let result = validate_syntax("+ 1 + 2");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unexpected operator"));
    }

    #[test]
    fn test_validate_syntax_operator_at_end() {
        let result = validate_syntax("1 + 2 *");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_syntax_empty_parentheses() {
        let result = validate_syntax("()");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_syntax_nested_unbalanced() {
        let result = validate_syntax("((1 + 2)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unbalanced parentheses"));
    }

    #[test]
    fn test_parse_error_missing_operand() {
        let result = parse_expression("1 +");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_double_operator() {
        let result = parse_expression("1 * / 2");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_only_operator() {
        let result = parse_expression("+");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_mismatched_parens() {
        let result = parse_expression("(1 + 2))");
        assert!(result.is_err());
    }

    #[test]
    fn test_tokenize_invalid_char() {
        let result = tokenize("1 + @ + 2");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character"));
    }
}
