use serde::{Deserialize, Serialize};

use super::distribution::Distribution;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDefinition {
    pub variables: Vec<Variable>,
    pub intermediate_expressions: Vec<IntermediateExpression>,
    pub target_expression: String,
    pub iteration_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub distribution: Distribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntermediateExpression {
    pub name: String,
    pub expression: String,
}
