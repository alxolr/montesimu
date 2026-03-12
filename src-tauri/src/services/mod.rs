pub mod greet;
pub mod histogram;
pub mod simulation;

use shaku::module;

use crate::services::{
    greet::GreetServiceImpl, 
    histogram::HistogramServiceImpl,
    simulation::SimulationServiceImpl,
};

module! {
    pub Container {
        components = [GreetServiceImpl, HistogramServiceImpl, SimulationServiceImpl],
        providers = []
    }
}
