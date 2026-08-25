#![allow(unused)]

use std::{iter::Map, sync::MutexGuard};

fn parallelism() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::thread;

    fn main() {
        let counter = Arc::new(AtomicI64::new(0));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let counter = Arc::clone(&counter);
                thread::spawn(move || {
                    for _ in 0..1_000_000 {
                        counter.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        println!("Counter: {}", counter.load(Ordering::Relaxed)); // Always 4,000,000
        use std::sync::Arc;
        use std::sync::atomic::{AtomicI64, Ordering};
        use std::thread;

        fn main() {
            let counter = Arc::new(AtomicI64::new(0));

            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let counter = Arc::clone(&counter);
                    thread::spawn(move || {
                        for _ in 0..1_000_000 {
                            counter.fetch_add(1, Ordering::Relaxed);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }

            println!("Counter: {}", counter.load(Ordering::Relaxed)); // Always 4,000,000

            // Runs on ALL cores — true parallelism, no GIL
        }
    }
}

// Thread Safety: Type System Guarantees
fn thread_safe() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let shared = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                let mut data = shared.lock().unwrap();
                data.push(i);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", shared.lock().unwrap());
}

// Mutex Poisoning
fn mutex_poison() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data2 = Arc::clone(&data);

    let _ = thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        guard.push(4);
        panic!("oops!"); // Lock is now poisoned
    })
    .join();

    // Subsequent lock attempts return Err(PoisonError)
    match data.lock() {
        Ok(guard) => println!("Data: {guard:?}"),
        Err(poisoned) => {
            println!("Lock was poisoned! Recovering...");
            let guard = poisoned.into_inner();
            println!("Recovered: {guard:?}"); // [1, 2, 3, 4]
        }
    }
}

// Rust async/await
fn async_await() {
    // use futures::future::join_all;

    // async fn fetch_url(url: &'static str) -> Result<String, reqwest::Error> {
    //     reqwest::get(url).await?.text().await
    // }

    // #[tokio::main]
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //     let urls = vec!["https://example.com", "https://httpbin.org/get"];

    //     let tasks: Vec<_> = urls
    //         .into_iter()
    //         .map(|url| tokio::spawn(fetch_url(url)))
    //         .collect();

    //     let results = join_all(tasks).await;

    //     for result in results {
    //         match result? {
    //             Ok(content) => println!("Fetched {} bytes", content.len()),
    //             Err(e) => eprintln!("Request failed: {}", e),
    //         }
    //     }

    //     Ok(())
    // }
}

// Simple Parallelism with Rayon
fn rayon_usage() {
    // Rust — rayon for effortless CPU parallelism (one line change!)
    // use rayon::prelude::*;

    // // Sequential:
    // let results: Vec<_> = items.iter().map(|item| heavy_computation(item)).collect();

    // // Parallel (change .iter() to .par_iter() — that's it!):
    // let results: Vec<_> = items
    //     .par_iter()
    //     .map(|item| heavy_computation(item))
    //     .collect();

    // // No pickle, no process overhead, no serialization.
    // // Rayon automatically distributes work across cores.
}

fn case_study() { // [Case Study]: https://tinyurl.com/yyu7yk2h
    // 💼 Case Study: Parallel Image Processing Pipeline
    // A data science team processes 50,000 satellite images nightly. Their Python pipeline uses

    // Metric	            Python (multiprocessing)	  Rust (rayon)
    // Time (50k images)	~4.5 hours	                  ~35 minutes
    // Memory overhead	    800MB (16 workers)	          ~50MB (shared)
    // Error handling	    Opaque pickle errors	      Result<T, E> at every step
    // Startup cost	        2–3s (fork + pickle)	      None (threads)
}

fn main() {
    case_study();
    // rayon_usage();
    // async_await();
    // mutex_poison();
    // thread_safe();
    // parallelism();
}
