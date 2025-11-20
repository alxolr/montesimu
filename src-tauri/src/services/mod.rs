pub mod greet;
pub mod histogram;

use shaku::module;

use crate::services::{greet::GreetServiceImpl, histogram::HistogramServiceImpl};

module! {
    pub Container {
        components = [GreetServiceImpl, HistogramServiceImpl],
        providers = []
    }
}
