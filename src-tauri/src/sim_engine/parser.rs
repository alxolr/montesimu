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
/// Builds a dependency graph showing which expressions reference which identifiers
///
/// # Arguments
/// * `intermediate_exprs` - Map of intermediate expression names to their parsed ASTs
/// * `target_expr` - The target expression AST
///
/// # Returns
/// A HashMap where keys are expression names and values are sets of identifiers they depend on
/// The target expression is represented with the key "__target__"
pub fn build_dependency_graph(
    intermediate_exprs: &std::collections::HashMap<String, Expr>,
    target_expr: &Expr,
) -> std::collections::HashMap<String, HashSet<String>> {
    use std::collections::HashMap;

    let mut graph = HashMap::new();

    // Add dependencies for each intermediate expression
    for (name, expr) in intermediate_exprs {
        let dependencies = extract_identifiers(expr);
        graph.insert(name.clone(), dependencies);
    }

    // Add dependencies for target expression
    let target_dependencies = extract_identifiers(target_expr);
    graph.insert("__target__".to_string(), target_dependencies);

    graph
}
/// Performs topological sort on the dependency graph to determine evaluation order
///
/// # Arguments
/// * `intermediate_exprs` - Map of intermediate expression names to their parsed ASTs
/// * `target_expr` - The target expression AST
///
/// # Returns
/// A vector of expression names in the order they should be evaluated
/// Returns an error if circular dependencies are detected
///
/// # Example
/// If expr1 depends on x, expr2 depends on expr1, and target depends on expr2,
/// the evaluation order would be: ["expr1", "expr2"]
pub fn topological_sort(
    intermediate_exprs: &std::collections::HashMap<String, Expr>,
    target_expr: &Expr,
) -> Result<Vec<String>, String> {
    use std::collections::{HashMap, HashSet, VecDeque};

    // Build the dependency graph
    let dep_graph = build_dependency_graph(intermediate_exprs, target_expr);

    // Separate variables from expressions
    let expr_names: HashSet<String> = intermediate_exprs.keys().cloned().collect();

    // Build adjacency list (only for expressions, not variables)
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();

    // Initialize in-degree for all expressions
    for name in &expr_names {
        in_degree.insert(name.clone(), 0);
        adj_list.insert(name.clone(), Vec::new());
    }

    // Build adjacency list and calculate in-degrees
    for (expr_name, dependencies) in &dep_graph {
        // Skip target expression
        if expr_name == "__target__" {
            continue;
        }

        for dep in dependencies {
            // Only consider dependencies on other expressions (not variables)
            if expr_names.contains(dep) {
                // dep -> expr_name (dep must be evaluated before expr_name)
                adj_list.get_mut(dep).unwrap().push(expr_name.clone());
                *in_degree.get_mut(expr_name).unwrap() += 1;
            }
        }
    }

    // Kahn's algorithm for topological sort
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut result: Vec<String> = Vec::new();

    // Start with expressions that have no dependencies on other expressions
    for (name, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(name.clone());
        }
    }

    while let Some(current) = queue.pop_front() {
        result.push(current.clone());

        // Reduce in-degree for all neighbors
        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors {
                let degree = in_degree.get_mut(neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    // Check for circular dependencies
    if result.len() != expr_names.len() {
        // Find the expressions involved in the cycle
        let sorted_set: HashSet<String> = result.iter().cloned().collect();
        let cycle_exprs: Vec<String> = expr_names
            .iter()
            .filter(|name| !sorted_set.contains(*name))
            .cloned()
            .collect();

        return Err(format!(
            "Circular dependency detected among expressions: {}",
            cycle_exprs.join(", ")
        ));
    }

    Ok(result)
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

    // Dependency graph tests
    #[test]
    fn test_build_dependency_graph_empty() {
        use std::collections::HashMap;
        
        let intermediate_exprs = HashMap::new();
        let target_expr = parse_expression("42").unwrap();
        
        let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
        
        // Should only have target expression
        assert_eq!(graph.len(), 1);
        assert!(graph.contains_key("__target__"));
        assert_eq!(graph.get("__target__").unwrap().len(), 0);
    }

    #[test]
    fn test_build_dependency_graph_target_with_variables() {
        use std::collections::HashMap;
        
        let intermediate_exprs = HashMap::new();
        let target_expr = parse_expression("x + y").unwrap();
        
        let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
        
        assert_eq!(graph.len(), 1);
        let target_deps = graph.get("__target__").unwrap();
        assert_eq!(target_deps.len(), 2);
        assert!(target_deps.contains("x"));
        assert!(target_deps.contains("y"));
    }

    #[test]
    fn test_build_dependency_graph_with_intermediate_expressions() {
        use std::collections::HashMap;
        
        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("expr1".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("expr2".to_string(), parse_expression("z * 2").unwrap());
        
        let target_expr = parse_expression("expr1 + expr2").unwrap();
        
        let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
        
        assert_eq!(graph.len(), 3);
        
        // Check expr1 dependencies
        let expr1_deps = graph.get("expr1").unwrap();
        assert_eq!(expr1_deps.len(), 2);
        assert!(expr1_deps.contains("x"));
        assert!(expr1_deps.contains("y"));
        
        // Check expr2 dependencies
        let expr2_deps = graph.get("expr2").unwrap();
        assert_eq!(expr2_deps.len(), 1);
        assert!(expr2_deps.contains("z"));
        
        // Check target dependencies
        let target_deps = graph.get("__target__").unwrap();
        assert_eq!(target_deps.len(), 2);
        assert!(target_deps.contains("expr1"));
        assert!(target_deps.contains("expr2"));
    }

    #[test]
    fn test_build_dependency_graph_mixed_references() {
        use std::collections::HashMap;
        
        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("expr1".to_string(), parse_expression("x * 2").unwrap());
        
        // Target references both a variable and an intermediate expression
        let target_expr = parse_expression("expr1 + y").unwrap();
        
        let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
        
        assert_eq!(graph.len(), 2);
        
        // Check expr1 dependencies
        let expr1_deps = graph.get("expr1").unwrap();
        assert_eq!(expr1_deps.len(), 1);
        assert!(expr1_deps.contains("x"));
        
        // Check target dependencies (should have both expr1 and y)
        let target_deps = graph.get("__target__").unwrap();
        assert_eq!(target_deps.len(), 2);
        assert!(target_deps.contains("expr1"));
        assert!(target_deps.contains("y"));
    }

    #[test]
    fn test_build_dependency_graph_complex_expressions() {
        use std::collections::HashMap;
        
        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("a * z").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("w / 2").unwrap());
        
        let target_expr = parse_expression("(b + c) * a").unwrap();
        
        let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
        
        assert_eq!(graph.len(), 4);
        
        // Check a dependencies
        let a_deps = graph.get("a").unwrap();
        assert_eq!(a_deps.len(), 2);
        assert!(a_deps.contains("x"));
        assert!(a_deps.contains("y"));
        
        // Check b dependencies (references a and z)
        let b_deps = graph.get("b").unwrap();
        assert_eq!(b_deps.len(), 2);
        assert!(b_deps.contains("a"));
        assert!(b_deps.contains("z"));
        
        // Check c dependencies
        let c_deps = graph.get("c").unwrap();
        assert_eq!(c_deps.len(), 1);
        assert!(c_deps.contains("w"));
        
        // Check target dependencies
        let target_deps = graph.get("__target__").unwrap();
        assert_eq!(target_deps.len(), 3);
        assert!(target_deps.contains("b"));
        assert!(target_deps.contains("c"));
        assert!(target_deps.contains("a"));
    }

    #[test]
    fn test_build_dependency_graph_no_duplicates() {
        use std::collections::HashMap;
        
        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("expr1".to_string(), parse_expression("x + x + x").unwrap());
        
        let target_expr = parse_expression("expr1 + expr1").unwrap();
        
        let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
        
        // expr1 should only list x once
        let expr1_deps = graph.get("expr1").unwrap();
        assert_eq!(expr1_deps.len(), 1);
        assert!(expr1_deps.contains("x"));
        
        // target should only list expr1 once
        let target_deps = graph.get("__target__").unwrap();
        assert_eq!(target_deps.len(), 1);
        assert!(target_deps.contains("expr1"));
    }
    // Topological sort tests
    #[test]
    fn test_topological_sort_no_expressions() {
        use std::collections::HashMap;

        let intermediate_exprs = HashMap::new();
        let target_expr = parse_expression("x + y").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        // No intermediate expressions, so result should be empty
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_topological_sort_single_expression() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("expr1".to_string(), parse_expression("x + y").unwrap());

        let target_expr = parse_expression("expr1 * 2").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "expr1");
    }

    #[test]
    fn test_topological_sort_linear_dependency() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("a * 2").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("b + 5").unwrap());

        let target_expr = parse_expression("c / 3").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        assert_eq!(result.len(), 3);

        // a must come before b, b must come before c
        let a_pos = result.iter().position(|x| x == "a").unwrap();
        let b_pos = result.iter().position(|x| x == "b").unwrap();
        let c_pos = result.iter().position(|x| x == "c").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_topological_sort_independent_expressions() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("z * 2").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("w / 3").unwrap());

        let target_expr = parse_expression("a + b + c").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        // All three expressions are independent, so all should be in result
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        assert!(result.contains(&"c".to_string()));
    }

    #[test]
    fn test_topological_sort_diamond_dependency() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("a * 2").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("a * 3").unwrap());
        intermediate_exprs.insert("d".to_string(), parse_expression("b + c").unwrap());

        let target_expr = parse_expression("d / 2").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        assert_eq!(result.len(), 4);

        // a must come before b and c
        let a_pos = result.iter().position(|x| x == "a").unwrap();
        let b_pos = result.iter().position(|x| x == "b").unwrap();
        let c_pos = result.iter().position(|x| x == "c").unwrap();
        let d_pos = result.iter().position(|x| x == "d").unwrap();

        assert!(a_pos < b_pos);
        assert!(a_pos < c_pos);
        assert!(b_pos < d_pos);
        assert!(c_pos < d_pos);
    }

    #[test]
    fn test_topological_sort_circular_dependency_simple() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("b + 1").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("a + 1").unwrap());

        let target_expr = parse_expression("a + b").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Circular dependency detected"));
        assert!(err.contains("a") || err.contains("b"));
    }

    #[test]
    fn test_topological_sort_circular_dependency_complex() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("b + 1").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("c + 1").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("a + 1").unwrap());

        let target_expr = parse_expression("a").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Circular dependency detected"));
    }

    #[test]
    fn test_topological_sort_self_reference() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("a + 1").unwrap());

        let target_expr = parse_expression("a * 2").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Circular dependency detected"));
        assert!(err.contains("a"));
    }

    #[test]
    fn test_topological_sort_mixed_dependencies() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("a + z").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("w * 2").unwrap());
        intermediate_exprs.insert("d".to_string(), parse_expression("b + c").unwrap());

        let target_expr = parse_expression("d + a").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        assert_eq!(result.len(), 4);

        // a must come before b and d
        let a_pos = result.iter().position(|x| x == "a").unwrap();
        let b_pos = result.iter().position(|x| x == "b").unwrap();
        let d_pos = result.iter().position(|x| x == "d").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < d_pos);

        // c must come before d
        let c_pos = result.iter().position(|x| x == "c").unwrap();
        assert!(c_pos < d_pos);
    }

    #[test]
    fn test_topological_sort_only_variable_dependencies() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("z * w").unwrap());

        let target_expr = parse_expression("a + b").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        // Both expressions only depend on variables, so they can be in any order
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
    }

    #[test]
    fn test_topological_sort_partial_circular() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("c + 1").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("b + 1").unwrap());

        let target_expr = parse_expression("a + b").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr);

        // b and c form a cycle, so this should fail
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Circular dependency detected"));
    }

    #[test]
    fn test_topological_sort_long_chain() {
        use std::collections::HashMap;

        let mut intermediate_exprs = HashMap::new();
        intermediate_exprs.insert("a".to_string(), parse_expression("x + 1").unwrap());
        intermediate_exprs.insert("b".to_string(), parse_expression("a + 1").unwrap());
        intermediate_exprs.insert("c".to_string(), parse_expression("b + 1").unwrap());
        intermediate_exprs.insert("d".to_string(), parse_expression("c + 1").unwrap());
        intermediate_exprs.insert("e".to_string(), parse_expression("d + 1").unwrap());

        let target_expr = parse_expression("e").unwrap();

        let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();

        assert_eq!(result.len(), 5);
        assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
    }
}
