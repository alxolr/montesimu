use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramBin {
    pub label: String,
    pub count: usize,
    pub min: f64,
    pub max: f64,
}
