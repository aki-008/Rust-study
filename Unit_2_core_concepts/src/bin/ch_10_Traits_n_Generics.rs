#![allow(unused)]

use std::fmt;
use std::hash::Hash;

use serde_json::value;
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

// Structural typing

// Structural typing is a type system where compatibility between types is determined by their actual properties or shape, rather than their explicit names or declaration.

fn structural_typing() {
    trait Printable {
        fn to_string(&self) -> String;
    }

    struct User {
        name: String,
    }
    struct Product {
        name: String,
        price: f64,
    }

    impl Printable for User {
        fn to_string(&self) -> String {
            format!("User ({}", self.name)
        }
    }

    impl Printable for Product {
        fn to_string(&self) -> String {
            format!("Product({}, ${:?})", self.name, self.price)
        }
    }

    fn print_all(items: &[&dyn Printable]) {
        for item in items {
            println!("{}", item.to_string());
        }
    }
    // Object creation using User & product struct (python class equivalent)
    let user = User {
        name: "Alice".to_string(),
    };

    let product = Product {
        name: "Laptop".to_string(),
        price: 99.99,
    };

    // Both implement Printable, so both can be passed
    // as &dyn Printable.
    let items: &[&dyn Printable] = &[&user, &product];

    print_all(items);
}

fn type_safety() {
    // Rust — generics with trait bounds
    fn first<T>(items: &[T]) -> Option<&T> {
        items.first()
    }

    // With trait bounds — "T must implement these traits"
    fn average<T>(items: &[T]) -> f64
    where
        T: Into<f64> + Copy, // T must convert to f64 and be copyable
    {
        let sum: f64 = items.iter().map(|&x| x.into()).sum();
        sum / items.len() as f64
    }

    // Multiple bounds — "T must implement Display AND Debug AND Clone"
    fn log_and_clone<T: std::fmt::Display + std::fmt::Debug + Clone>(item: &T) -> T {
        println!("Display: {}", item);
        println!("Debug: {:?}", item);
        item.clone()
    }

    // Shorthand with impl Trait (for simple cases)
    fn print_it(item: &impl std::fmt::Display) {
        println!("{}", item);
    }
}

// Common Standard Library Traits

// These are Rust’s version of Python’s “dunder methods” — they define how types behave in common situations.

fn dunder_method() {
    // Debug — like __repr__ (auto-derivable)
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    // Now you can: println!("{:?}", point);

    // Display — like __str__ (must implement manually)
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let value = Point { x: 2.0, y: 3.0 };
    println!("{}", value);
}

fn comparison_traits() {
    // PartialEq — like __eq__
    // Eq — total equality (f64 is PartialEq but not Eq because NaN != NaN)
    // PartialOrd — like __lt__, __le__, etc.
    // Ord — total ordering

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    struct Student {
        name: String,
        grade: i32,
    }

    // Now students can be: compared, sorted, used as HashMap keys, cloned
    let mut students = vec![
        Student {
            name: "Charlie".into(),
            grade: 85,
        },
        Student {
            name: "Alice".into(),
            grade: 92,
        },
    ];
    students.sort(); // Uses Ord — sorts by name then grade (struct field order)
}

fn iter_trait() {
    struct Countdown {
        value: i32,
    }

    impl Iterator for Countdown {
        type Item = i32; // What the iterator yields

        fn next(&mut self) -> Option<Self::Item> {
            if self.value > 0 {
                self.value -= 1;
                Some(self.value + 1)
            } else {
                None // Iteration complete
            }
        }
    }

    // Usage:
    for n in (Countdown { value: 5 }) {
        println!("{n}"); // 5, 4, 3, 2, 1
    }
}

// Associated Types
// Rust traits can define associated types — type placeholders that each implementor fills in. Python has no equivalent:

fn assoc_types() {
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Countdown {
        remaining: u32,
    }

    impl Iterator for Countdown {
        type Item = u32; // This iterator yields u32 values
        fn next(&mut self) -> Option<u32> {
            if self.remaining > 0 {
                self.remaining -= 1;
                Some(self.remaining)
            } else {
                None
            }
        }
    }
}

// Operator Overloading: __add__ → impl Add
// Python uses magic methods (__add__, __mul__). Rust uses trait implementations — same idea, but type-checked at compile time:

fn op_overload() {
    use std::ops::Add;

    #[derive(Debug, Clone, Copy)]
    struct Vec2 {
        x: f64,
        y: f64,
    }

    impl Add for Vec2 {
        type Output = Vec2; // Associated type: what does + return?
        fn add(self, rhs: Vec2) -> Vec2 {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    let c = a + b; // Type-safe: only Vec2 + Vec2 is allowed
    println!("{:?}", c);

    // Key difference: Python’s __add__ accepts any other at runtime (you check types manually or get a TypeError). Rust’s Add trait enforces the operand types at compile time — Vec2 + i32 is a compile error unless you explicitly impl Add<i32> for Vec2.
}
fn main() {
    op_overload();
    // assoc_types();
    // iter_trait();
    // comparison_traits();
    // dunder_method();
    // structural_typing();
    // rs_traits();
}
