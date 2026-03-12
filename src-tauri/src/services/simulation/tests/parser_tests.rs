use crate::services::simulation::parser::{Expr, Operator, parse_expression, validate_syntax};

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
