#![allow(unused)]

/* =========================
  ⭕   Variable Declaration
========================= */

use std::{collections::HashMap, fmt::format};

fn variable_declaration() {
    // Rust — immutable by default, statically typed
    let count = 0; // Immutable, type inferred as i32
    // count = 5;            // ❌ Compile error: cannot assign twice to immutable variable
    // count = "hello";      // ❌ Compile error: expected integer, found &str

    let mut count = 0; // Explicitly mutable
    count = 5; // ✅ Works
    // count = "hello";      // ❌ Still can't change type

    const MAX_SIZE: usize = 1024; // True constant — enforced by compiler

    // Python: variables are labels that point to objects
    // Rust: variables are named storage locations that OWN their values

    // Variable shadowing — unique to Rust, very useful
    let input = "42"; // &str
    let input = input.parse::<i32>().unwrap(); // Now it's i32 — new variable, same name
    let input = input * 2; // Now it's 84 — another new variable, same name

    // In Python, you'd just reassign and lose the old type:
    // input = "42"
    // input = int(input)   # Same name, different type — Python allows this too
    // But in Rust, each `let` creates a genuinely new binding. The old one is gone.

    println!("{count}, {input}, {MAX_SIZE}");
}

/* =========================
  ⭕  Counter
========================= */

struct Counter {
    value: i64,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i64 {
        self.value
    }
}

fn counter() {
    let mut c = Counter::new();

    c.increment();

    println!("{}", c.get_value());
}

/* =========================
  ⭕  Data Types
========================= */

fn data_types() {
    // Rust : strict sizes, prevents overflow error
    let x: i32 = 42;
    let y: f64 = 3.14;
    let big: i128 = 2_i128.pow(100); // precision values limit to only 128 bits

    let million = 1_000_000;

    let a = 42u8; // value with precision value/range as u8
    let b = 3.14f32; // same as above

    println!(" i32: {x} ,\n f64: {y},\n i128: {big},\n million: {million},\n a: {a},\n b:  {b}");
}

// ⭕ Size Types (Important!)
fn sizes() {
    // usize and isize — pointer-sized integers, used for indexing
    let length: usize = vec![1, 2, 3].len(); // .len() returns usize
    let index: usize = 0; // Array indices are always usize

    // In Python, len() returns int and indices are int — no distinction.
    // In Rust, mixing i32 and usize requires explicit conversion:
    let i: i32 = 5;
    // let item = vec[i];    // ❌ Error: expected usize, found i32
    // let item = vec[i as usize]; // ✅ Explicit conversion
}

// ⭕  Type inference
fn type_inf() {
    // Rust infers types but they're FIXED — not dynamic
    let x = 42; // Compiler infers i32 (default integer type)
    let y = 3.14; // Compiler infers f64 (default float type)
    let s = "hello"; // Compiler infers &str (string slice)
    let v = vec![1, 2]; // Compiler infers Vec<i32>

    // You can always be explicit:
    let x: i64 = 42;
    let y: f32 = 3.14;

    // Unlike Python, the type can NEVER change after inference:
    let x = 42;
    // x = "hello";      // ❌ Error: expected integer, found &str
}

// ⭕  String Types: String vs &str
// This is one of the biggest surprises for Python developers. Rust has two main string types where Python has one.

fn string_types() {
    // Rust has TWO string types:

    // 1. &str (string slice) — borrowed, immutable, like a "view" into string data
    let name: &str = "Alice"; // Points to string data in the binary
    // Closest to Python's str, but it's a REFERENCE

    // 2. String (owned string) — heap-allocated, growable, owned
    let mut greeting = String::from("Hello, "); // Owned, can be modified
    greeting.push_str(name);
    greeting.push('!');
    // greeting is now "Hello, Alice!"

    println!("{}", greeting);
    println!("{}", name);
}
fn use_greet() {
    // Think of it like this:
    // &str  = "I'm looking at a string someone else owns"  (read-only view)
    // String = "I own this string and can modify it"        (owned data)

    // Function parameters: prefer &str (accepts both types)
    fn greet(name: &str) -> String {
        // accepts &str AND &String
        format!("Hello, {}!", name) // format! creates a new String
    }

    let s1 = "world"; // &str literal
    let s2 = String::from("Rust"); // String

    print!("{}\n", greet(s1)); // ✅ &str works directly
    print!("{}\n", greet(&s2)); // ✅ &String auto-converts to &str (Deref coercion)
}

