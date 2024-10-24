mod bindings;

use bindings::exports::docs::calculator::calculate::{Guest, Op};


use bindings::docs::calculator::add::add;
use bindings::docs::calculator::sub::sub;
use bindings::docs::calculator::mul::mul;
use bindings::docs::calculator::div::div;
use bindings::docs::calculator::exp::exp;
use bindings::docs::calculator::square::square;

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
            Op::Sub => sub(x, y),
            Op::Mul => mul(x, y),
            Op::Div => div(x, y),
            Op::Exp => exp(x, y),
            Op::Square => square(x),
        }
    }
}

bindings::export!(Component with_types_in bindings);
