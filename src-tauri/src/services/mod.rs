pub mod greet;
pub mod histogram;
pub mod simulation;
pub mod sim_engine;

use shaku::module;

use crate::services::{greet::GreetServiceImpl, histogram::HistogramServiceImpl};

module! {
    pub Container {
        components = [GreetServiceImpl, HistogramServiceImpl],
        providers = []
    }
}
