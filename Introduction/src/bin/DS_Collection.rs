#![allow(unused)]

//  ⭕ Tuples and Destructuring
fn tuples_dest() {
    let point: (f64, f64) = (3.0, 4.0);
    let (x, y) = point; // Destructuring (same as Python unpacking)
    print!("x={x}, y={y}");

    // Mixed Types
    let record: (&str, i32, bool) = ("Alice", 30, true);
    let (name, age, active) = record;

    // Access by index (unlike Python, uses .0 .1 .2 syntax)
    let first = record.0;
    let second = record.1;
    println!(r"\n");
    println!("first={first}, second={second}")

    // Python: record[0]
    // Rust:   record.0      ← dot-index, not bracket-index
}

//  ⭕ When to Use Tuples vs Structs
fn tup_struct_usage() {
    // Tuples: quick grouping, function returns, temporary values
    fn min_max(data: &[i32]) -> (i32, i32) {
        (*data.iter().min().unwrap(), *data.iter().max().unwrap())
    }
    let (lo, hi) = min_max(&[3, 1, 2, 4, 5, 6, 7]);
    println!("lo={lo}, hi={hi}");

    // Structs: named fields, clear intent, methods
    struct Point {
        x: f64,
        y: f64,
    };

    // Rule of thumb:
    // - 2-3 same-type fields → tuple is fine
    // - Named fields needed  → use struct
    // - Methods needed       → use struct
    // (Same guidance as Python: tuple vs namedtuple vs dataclass)
}

//  ⭕ Arrays and Slices
fn rs_arr() {
    // 1. Array — fixed size, stack-allocated (no Python equivalent)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Size is part of the type!
    // numbers.push(6);  // ❌ Arrays can't grow

    // Initialize all elements to same value:
    let zeros = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // 2. Slice — a view into an array or Vec (like Python slicing, but borrowed)
    let slice: &[i32] = &numbers[1..4]; // [2, 3, 4] — a reference, not a copy!

    // Python: numbers[1:4] creates a NEW list (copy)
    // Rust:   &numbers[1..4] creates a VIEW (no copy, no allocation)

    println!(
        "numbers={:?},\n zeros={:?},\n slice={:?}",
        numbers, zeros, slice
    );
}

// Array slicing
fn arr_slice() {
    // Rust slicing — creates views (references)
    let data = [1, 2, 3, 4, 5, 6, 7, 5, 6, 8];
    let first_three = &data[..3]; // &[i32], view: [10, 20, 30]
    let last_two = &data[3..]; // &[i32], view: [40, 50]

    // No negative indexing — use .len()
    let last_two_v2 = &data[data.len() - 2..]; // &[i32], view: [40, 50]

    // Reverse: use an iterator
    let reversed: Vec<i32> = data.iter().rev().copied().collect();

    println!(
        " data={:?}, \n first_three={:?}, \n last_two={:?}, \n last_two_v2={:?}, \n reversed={:?}",
        data, first_three, last_two, last_two_v2, reversed
    )
}

//  ⭕ Structs vs Classes
// Rust — struct + impl blocks (no inheritance!)
// #[derive(Debug, Clone)]
// struct Rectangle {
//     width: f64,
//     height: f64,
// }
// fn rs_struct() {
//     impl Rectangle {
//         // "Constructor" — associated function (no self)
//         fn new(width: f64, height: f64) -> Self {
//             Rectangle { width, height } // Field shorthand when names match
//         }

//         fn area(&self) -> f64 {
//             self.width * self.height
//         }

//         fn perimeter(&self) -> f64 {
//             2.0 * (self.width + self.height)
//         }

//         fn scale(&self, factor: f64) -> Rectangle {
//             Rectangle::new(self.width * factor, self.height * factor)
//         }
//     }

//     // Display trait = Python's __str__
//     impl std::fmt::Display for Rectangle {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             write!(f, "Rectangle({} x {})", self.width, self.height)
//         }
//     }

//     fn usage() {
//         let r = Rectangle::new(10.0, 5.0);
//         println!("{}", r.area()); // 50.0
//         println!("{}", r); // Rectangle(10 x 5)
//     }
// }

//  ⭕ Main function
fn main() {
    arr_slice();
    // rs_arr();
    // tup_struct_usage();
    // tuples_dest();
}
