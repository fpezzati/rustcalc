use rustcalc::*;

pub fn main() {
    println!("Rust calc!");
    let a = IntegerOperand {
        value: 10
    };
    let b: IntegerOperand = IntegerOperand {
        value: 3
    };
    let c = Add {};
    println!("a + b = {}", c.apply(a, b).value);
}