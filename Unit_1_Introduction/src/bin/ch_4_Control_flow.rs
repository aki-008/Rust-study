#![allow(unused)]

// ⭕ Conditional Statements
fn conditional_stmt(temperature: f64) {
    if temperature > 100.0 {
        println!("Too hot!");
    } else if temperature < 0.0 {
        println!("Too cold!");
    } else {
        println!("Just right");
    }

    // if is an EXPRESSION — returns a value (like Python ternary, but more powerful)
    let status = if temperature > 100.0 { "hot" } else { "ok" };
}

//  ⭕ Loops and Iteration
fn iteration() {
    for i in 0..5 {
        // range(5) → 0..5
        println!("{}", i);
    }

    for item in ["a", "b", "c"] {
        // Direct iteration
        println!("{}", item);
    }

    for (i, item) in ["a", "b", "c"].iter().enumerate() {
        // enumerate()
        println!("{}: {}", i, item);
    }

    // HashMap iteration
    use std::collections::HashMap;
    let map = HashMap::from([("x", 1), ("y", 2)]);
    for (key, value) in &map {
        // & borrows the map
        println!("{} = {}", key, value);
    }

    // Python:              Rust:               Notes:
    // range(5)             0..5                Half-open (excludes end)
    // range(1, 10)         1..10               Half-open
    // range(1, 11)         1..=10              Inclusive (includes end)
    // range(0, 10, 2)      (0..10).step_by(2)  Step (method, not syntax)
}

//  ⭕ while Loops
fn iter_while() {
    use std::io;

    fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    let mut count = 0;
    while count < 5 {
        println!("{}", count);
        count += 1;
    }

    // Infinite loop — use `loop`, not `while true`
    loop {
        let data = get_input();
        if data == "quit" {
            break;
        }
    }

    // loop can return a value! (unique to Rust)
    let result = loop {
        let input = get_input();
        if let Ok(num) = input.parse::<i32>() {
            break num; // `break` with a value — like return for loops
        }
        println!("Not a number, try again");
    };
}

//  ⭕ List Comprehensions vs Iterator Chains
fn iter_chain() {
    // Rust — iterator chains (.map, .filter, .collect)
    let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
    let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
    let pairs: Vec<(i32, i32)> = (0..3).flat_map(|x| (0..3).map(move |y| (x, y))).collect();

    // These are LAZY — nothing runs until .collect()
    // Python comprehensions are eager (run immediately)
    // Rust iterators can be more efficient for large datasets
}

//  ⭕ Expression Blocks
// Everything in Rust is an expression (or can be). This is a big shift from Python, where if/for are statements.

fn exp_block() {
    let x = 2;
    let result = if x < 5 { "yes" } else { "no" };

    // Blocks are expressions — the last line (without semicolon) is the return value
    let value = {
        let x = 5;
        let y = 10;
        x + y // No semicolon → this is the value of the block (15)
    };

    let temperature = 55;

    // match is an expression too
    let description = match temperature {
        t if t > 100 => "boiling",
        t if t > 50 => "hot",
        t if t > 20 => "warm",
        _ => "cold",
    };
}

// ⭕ Functions and Type Signatures
fn rs_func() {
    // Rust — types REQUIRED on function signatures, no defaults
    fn greet(name: &str, greeting: &str) -> String {
        format!("{}, {}!", greeting, name)
    }

    // No default arguments — use builder pattern or Option
    fn greet_with_default(name: &str, greeting: Option<&str>) -> String {
        let greeting = greeting.unwrap_or("Hello");
        format!("{}, {}!", greeting, name)
    }

    // No *args/**kwargs — use slices or structs
    fn sum_all(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

    // First-class functions and closures
    fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    let result = apply(|x| x * 2, 5); // 10
}

//  ⭕ Return Values
fn ret_val() {
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None // Early return (could also write `return None;`)
        } else {
            Some(a / b) // Last expression — implicit return
        }
    }

    println!("{:?}", divide(2.0, 10.0));
}

//  ⭕ Multiple Return Values

fn multi_ret_val() {
    // Rust — return a tuple (same concept!)
    fn min_max(numbers: &[i32]) -> (i32, i32) {
        let min = *numbers.iter().min().unwrap();
        let max = *numbers.iter().max().unwrap();
        (min, max)
    }
    let (lo, hi) = min_max(&[3, 1, 4, 1, 5]);
}

//  ⭕ Methods: self vs &self vs &mut self
fn self_meth() {
    // A simple struct with one field.
    struct MyStruct {
        value: i32,
    }

    impl MyStruct {
        // `Self` means `MyStruct`.
        //
        // No `self` means this method does NOT need an existing object.
        // We call it using `MyStruct::new()`.
        fn new() -> Self {
            println!("new(): Creating a new MyStruct...");
            Self { value: 10 }
        }

        // `&self` = borrow the object immutably.
        // We can READ the object, but cannot change it.
        fn read_only(&self) {
            println!("read_only(): Value is {}", self.value);
        }

        // `&mut self` = borrow the object mutably.
        // We can READ and MODIFY the object.
        fn modify(&mut self) {
            println!(
                "modify(): Changing value from {} to {}",
                self.value,
                self.value + 5
            );
            self.value += 5;
        }

        // `self` = take OWNERSHIP of the object.
        // The object is moved into this method.
        // After calling this, `x` can no longer be used.
        fn consume(self) {
            println!("consume(): Took ownership of the object.");
            println!("consume(): Final value = {}", self.value);
        }
    }

    // `new()` doesn't need an existing object.
    let mut x = MyStruct::new();

    println!("--- Read only ---");
    x.read_only();

    println!("--- Modify ---");
    x.modify();

    println!("--- Read after modify ---");
    x.read_only();

    println!("--- Consume ---");
    x.consume();

    // `x` is now moved/consumed.
    // println!("{}", x.value); // ERROR
}

//  ⭕ Main
fn main() {
    self_meth();
    // multi_ret_val();
    // ret_val();
    // exp_block();
    // iter_chain();
    // iter_while();
    // iteration();
    // conditional_stmt(45.0);
}
