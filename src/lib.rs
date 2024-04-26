pub trait Operator<O> {
    fn apply(&self, left_one: O, right_one: O) -> O;
}

pub struct Add {}

pub struct IntegerOperand {
    pub value: i32
}

impl Operator<IntegerOperand> for Add {

    fn apply(&self, left_one: IntegerOperand, right_one: IntegerOperand) -> IntegerOperand {
        IntegerOperand {
            value: left_one.value + right_one.value,
        }
    }
}