fn main() {
    println!("{}", sum(12.00, 24.00));
    println!("{}", sub(12.00, 24.00));
    println!("{}", divide(12.00, 24.00));
    println!("{}", multiple(12.00, 24.00));
}
fn sum(num1: f32, num2: f32) -> f32 {
    num1 + num2
}
fn sub(num1: f32, num2: f32) -> f32 {
    num1 - num2
}
fn divide(num1: f32, num2: f32) -> f32 {
    num1 / num2
}
fn multiple(num1: f32, num2: f32) -> f32 {
    num1 * num2
}
