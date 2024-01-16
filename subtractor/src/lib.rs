cargo_component_bindings::generate!();
use bindings::exports::docs::calculator::sub::Guest;

struct Component;

impl Guest for Component {
    fn sub(a: u32, b: u32) -> u32 {
        a - b
    }
}