fn string_manip() {
    // String Modify/Grow
    let mut name = String::from("Alice"); // &str can't be modified in-place
    name = name.to_uppercase();
    println!("{}", name);

    let lower = "alice";
    let upper = name.to_uppercase(); //"ALICE" :: String — New allocation
    let contains = name.contains("lic"); // True :: Bool
    let parts: Vec<&str> = "a, b, c".split(",").collect(); // ["a", "b", "c"] :: Vec<&str>
    let joined = ["a", "b", "c"].join("-"); //  "a-b-c" :: String
    let stripped = "  hello  ".trim(); //"hello" :: &str — no allocation!
    let replaced = name.replace("a", "K"); //"Klice" :: String

    // Key insight: some operations return &str (no allocation), others return String.
    // .trim() returns a slice of the original — efficient!
    // .to_uppercase() must create a new String — allocation required.

    println!("name: {}", name);
    println!("upper: {}", upper);
    println!("contains: {}", contains);
    println!("parts: {:?}", parts);
    println!("joined: {}", joined);
    println!("stripped: {}", stripped);
    println!("replaced: {}", replaced);

    // Python str     ≈ Rust &str     (you usually read strings)
    // Python str     ≈ Rust String   (when you need to own/modify)

    // Rule of thumb:
    // - Function parameters → use &str (most flexible)
    // - Struct fields       → use String (struct owns its data)
    // - Return values       → use String (caller needs to own it)
    // - String literals     → automatically &str
}

// ⭕  Printing and String Formatting

fn print_n_str_format() {
    let name = "Alice";
    let age = 35;

    println!("Hello, World!");
    println!("Name: {} Age: {}", name, age); // Positional {}
    println!("Name: {name}, Age: {age}"); // Inline variables (Rust 1.58+, like f-strings!);       // Inline variables (Rust 1.58+, like f-strings!)

    // Format Specifiers
    // Rust formatting (very similar to Python!)
    println!("{:.2}", 3.14159); // "3.14" — 2 decimal places
    println!("{:05}", 402); // "00042" — zero-padded
    println!("{:#x}", 255); // "0xff" — hex
    println!("{:>10}", 42); // "        42" — right-aligned
    println!("{:<10}|", "left"); // "left      |" — left-aligned
}

// ⭕ Debug Printing
fn debug_print() {
    // Rust — {:?} and {:#?}
    println!("{:?}", vec![1, 2, 3]); // "[1, 2, 3]" — Debug format
    println!("{:#?}", vec![1, 2, 3]); // Pretty-printed Debug format

    // To make your types printable, derive Debug:
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p); // "Point { x: 1.0, y: 2.0 }"
    println!("{p:?}"); // Same, with inline syntax
}

// ⭕  Type Annotations
fn annotate() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    add(1, 2); // ✅
    // add("a", "b");  // ❌ Compile error: expected i32, found &str

    // Optional values use Option<T>
    fn find(key: &str) -> Option<i32> {
        // Returns Some(value) or None
        Some(42)
    }

    // Generic types
    fn first(items: &[i32]) -> Option<i32> {
        items.first().copied()
    }

    // Type aliases
    type UserId = i64;
    type Mapping = HashMap<String, Vec<i32>>;
}

/* =========================
   ⭕  MAIN — Run Current Concept
========================= */

fn main() {
    annotate();
    // debug_print();
    // print_n_str_format();
    // string_manip();
    // use_greet();
    // string_types();
    // variable_declaration();
    // counter();
    // data_types();
    // sizes();
}

// Key insight: In Python, type hints help your IDE and mypy but don’t affect runtime. In Rust, types ARE the program — the compiler uses them to guarantee memory safety, prevent data races, and eliminate null pointer errors.
