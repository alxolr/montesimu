use std::collections::HashMap;
use super::parser::{Expr, Operator};

/// Evaluator for mathematical expressions with variable and intermediate expression values
pub struct Evaluator {
    values: HashMap<String, f64>,
}

impl Evaluator {
    /// Creates a new evaluator with the given initial values
    pub fn new(values: HashMap<String, f64>) -> Self {
        Evaluator { values }
    }

    /// Creates a new evaluator with no initial values
    pub fn empty() -> Self {
        Evaluator {
            values: HashMap::new(),
        }
    }

    /// Adds or updates a value in the evaluator's context
    pub fn add_value(&mut self, name: String, value: f64) {
        self.values.insert(name, value);
    }

    /// Evaluates an expression using the current value context
    pub fn evaluate(&self, expr: &Expr) -> Result<f64, String> {
        match expr {
            Expr::Number(n) => Ok(*n),
            
            Expr::Identifier(id) => {
                self.values.get(id).copied().ok_or_else(|| {
                    format!("Undefined identifier: '{}'", id)
                })
            }
            
            Expr::BinaryOp { op, left, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.evaluate_binary_op(op, left_val, right_val)
            }
        }
    }

    /// Evaluates a binary operation with two operands
    fn evaluate_binary_op(&self, op: &Operator, left: f64, right: f64) -> Result<f64, String> {
        match op {
            Operator::Add => Ok(left + right),
            Operator::Subtract => Ok(left - right),
            Operator::Multiply => Ok(left * right),
            Operator::Divide => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::simulation::parser::parse_expression;

    #[test]
    fn test_evaluate_number() {
        let evaluator = Evaluator::empty();
        let expr = Expr::Number(42.5);
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 42.5);
    }

    #[test]
    fn test_evaluate_identifier() {
        let mut values = HashMap::new();
        values.insert("x".to_string(), 10.0);
        let evaluator = Evaluator::new(values);
        
        let expr = Expr::Identifier("x".to_string());
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 10.0);
    }

    #[test]
    fn test_evaluate_undefined_identifier() {
        let evaluator = Evaluator::empty();
        let expr = Expr::Identifier("undefined".to_string());
        
        let result = evaluator.evaluate(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Undefined identifier: 'undefined'");
    }

    #[test]
    fn test_evaluate_addition() {
        let evaluator = Evaluator::empty();
        let expr = Expr::BinaryOp {
            op: Operator::Add,
            left: Box::new(Expr::Number(5.0)),
            right: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 8.0);
    }

    #[test]
    fn test_evaluate_subtraction() {
        let evaluator = Evaluator::empty();
        let expr = Expr::BinaryOp {
            op: Operator::Subtract,
            left: Box::new(Expr::Number(10.0)),
            right: Box::new(Expr::Number(4.0)),
        };
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 6.0);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let evaluator = Evaluator::empty();
        let expr = Expr::BinaryOp {
            op: Operator::Multiply,
            left: Box::new(Expr::Number(6.0)),
            right: Box::new(Expr::Number(7.0)),
        };
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 42.0);
    }

    #[test]
    fn test_evaluate_division() {
        let evaluator = Evaluator::empty();
        let expr = Expr::BinaryOp {
            op: Operator::Divide,
            left: Box::new(Expr::Number(20.0)),
            right: Box::new(Expr::Number(4.0)),
        };
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 5.0);
    }

    #[test]
    fn test_evaluate_division_by_zero() {
        let evaluator = Evaluator::empty();
        let expr = Expr::BinaryOp {
            op: Operator::Divide,
            left: Box::new(Expr::Number(10.0)),
            right: Box::new(Expr::Number(0.0)),
        };
        
        let result = evaluator.evaluate(&expr);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_evaluate_with_variables() {
        let mut values = HashMap::new();
        values.insert("x".to_string(), 5.0);
        values.insert("y".to_string(), 3.0);
        let evaluator = Evaluator::new(values);
        
        // x + y
        let expr = Expr::BinaryOp {
            op: Operator::Add,
            left: Box::new(Expr::Identifier("x".to_string())),
            right: Box::new(Expr::Identifier("y".to_string())),
        };
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 8.0);
    }

    #[test]
    fn test_evaluate_complex_expression() {
        let mut values = HashMap::new();
        values.insert("a".to_string(), 2.0);
        values.insert("b".to_string(), 3.0);
        values.insert("c".to_string(), 4.0);
        let evaluator = Evaluator::new(values);
        
        // (a + b) * c = (2 + 3) * 4 = 20
        let expr = parse_expression("(a + b) * c").unwrap();
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_operator_precedence() {
        let evaluator = Evaluator::empty();
        
        // 2 + 3 * 4 = 2 + 12 = 14
        let expr = parse_expression("2 + 3 * 4").unwrap();
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 14.0);
    }

    #[test]
    fn test_add_value_dynamically() {
        let mut evaluator = Evaluator::empty();
        evaluator.add_value("x".to_string(), 10.0);
        
        let expr = Expr::Identifier("x".to_string());
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 10.0);
        
        // Update value
        evaluator.add_value("x".to_string(), 20.0);
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 20.0);
    }

    #[test]
    fn test_evaluate_with_intermediate_expressions() {
        let mut evaluator = Evaluator::empty();
        
        // Set variable values
        evaluator.add_value("x".to_string(), 5.0);
        evaluator.add_value("y".to_string(), 3.0);
        
        // Evaluate intermediate expression: temp = x + y
        let temp_expr = parse_expression("x + y").unwrap();
        let temp_result = evaluator.evaluate(&temp_expr).unwrap();
        evaluator.add_value("temp".to_string(), temp_result);
        
        // Evaluate target expression: temp * 2
        let target_expr = parse_expression("temp * 2").unwrap();
        assert_eq!(evaluator.evaluate(&target_expr).unwrap(), 16.0);
    }

    #[test]
    fn test_evaluate_overflow() {
        let evaluator = Evaluator::empty();
        let expr = Expr::BinaryOp {
            op: Operator::Multiply,
            left: Box::new(Expr::Number(f64::MAX)),
            right: Box::new(Expr::Number(2.0)),
        };
        
        let result = evaluator.evaluate(&expr).unwrap();
        assert!(result.is_infinite());
    }

    #[test]
    fn test_evaluate_very_large_expression() {
        let mut values = HashMap::new();
        for i in 0..100 {
            values.insert(format!("x{}", i), i as f64);
        }
        let evaluator = Evaluator::new(values);
        
        // Build a large expression: x0 + x1 + x2 + ... + x99
        let mut expr = Expr::Identifier("x0".to_string());
        for i in 1..100 {
            expr = Expr::BinaryOp {
                op: Operator::Add,
                left: Box::new(expr),
                right: Box::new(Expr::Identifier(format!("x{}", i))),
            };
        }
        
        // Sum of 0 to 99 = 4950
        assert_eq!(evaluator.evaluate(&expr).unwrap(), 4950.0);
    }
}
