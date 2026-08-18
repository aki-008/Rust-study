#![allow(unused)]

use std::collections::HashMap;

//  ⭕ Rust Enums — Data-Carrying Variants
enum Shape {
    Circle(f64),                         // Circle carries radius
    Rectangle(f64, f64),                 // Rectangle carries width, height
    Triangle { base: f64, height: f64 }, // Named fields also work
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
        // ❌ If you add Shape::Pentagon and forget to handle it here,
        //    the compiler refuses to build. No grep needed.
    }
}

// Key insight: Rust’s match is exhaustive — the compiler verifies you handle every variant. Add a new variant to an enum and the compiler tells you exactly which match blocks need updating. Python’s match has no such guarantee.

fn area_usage() {
    // Enum variant with tuple-like data:
    // Shape::Circle(value)
    let circle = Shape::Circle(5.0);

    // Enum variant with multiple tuple-like values:
    // Shape::Rectangle(value1, value2)
    let rectangle = Shape::Rectangle(10.0, 4.0);

    // Enum variant with named fields:
    // Shape::Triangle { field: value, field: value }
    let triangle = Shape::Triangle {
        base: 6.0,
        height: 3.0,
    };

    // &shape = borrow the Shape instead of moving it
    // area() receives: &Shape
    println!("Circle area: {}", area(&circle));
    println!("Rectangle area: {}", area(&rectangle));
    println!("Triangle area: {}", area(&triangle));
}

//  ⭕ Enums Replace Multiple Python Patterns

// simple enum
enum Status {
    Pending,
    Active,
    Closed,
}

// Data-carrying enum (tagged union — type-safe!)
enum Message {
    Text(String),
    Image {
        url: String,
        width: u32,
        height: u32,
    },
    Quit, // No data
    Move {
        x: i32,
        y: i32,
    },
}

fn process(msg: Message) {
    match msg {
        Message::Text(content) => println!("Text: {content}"),
        Message::Image { url, width, height } => {
            println!("Image: {url}, ({width}x{height})")
        }
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({x}, {y})"),
    }
}

fn process_usage() {
    let text = Message::Text(String::from("Roses are red"));
    let img = Message::Image {
        url: (String::from("goofle.com")),
        width: (5),
        height: (4),
    };
    let quit = Message::Quit;
    let mv = Message::Move { x: (5), y: (8) };
    process(text);
    process(img);
    process(quit);
    process(mv);
}

//  ⭕ Exhaustive Pattern Matching

fn matching() {
    // Rust — MUST handle every possible case
    fn describe(value: i32) -> &'static str {
        match value {
            0 => "zero",
            1 => "one",
            // ❌ Compile error: non-exhaustive patterns: `i32::MIN..=-1_i32`
            //    and `2_i32..=i32::MAX` not covered
            _ => "other", // _ = catch-all (required for open-ended types)
        }
    }

    println!("describe_call: {}", describe(0));

    // For enums, NO catch-all needed — compiler knows all variants:
    enum Color {
        Red,
        Green,
        Blue,
    }

    fn color_hex(c: Color) -> &'static str {
        match c {
            Color::Red => "#ff0000",
            Color::Green => "#00ff00",
            Color::Blue => "#0000ff",
            // No _ needed — all variants covered
            // Add Color::Yellow later → compiler error HERE
        }
    }

    fn get_hex() {
        let color = Color::Red;
        println!("Return code: {}", color_hex(color));
    }
    get_hex();
}

//  ⭕ Pattern Matching Features
fn pattern_match() {
    // Multiple values (like Python's case 1 | 2 | 3:)
    let value = 4;
    match value {
        1 | 2 | 3 => println!("samall"),
        4..=9 => println!("medium"),
        _ => println!("large"),
    }
    // Guards (like Python's case x if x > 0:
    let temperature = 150;
    match temperature {
        t if t > 100 => println!("boiling"),
        t if t < 0 => println!("freezing"),
        t => println!("normal: {t}"),
    }
    // Nested destructuring
    let point = (3, (4, 5));
    match point {
        (0, _) => println!("on y-axis"),
        (_, (0, _)) => println!("y=0"),
        (x, (y, z)) => println!("x={x}, y={y}, z={z}"),
    }
}

//  ⭕ Option for None Safety
// Option<T> is the most important Rust enum for Python developers. It replaces None with a type-safe alternative.
// Rust — Handling Option<T> / None
fn none_handle() {
    use std::collections::HashMap;

    #[derive(Clone)]
    struct User {
        name: String,
    }

    fn find_user(user_id: i64) -> Option<User> {
        let users = HashMap::from([(
            1,
            User {
                name: "Alice".into(),
            },
        )]);

        users.get(&user_id).cloned()
    }

    // find_user() returns Option<User>
    // So `user` is NOT directly a User.
    // It can be Some(User) or None.
    let user = find_user(999);

    // ─────────────────────────────────────────
    // Method 1: match
    // ─────────────────────────────────────────

    // match forces us to handle both possibilities:
    // Some(user) -> User was found
    // None       -> User was not found
    match find_user(999) {
        Some(user) => println!("Found: {}", user.name),
        None => println!("Not found"),
    }

    // ─────────────────────────────────────────
    // Method 2: if let
    // ─────────────────────────────────────────

    // `if let Some(user) = ...`
    // means:
    // "If the result is Some, extract the User."
    //
    // If the result is None, the block is skipped.
    if let Some(user) = find_user(1) {
        println!("Found: {}", user.name);
    }

    // ─────────────────────────────────────────
    // Method 3: map + unwrap_or_else
    // ─────────────────────────────────────────

    // map() transforms the value INSIDE Some.
    //
    // Some(User) -> Some(user.name)
    // None       -> None
    //
    // unwrap_or_else() provides a fallback when None.
    let name = find_user(999)
        .map(|u| u.name)
        .unwrap_or_else(|| "Unknown".to_string());

    println!("Name: {}", name);

    // ─────────────────────────────────────────
    // Method 4: ? operator
    // ─────────────────────────────────────────

    // The function itself returns Option<String>,
    // so `?` can return None early.
    fn get_user_name(id: i64) -> Option<String> {
        // If find_user() returns Some(user):
        //     `user` gets the actual User.
        //
        // If find_user() returns None:
        //     this function immediately returns None.
        let user = find_user(id)?;

        // At this point `user` is a User, NOT Option<User>.
        Some(user.name)
    }

    println!("{:?}", get_user_name(1)); // Some("Alice")
    println!("{:?}", get_user_name(999)); // None
}

fn main() {
    none_handle();
    // pattern_match();
    // matching();
    // process_usage();
    // area_usage();
}
