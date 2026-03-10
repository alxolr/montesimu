use serde::{Deserialize, Serialize};

use super::distribution::Distribution;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDefinition {
    pub variables: Vec<Variable>,
    pub constants: Vec<Constant>,
    pub expression: String,
    pub iteration_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub distribution: Distribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub value: f64,
}
