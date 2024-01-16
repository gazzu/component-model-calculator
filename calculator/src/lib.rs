cargo_component_bindings::generate!();

use bindings::exports::docs::calculator::calculate::{Guest, Op};


use bindings::docs::calculator::add::add;
use bindings::docs::calculator::sub::sub;
use bindings::docs::calculator::mul::mul;
use bindings::docs::calculator::div::div;

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
            Op::Sub => sub(x, y),
            Op::Mul => mul(x, y),
            Op::Div => div(x, y),
        }
    }
}
