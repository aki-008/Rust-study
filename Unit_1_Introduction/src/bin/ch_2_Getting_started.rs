#![allow(unused)]

/* =========================
  ⭕   CODE 1 — Hello World
========================= */

fn hello_world() {
    println!("Hello, world!");
}

/* =========================
  ⭕   CODE 2 — Variables and Mutability Keywords
========================= */

fn variables_and_mutability() {
    // let — declare a variable (like Python assignment, but immutable by default)
    let name = "Alice";
    // name = "Bob"; // ❌ Compile error! Immutable by default

    // mut — opt into mutability
    let mut count = 0; // Python: count = 0 (always mutable in Python)
    count += 1; // ✅ Allowed because of `mut`

    // const — compile-time constant (like Python's convention of UPPER_CASE, but enforced)
    const MAX_SIZE: usize = 1024; // Python: MAX_SIZE = 1024 (convention only)

    println!("{name}, {count}, {MAX_SIZE}");
}

/* =========================
   ⭕  CODE 3 — Ownership and Borrowing Keywords
========================= */

fn ownership_and_borrowing() {
    // These have NO Python equivalents — they're Rust-specific concepts

    // & — borrow (read-only reference)
    fn print_name(name: &str) {
        println!("{name}");
    }
    // Python: def print_name(name: str) — but Python passes ref always

    // &mut — mutable borrow
    fn append(list: &mut Vec<i32>) {
        list.push(10);
    }
    // Python: def append(lst: list) — always mutable in Python

    // move — transfer ownership (happens implicitly in Rust, never in Python)
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is MOVED to s2 — s1 is no longer valid
    // println!("{}", s1); // ❌ Compile error: value moved

    println!("{s2}");
}

/* =========================
  ⭕   CODE 4 — Type Definition Keywords
========================= */

fn type_definitions() {
    struct Point {
        x: f64,
        y: f64,
    }

    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
    }

    impl Point {
        fn distance(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    trait Drawable {
        fn draw(&self);
    }

    type UserId = i64;

    let point = Point { x: 3.0, y: 4.0 };

    println!("{}", point.distance());
}

/* =========================
  ⭕   CODE 5 — Control Flow Keywords
========================= */

fn control_flow() {
    let value = 2;

    // match — exhaustive pattern matching (like Python 3.10+ match, but enforced)
    match value {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        _ => println!("other"), // _ = wildcard (like Python's case _:)
    }

    // if let — destructure + conditional (Pythonic: if (m := regex.match(s)):)
    let optional_value = Some(42);

    if let Some(x) = optional_value {
        println!("{}", x);
    }

    // loop — infinite loop (like while True:)
    loop {
        break; // Must break to exit
    }

    // for — iteration (like Python's for, but needs .iter() more often)
    let collection = vec![1, 2, 3];

    for item in collection.iter() {
        // for item in collection:
        println!("{}", item);
    }

    // while let — loop with destructuring
    let mut stack = vec![1, 2, 3];

    while let Some(item) = stack.pop() {
        println!("{}", item);
    }
}

/* =========================
  ⭕   CODE 6 — Visibility Keywords
========================= */

fn visibility() {
    // In Python, "private" is a gentleman's agreement.
    // In Rust, private is enforced by the compiler.

    fn greet() {
        println!("Hello");
    }

    fn internal() {
        println!("Internal");
    }

    fn private_helper() {
        println!("Private");
    }

    greet();
    internal();
    private_helper();
}

/* =========================
   ⭕  MAIN — Run Current Concept
========================= */

fn main() {
    visibility();

    // hello_world();
    // variables_and_mutability();
    // ownership_and_borrowing();
    // type_definitions();
    // control_flow();
}
