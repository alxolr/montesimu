use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Distribution {
    Normal { mean: f64, std_dev: f64 },
    Lognormal { mean: f64, std_dev: f64 },
    Uniform { min: f64, max: f64 },
}
