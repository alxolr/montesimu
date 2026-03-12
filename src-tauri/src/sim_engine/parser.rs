use std::collections::HashSet;
use std::fmt;

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
}
