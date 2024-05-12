use rustcalc::*;

pub mod plugloader;

fn main() {
    println!("Rust calc!");

    plugloader::load_modules_from("/tmp/rustcalcmods");
    let a = IntegerOperand {
        value: 10
    };
    let b: IntegerOperand = IntegerOperand {
        value: 3
    };
    let c = Add {};
    println!("a + b = {}", c.apply(a, b).value);
}