pub mod greet;

use shaku::module;

use crate::services::greet::GreetServiceImpl;

module! {
    pub Container {
        components = [GreetServiceImpl],
        providers = []
    }
}
