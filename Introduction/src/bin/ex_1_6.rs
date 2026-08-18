#![allow(unused)]

use crate::Shape::Circle;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(b, h) => 0.5 * b * h,
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let triangle = Shape::Triangle(5.0, 8.0);

    println!("Circle area: {}", area(&circle));
    println!("Rectangle area: {}", area(&rectangle));
    println!("Triangle area: {}", area(&triangle));
}
