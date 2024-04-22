pub trait Operator<O> {
    fn apply(&self, left_one: O, right_one: O) -> O;
}

fn main() {
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

struct Add {}

struct IntegerOperand {
    pub value: i32
}

impl Operator<IntegerOperand> for Add {

    fn apply(&self, left_one: IntegerOperand, right_one: IntegerOperand) -> IntegerOperand {
        IntegerOperand {
            value: left_one.value + right_one.value,
        }
    }
}