//pub trait Operator<O> {
//    fn apply(&self, left_one: O, right_one: O) -> O;
//}
//
//pub struct Add {}
//
//pub struct IntegerOperand {
//    pub value: i32
//}
//
//impl Operator<IntegerOperand> for Add {
//
//    fn apply(&self, left_one: IntegerOperand, right_one: IntegerOperand) -> IntegerOperand {
//        IntegerOperand {
//            value: left_one.value + right_one.value,
//        }
//    }
//}
use num::Num;

pub trait Monad<N: Num> {
    fn apply(self) -> N;
}

pub struct Operand<N: Num> {
    value: N
}

impl<N: Num> Monad<N> for Operand<N> {

    fn apply(self) -> N {
        self.value
    }
}

pub fn sum<N: Num>(operand: Vec<Operand<N>>) -> Operand<N> {
    let mut result = N::zero();
    for op in operand {
        result = result + op.apply();
    };
    Operand {
        value: result
    }
}

pub fn multiply<N: Num>(operand: Vec<Operand<N>>) -> Operand<N> {
    let mut result = N::one();
    for op in operand {
        result = result * op.apply();
    };
    Operand {
        value: result
    }
}

pub fn divide<N: Num>(dividend: Operand<N>, divisor: Operand<N>) -> Operand<N> {
    Operand {
        value: dividend.apply() / divisor.apply()
    }
}

#[test]
fn operand_ships_a_value() {
    let a = Operand {
        value: 10
    };
    assert_eq!(a.value, 10);
}

#[test]
fn add_evaluates_two_integer_operand() {
    let a = Operand {
        value: 1
    };
    let b = Operand {
        value: 2
    };
    assert_eq!(sum(vec![a, b]).apply(), 3);
}

#[test]
fn add_evaluates_negative_operands() {
    assert_eq!(sum(vec![
        Operand { value: 5 },
        Operand { value: -8 }
    ]).apply(), -3);
}

#[test]
fn mutliply_evaluates_two_operands() {
    assert_eq!(
        multiply(vec![
            Operand { value: 5 },
            Operand { value: 7 }
        ]).apply(),
        35
    );
}

// 1 + (5 - 4) * 7 == SUM(1, MUL(MIN(5,4), 7))
#[test]
fn represent_complex_int_expression1() {
    assert_eq!(sum(vec![
        Operand { value: 1},
        multiply(vec![
                sum(vec![
                    Operand { value: 5 },
                    Operand { value: -4 }
                ]),
                Operand { value: 7 }
            ])
        ]).apply(), 8);
}

#[test]
fn divide_always_evaluates_two_operands() {
    assert_eq!(divide(
        divide(
            Operand { value: 27 },
            Operand { value: 3 }
        ),
        Operand { value: 3 }
    ).apply(), 3);
}

// 1 + 3 + 5 + 6 - 2 + 4 == SUM(1, 3, 5, 6, -2, 4)
#[test]
fn represent_multiple_int_sum() {
    assert_eq!(sum(vec![
        Operand { value: 1 },
        Operand { value: 3 },
        Operand { value: 5 },
        Operand { value: 6 },
        Operand { value: -2 },
        Operand { value: 4 },
    ]).apply(), 17);
}
