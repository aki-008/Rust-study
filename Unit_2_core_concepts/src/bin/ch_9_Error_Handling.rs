#![allow(unused)]

use Result::Err;
use core::error;
use serde_json::{Error, Value};
use std::{
    fs,
    io::{self, read_to_string},
    result,
};
use thiserror::Error;

//  ⭕ Rust Result-Based Error Handling
fn err_handle() {
    // map_err: transforms the error if encountered
    // ?: handles the error if encountered, returns map_err here

    fn load_config(path: &str) -> Result<Value, ConfigError> {
        let contents =
            fs::read_to_string(path).map_err(|e| ConfigError::FileError(e.to_string()))?;

        let data: Value =
            serde_json::from_str(&contents).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        if data.get("version").is_none() {
            return Err(ConfigError::MissingField("version".to_string()));
        }

        Ok(data)
    }

    #[derive(Debug)]
    enum ConfigError {
        FileError(String),
        ParseError(String),
        MissingField(String),
    }

    //     Python:                                   |          Rust:
    // ─────────                                     |      ─────
    // - Errors are exceptions (thrown)              |      - Errors are values (returned)
    // - Hidden control flow (stack unwinding)       |      - Explicit control flow (? operator)
    // - Can't tell what errors from signature       |      - MUST see errors in return type
    // - Uncaught exceptions crash at runtime        |      - Unhandled Results produce compile warnings (always handle them)
    // - try/except is optional                      |      - Handling Result is required
    // - Broad except catches everything             |      - match arms are exhaustive
}

// ⭕  The Two Result Variants
fn result_var() {
    // enum Result<T, E> {
    //     Ok(T),  // Success — contains the value (like Python's return value)
    //     Err(E), // Failure — contains the error (like Python's raised exception)
    // }

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Division by zero".to_string()) // Like: raise ValueError("...")
        } else {
            Ok(a / b) // Like: return a / b
        }
    }

    // Handling Result — like try/except but explicit
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {result}"),
        Err(msg) => println!("Error: {msg}"),
    }
}

//  ⭕ The ? Operator
fn exp_propogate() {
    fn read_username() -> Result<String, io::Error> {
        let contents = fs::read_to_string("config.txt")?; // ? = propagate on Err
        Ok(contents.lines().next().unwrap_or("").to_string())
    }

    fn greet() -> Result<(), io::Error> {
        let name = read_username()?; // ? = if Err, return Err immediately
        println!("Hello, {name}!"); // Only reached on Ok
        Ok(())
    }

    // The ? says: "if this is Err, return it from THIS function immediately."
    // It's like Python's exception propagation, but:
    // 1. It's visible (you see the ?)
    // 2. It's in the return type (Result<..., io::Error>)
    // 3. The compiler ensures you handle it somewhere
}

//  ⭕ Chaining with ?

fn chain_propogation() {
    // Rust — same chain, but explicit

    // fn process_file(path: &str) -> Result<Data, AppError> {
    //     let text = fs::read_to_string(path)?; // ? propagates io::Error
    //     let data: Value = serde_json::from_str(&text)?; // ? propagates serde error
    //     let validated = validate(&data)?; // ? propagates validation error
    //     let result = transform(&validated)?; // ? propagates transform error

    //     Ok(result)
    // }

    // Every ? is a potential early return — and they're all visible!
}
/*
🔗 Chaining Daigram below:
https://tinyurl.com/4eh6r4vk
*/

//  ⭕ Rust Custom Errors with thiserror
// fn custom_err() {
//     #[derive(Debug, Error)]
//     enum AppError {
//         #[error("{entity} with id {id} not found")]
//         NotFound { entity: String, id: i64 },

//         #[error("Validation error on {field}: {message}")]
//         Validation { field: String, message: String },

//         #[error("IO error: {0}")]
//         Io(#[from] std::io::Error), // Auto-convert from io::Error

//         #[error("JSON error: {0}")]
//         Json(#[from] serde_json::Error), // Auto-convert from serde error
//     }

//     // Usage:
//     fn find_user(user_id: i64) -> Result<User, AppError> {
//         users.get(&user_id).cloned().ok_or(AppError::NotFound {
//             entity: "User".to_string(),
//             id: user_id,
//         })
//     }

//     // The #[from] attribute means ? auto-converts io::Error → AppError::Io
//     fn load_users(path: &str) -> Result<Vec<User>, AppError> {
//         let data = fs::read_to_string(path)?; // io::Error → AppError::Io automatically
//         let users: Vec<User> = serde_json::from_str(&data)?; // → AppError::Json
//         Ok(users)
//     }
// }

fn main() {
    // result_var();
}
