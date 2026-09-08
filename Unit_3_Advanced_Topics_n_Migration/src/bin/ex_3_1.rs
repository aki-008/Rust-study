// 🏋️ Exercise: Thread-Safe Counter (click to expand)
// Challenge: In Python, you might use threading.Lock to protect a shared counter. Translate this to Rust: spawn 10 threads, each incrementing a shared counter 1000 times. Print the final value (should be 10000). Use Arc<Mutex<u64>>.
#![allow(unused)]

use std::ops::ControlFlow::Continue;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Count: {}", *counter.lock().unwrap());
}
