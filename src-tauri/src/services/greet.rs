use shaku::{Component, Interface};

pub trait GreetService: Interface {
    fn greet(&self, name: &str) -> String;
}

#[derive(Component)]
#[shaku(interface = GreetService)]
pub struct GreetServiceImpl;

impl GreetService for GreetServiceImpl {
    fn greet(&self, name: &str) -> String {
        format!("Hello {name} from Shaku service")
    }
}
