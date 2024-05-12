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

#[test]
fn operator_Add_evaluates_two_integer_operand() {
    let a = IntegerOperand {
        value: 1
    };
    let b = IntegerOperand {
        value: 2
    };
    let c = Add {};
    assert_eq!(c.apply(a, b).value, 3);
}