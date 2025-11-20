use shaku::{Component, Interface};

use crate::dtos::histogram::HistogramBin;

pub trait HistogramService: Interface {
    fn calculate(&self, dataset: &[f64], num_bins: usize) -> Vec<HistogramBin>;
}

#[derive(Component)]
#[shaku(interface = HistogramService)]
pub struct HistogramServiceImpl;

impl HistogramService for HistogramServiceImpl {
    fn calculate(&self, dataset: &[f64], num_bins: usize) -> Vec<HistogramBin> {
        if dataset.is_empty() || num_bins == 0 {
            return vec![];
        }

        let min_value = dataset.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_value = dataset.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        if min_value == max_value {
            return vec![HistogramBin {
                label: format!("{:.2}", min_value),
                count: dataset.len(),
                min: min_value,
                max: max_value,
            }];
        }

        let bin_width = (max_value - min_value) / num_bins as f64;
        let mut bins: Vec<HistogramBin> = Vec::new();

        // Create bins
        for i in 0..num_bins {
            let bin_min = min_value + (i as f64 * bin_width);
            let bin_max = if i == num_bins - 1 {
                max_value
            } else {
                bin_min + bin_width
            };

            bins.push(HistogramBin {
                label: format!("{:.2}-{:.2}", bin_min, bin_max),
                count: 0,
                min: bin_min,
                max: bin_max,
            });
        }

        // Count data points in each bin
        for &value in dataset {
            for (i, bin) in bins.iter_mut().enumerate() {
                if i == num_bins - 1 {
                    // Last bin includes the max value
                    if value >= bin.min && value <= bin.max {
                        bin.count += 1;
                        break;
                    }
                } else {
                    if value >= bin.min && value < bin.max {
                        bin.count += 1;
                        break;
                    }
                }
            }
        }

        bins
    }
}
