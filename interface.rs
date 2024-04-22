// Define the Plugin API trait
pub trait ArithmeticPlugin {
    fn add(&self, a: i32, b: i32) -> i32;
    fn subtract(&self, a: i32, b: i32) -> i32;
}
