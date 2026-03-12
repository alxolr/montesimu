use std::fmt;
use super::lexer::{Token, tokenize};

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
