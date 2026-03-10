use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationResults {
    pub values: Vec<f64>,
    pub errors: Option<Vec<String>>,
}
