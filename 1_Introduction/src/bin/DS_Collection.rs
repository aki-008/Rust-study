#![allow(unused)]

use std::fmt::format;

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

#[derive(Debug, Clone)]
struct Rectangle {
    // this is like the __init__ in python class
    width: f64,
    height: f64,
}

// impl is used to define the functionality, like class methods in python
impl Rectangle {
    // "Constructor" — associated function (no self)
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height } // Field shorthand when names match
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn scale(&self, factor: f64) -> Rectangle {
        Rectangle::new(self.width * factor, self.height * factor)
    }
}

// Display trait = Python's __str__
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle({} x {})", self.width, self.height)
    }
}

fn usage() {
    let r = Rectangle::new(10.0, 5.0);
    println!("{}", r.area()); // 50.0
    println!("{}", r); // Rectangle(10 x 5)
}

//  ⭕ No Inheritance — Composition Instead
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> String;
}
struct Dog {
    name: String,
}
struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    fn speak(&self) -> String {
        format!("{} says woof!", self.name)
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn speak(&self) -> String {
        format!("{} says Meow!", self.name)
    }
}

fn animal_roll_call(animals: &[&dyn Animal]) {
    for a in animals {
        println!("{}", a.speak());
    }
}

fn roll_call_usage() {
    let dog = Dog {
        name: String::from("Buddy"),
    };

    let cat = Cat {
        name: String::from("Whiskers"),
    };

    animal_roll_call(&[&dog, &cat]);
}

//  ⭕ Vec vs list
// Vec<T> is Rust’s growable, heap-allocated array — the closest equivalent to Python’s list.

fn vec_list() {
    let numbers = vec![1, 2, 3]; // vec! macro (like a list literal)
    let empty: Vec<i32> = Vec::new(); // Empty vec (type annotation needed)
    let repeated = vec![0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    let from_range: Vec<i32> = (1..6).collect(); // [1, 2, 3, 4, 5]
    println!(
        " numbers={:?}\n, empty={:?}\n, repeated={:?}\n, from_ranges={:?}\n",
        numbers, empty, repeated, from_range
    )
}

// list Common operations
fn list_ops() {
    let mut nums = vec![1, 2, 3];
    nums.push(4); // [1, 2, 3, 4]
    nums.extend([5, 6]); // [1, 2, 3, 4, 5, 6]
    nums.insert(0, 0); // [0, 1, 2, 3, 4, 5, 6]
    let last = nums.pop(); // Some(6), nums = [0, 1, 2, 3, 4, 5]
    let length = nums.len(); // 6
    nums.sort(); // In-place sort
    let mut sorted_copy = nums.clone();
    sorted_copy.sort(); // Sort a clone
    nums.reverse(); // In-place reverse
    let contains = nums.contains(&3); // true
    let index = nums.iter().position(|&x| x == 3); // Some(index) or None

    println!(
        "nums={:?},\nlast={:?},\nlength={},\nsorted_copy={:?},\nnums_reverse={:?},\ncontains={},\nindex={:?}",
        nums, last, length, sorted_copy, nums, contains, index
    );
}

fn hashmap_dict() {
    use std::collections::HashMap;

    let scores = HashMap::from([("Alice", 100), ("Bob", 85)]);

    let empty: HashMap<String, i32> = HashMap::new();

    let from_pairs: HashMap<&str, i32> = [("x", 1), ("y", 2)].into_iter().collect();

    let keys = ["a", "b", "c"];
    let values = [1, 2, 3];

    let comprehension: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
}

fn hash_ops() {
    use std::collections::HashMap;

    let mut d = HashMap::new();
    d.insert("a", 1);
    d.insert("b", 2);
    d.insert("c", 3);

    let val_a = d["a"];
    let val_z = d.get("z").copied().unwrap_or(0);

    d.remove("b");

    let exists = d.contains_key("a");
    let keys: Vec<_> = d.keys().collect();
    let values: Vec<_> = d.values().collect();
    let length = d.len();

    // Entry API = Python's setdefault/defaultdict pattern
    let words = ["hello", "world", "hello", "rust"];

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in words {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!(
        "d={:?}\nval_a={}\nval_z={}\nexists={}\nkeys={:?}\nvalues={:?}\nlength={}\nword_count={:?}",
        d, val_a, val_z, exists, keys, values, length, word_count
    );
}

//  ⭕ Main function
fn main() {
    // hash_ops();
    // hashmap_dict();
    // list_ops();
    // vec_list();
    // roll_call_usage();
    // usage();
    // arr_slice();
    // rs_arr();
    // tup_struct_usage();
    // tuples_dest();
}
