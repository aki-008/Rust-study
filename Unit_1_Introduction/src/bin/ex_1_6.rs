// 🏋️ Exercise: Shape Area Calculator (click to expand)
// Challenge: Define an enum Shape with variants Circle(f64) (radius), Rectangle(f64, f64) (width, height), and Triangle(f64, f64) (base, height). Implement a method fn area(&self) -> f64 using match. Create one of each and print the area.

#![allow(unused)]

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(b, h) => 0.5 * b * h,
        }
    }
}
fn main() {
    // let circle = Shape::Circle(5.0);
    // let rectangle = Shape::Rectangle(4.0, 6.0);
    // let triangle = Shape::Triangle(5.0, 8.0);

    // println!("Circle area: {}", area(&circle));
    // println!("Rectangle area: {}", area(&rectangle));
    // println!("Triangle area: {}", area(&triangle));

    let shapes = [
        Shape::Circle(5.0),
        Shape::Rectangle(5.0, 7.0),
        Shape::Triangle(5.0, 8.0),
    ];

    for shape in &shapes {
        println!("Areas: {:.2}", shape.area());
    }
}
