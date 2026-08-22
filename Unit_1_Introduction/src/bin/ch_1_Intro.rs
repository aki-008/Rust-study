#![allow(unused)]

// Performance: From Minutes to Milliseconds
// Python is famously slow for CPU-bound work. Rust provides C-level performance with a high-level feel.
//  Python — ~2 seconds for 10 million calls  # ~2s on typical hardware
// Rust — ~0.07 seconds for the same 10 million calls
use std::collections::HashMap;
use std::time::Instant;
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}

fn fib_usage() {
    let start = Instant::now();
    let results: Vec<u64> = (0..100_000_000_000).map(|n| fibonacci(n % 94)).collect();
    println!("Elapsed: {:.2?}", start.elapsed()); // ~100.00ns (compilation) and 0.00ns for running
}

// Memory Safety Without a Garbage Collector
fn mem_safe() {
    // Rust — ownership prevents circular references by design
    struct Node {
        value: String,
        children: Vec<Node>, // Children are OWNED — no cycles possible
    }

    impl Node {
        fn new(value: &str) -> Self {
            Node {
                value: value.to_string(),
                children: Vec::new(),
            }
        }

        fn add_child(&mut self, child: Node) {
            self.children.push(child); // Ownership transfers here
        }
    }

    fn usage() {
        let mut root = Node::new("root");
        let child = Node::new("child");
        root.add_child(child);
        // When root is dropped, all children are dropped too.
        // Deterministic, zero overhead, no GC.
    }
    // Key insight: In Rust, the child doesn’t hold a reference back to the parent. If you truly need cross-references (like a graph), you use explicit mechanisms like Rc<RefCell<T>> or indices — making the complexity visible and intentional.
}

// 2. None: The Billion Dollar Mistake (Python Edition)

fn none_handle() {
    #[derive(Clone)]
    struct User {
        name: String,
    }
    // Rust — None is impossible unless explicitly handled
    fn find_user(user_id: i64) -> Option<User> {
        let users = HashMap::from([
            (
                1,
                User {
                    name: "Alice".into(),
                },
            ),
            (2, User { name: "Bob".into() }),
        ]);
        users.get(&user_id).cloned()
    }

    let user = find_user(999); // Returns None variant of Option<User>
    // println!("{}", user.name);  // ❌ Compile error: Option<User> has no field `name`

    // You MUST handle the None case:
    match find_user(999) {
        Some(user) => println!("{}", user.name),
        None => println!("User not found"),
    }

    // Or use combinators:
    let name = find_user(999)
        .map(|u| u.name)
        .unwrap_or_else(|| "Unknown".to_string());
}

// 3. The GIL: Python’s Concurrency Ceiling
// Rust — true parallelism, no GIL, no serialization overhead
use std::thread;

fn cpu_work(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum()
}

fn concun() {
    let start = std::time::Instant::now();
    let handles: Vec<_> = (0..4)
        .map(|_| thread::spawn(|| cpu_work(3_000_000)))
        .collect();

    let results: Vec<u64> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("4 threads: {:.2?}", start.elapsed()); // ~4x faster than single thread

    // With Rayon (Rust’s parallel iterator library), parallelism is even simpler
}

fn main() {
    concun();
    // mem_safe();
    // fib_usage();
}
