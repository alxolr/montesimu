use shaku::HasComponent;
use tauri::State;

use crate::{
    dtos::histogram::HistogramBin,
    services::{histogram::HistogramService, Container},
};

#[tauri::command]
pub fn generate_histogram(
    state: State<'_, Container>,
    dataset: Vec<f64>,
    num_bins: usize,
) -> Vec<HistogramBin> {
    let service: &dyn HistogramService = state.resolve_ref();

    service.calculate(&dataset, num_bins)
}
