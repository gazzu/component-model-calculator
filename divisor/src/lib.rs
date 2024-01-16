cargo_component_bindings::generate!();
use bindings::exports::docs::calculator::div::Guest;

struct Component;

impl Guest for Component {
    fn div(a: u32, b: u32) -> u32 {
        a / b
    }
}
