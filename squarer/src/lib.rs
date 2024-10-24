mod bindings;

use crate::bindings::exports::docs::calculator::square::Guest;

struct Component;

impl Guest for Component {
    fn square(a: u32) -> u32 {
        a * a
    }
}

bindings::export!(Component with_types_in bindings);
