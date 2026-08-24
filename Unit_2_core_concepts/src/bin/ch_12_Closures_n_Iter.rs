#![allow(unused)]

fn rs_closures() {
    // Rust — closures use |args| body syntax
    let double = |x: i32| x * 2;
    let result = double(5); // 10

    // Closures capture variables from enclosing scope:
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n // `move` transfers ownership of `n` into the closure
    }

    let add_5 = make_adder(5);
    println!("{}", add_5(10)); // 15

    // Higher-order functions with iterators:
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();

    //     Python:                              Rust:
    // ─────────                            ─────
    // lambda x: x * 2                      |x| x * 2
    // lambda x, y: x + y                   |x, y| x + y
    // lambda: 42                           || 42

    // # Multi-line
    // def f(x):                            |x| {
    //     y = x * 2                            let y = x * 2;
    //     return y + 1                         y + 1
    //                                       }
}

// a Closure is a function that remembers and keeps access to the variables in its outer scope, even after that outer function has finished running

fn closure_capture() {
    // Rust — closures capture correctly (no late-binding gotcha)
    let funcs: Vec<Box<dyn Fn() -> i32>> = (0..3)
        .map(|i| Box::new(move || i) as Box<dyn Fn() -> i32>)
        .collect();

    let results: Vec<i32> = funcs.iter().map(|f| f()).collect();
    println!("{:?}", results); // [0, 1, 2] — correct!

    // `move` captures a COPY of `i` for each closure — no late-binding surprise.
}

fn three_closure_traits() {
    // Rust closures implement one or more of these traits:

    // Fn — can be called multiple times, doesn't mutate captures (most common)
    fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    // FnMut — can be called multiple times, MAY mutate captures
    fn apply_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    // FnOnce — can only be called ONCE (consumes captures)
    fn apply_once(f: impl FnOnce() -> String) -> String {
        f()
    }

    // Python has no equivalent — closures are always Fn-like.
    // In Rust, the compiler automatically determines which trait to use.
}

fn rs_iterator() {
    // Rust — Iterator trait (similar concept, different syntax)
    struct Fibonacci {
        a: u64,
        b: u64,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { a: 0, b: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.a;
            self.a = self.b;
            self.b = current + self.b;
            Some(current)
        }
    }

    // Lazy — values computed on demand (just like Python generators)
    let first_10: Vec<u64> = Fibonacci::new().take(10).collect();

    // Iterator chains — like generator expressions
    let squares: Vec<u64> = (0..1_000_000u64).map(|x| x * x).take(5).collect();
}

// Comprehensions vs Iterator Chains
// This section maps Python’s comprehension syntax to Rust’s iterator chains.

// List Comprehension → map/filter/collect
fn iter_chain() {
    // Rust iterator chains:
    //     let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
    //     let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
    //     let names: Vec<&str> = users
    //         .iter()
    //         .filter(|u| u.active)
    //         .map(|u| u.name.as_str())
    //         .collect();
    //     let pairs: Vec<(i32, i32)> = (0..3).flat_map(|x| (0..3).map(move |y| (x, y))).collect();
    //     let flat: Vec<i32> = nested
    //         .iter()
    //         .flat_map(|sublist| sublist.iter().copied())
    //         .collect();
}

// Dict Comprehension → collect into HashMap
fn dict_comprehension() {
    // Rust
    // let word_lengths: HashMap<&str, usize> = words.iter().map(|w| (*w, w.len())).collect();
    // let inverted: HashMap<&V, &K> = mapping.iter().map(|(k, v)| (v, k)).collect();
}

// Set Comprehension → collect into HashSet
fn set_comprehension() {
    // Rust
    // let unique_lengths: HashSet<usize> = words.iter().map(|w| w.len()).collect();
}

// Rust Macros

fn rs_macro() {
    // Rust — derive macros and declarative macros for code generation
    #[derive(Debug, Clone, PartialEq)] // Generates Debug, Clone, PartialEq impls at COMPILE time
    struct Point {
        x: f64,
        y: f64,
    }

    // Declarative macro (like a template)
    macro_rules! log_call {
        ($func_name:expr, $body:expr) => {{
            println!("Calling {}", $func_name);
            $body
        }};
    }

    fn process(data: &str) -> String {
        log_call!("process", data.to_uppercase())
    }
}

// Common Built-in Macros
fn built_in_macros() {
    // // These macros are used everywhere in Rust:

    // println!("Hello, {}!", name); // Print with formatting
    // format!("Value: {}", x); // Create formatted String
    // vec![1, 2, 3]; // Create a Vec
    // assert_eq!(2 + 2, 4); // Test assertion
    // assert!(value > 0, "must be positive"); // Boolean assertion
    // dbg!(expression); // Debug print: prints expression AND value
    // todo!(); // Placeholder — compiles but panics if reached
    // unimplemented!(); // Mark code as unimplemented
    // panic!("something went wrong"); // Crash with message (like raise RuntimeError)

    // // Why are these macros instead of functions?
    // // - println! accepts variable arguments (Rust functions can't)
    // // - vec! generates code for any type and size
    // // - assert_eq! knows the SOURCE CODE of what you compared
    // // - dbg! knows the FILE NAME and LINE NUMBER
}

// Writing a Simple Macro with macro_rules!
fn sample_macro() {
    // Python dict() equivalent
    // Python: d = dict(a=1, b=2)
    // Rust:   let d = hashmap!{ "a" => 1, "b" => 2 };

    macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 90,
    };

    // Derive Macros — Auto-Implementing Traits
    // #[derive(...)] is the Rust equivalent of Python's @dataclass decorator

    // Python:
    // @dataclass(frozen=True, order=True)
    // class Student:
    //     name: str
    //     grade: int

    // Rust:
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Student {
        name: String,
        grade: i32,
    }

    // Common derive macros:
    // Debug         → {:?} formatting (like __repr__)
    // Clone         → .clone() deep copy
    // Copy          → implicit copy (only for simple types)
    // PartialEq, Eq → == comparison (like __eq__)
    // PartialOrd, Ord → <, >, sorting (like __lt__ etc.)
    // Hash          → usable as HashMap key (like __hash__)
    // Default       → MyType::default() (like __init__ with no args)

    // Crate-provided derive macros:
    // Serialize, Deserialize (serde) → JSON/YAML/TOML serialization
    //                                  (like Python's json.dumps/loads but type-safe)
}
fn main() {
    rs_macro();
    // rs_iterator();
    // closure_capture();
    // rs_closures();
}
