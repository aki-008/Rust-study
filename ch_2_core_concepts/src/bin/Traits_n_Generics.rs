#![allow(unused)]

use std::hash::Hash;
fn rs_traits() {
    // Rust — traits make the "duck" contract explicit
    trait HasArea {
        fn area(&self) -> f64; // Any type that implements this trait has .area()
    }

    struct Circle {
        radius: f64,
    }
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    // The trait constraint is explicit — compiler checks at compile time
    fn total_area(shapes: &[&dyn HasArea]) -> f64 {
        shapes.iter().map(|s| s.area()).sum()
    }

    fn train_usage() {
        let shapes: Vec<&dyn HasArea> = vec![
            &Circle { radius: 5.0 },
            &Rectangle {
                width: 3.0,
                height: 4.0,
            },
        ];
        println!("{}", total_area(&shapes));
    }
}

fn main() {
    rs_traits();
}
