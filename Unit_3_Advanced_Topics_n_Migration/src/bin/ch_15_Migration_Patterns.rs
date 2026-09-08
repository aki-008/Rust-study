#![allow(unused)]

use rayon::result;
use std::fs::File;
use std::io::Write;

// What you’ll learn: How to translate dict→struct, class→struct+impl, list comprehension→iterator chain, decorator→trait, and context manager→Drop/RAII. Plus essential crates and an incremental adoption strategy.

// Dictionary → Struct
fn dict_struct() {
    // Rust — struct with named fields
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct User {
        name: String,
        age: i32,
        email: String,
        active: bool,
    }

    let user = User {
        name: "Alice".into(),
        age: 30,
        email: "alic@example.com".into(),
        active: true,
    };
    println!("{}", user.name);
}

// Context Manager → RAII (Drop)
fn cxt_manager() {
    // Rust — RAII: Drop trait runs when value goes out of scope
    fn write_file() -> std::io::Result<()> {
        let mut file = File::create("output.txt")?;
        file.write_all(b"hello")?;
        Ok(())
        // File automatically closed when `file` goes out of scope
        // No `with` needed — RAII handles it!
    }
}

// Decorator → Higher-Order Function or Macro
fn high_ord() {
    // Rust — no decorators, use wrapper functions or macros
    use std::time::Instant;

    fn timed<F, R>(name: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        println!("{} took {:.4?}", name, start.elapsed());
        result
    }

    // Usage:
    let result = timed("slow_function", || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        42
    });
}

// Iterator Pipeline (Data Processing)
fn iter_pipeline() {
    use std::collections::HashMap;

    #[derive(Debug, serde::Deserialize)]
    struct Sale {
        region: String,
        amount: f64,
    }

    fn analyze_sales(filename: &str) -> Vec<(String, usize)> {
        let data = std::fs::read_to_string(filename).unwrap();
        let mut reader = csv::Reader::from_reader(data.as_bytes());

        let mut by_region: HashMap<String, usize> = HashMap::new();
        for sale in reader.deserialize::<Sale>().flatten() {
            if sale.amount > 100.0 {
                *by_region.entry(sale.region).or_insert(0) += 1;
            }
        }

        let mut top: Vec<_> = by_region.into_iter().collect();
        top.sort_by(|a, b| b.1.cmp(&a.1));
        top.truncate(5);
        top
    }
}

// Global Config / Singleton
fn global_config() {
    use serde_json::Value;
    use std::sync::OnceLock;

    static CONFIG: OnceLock<Value> = OnceLock::new();
    fn get_config() -> &'static Value {
        CONFIG.get_or_init(|| {
            let data = std::fs::read_to_string("config.json").expect("Failed to read config");
            serde_json::from_str(&data).expect("Failed to parse config")
        })
    }
    // Usage anywhere:
    let db_host = get_config()["database"]["host"].as_str().unwrap();
}
fn main() {
    // high_ord();
    // cxt_manager();
    // dict_struct();
}
