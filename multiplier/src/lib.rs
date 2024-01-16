cargo_component_bindings::generate!();
use bindings::exports::docs::calculator::mul::Guest;

struct Component;

impl Guest for Component {
    fn mul(a: u32, b: u32) -> u32 {
        a * b
    }
}
