// Exercise: Parse Config Value (click to expand)
// Challenge: Write a function parse_port(s: &str) -> Result<u16, String> that:

// Rejects empty strings with error "empty input"
// Parses the string to u16, mapping the parse error to "invalid number: {original_error}"
// Rejects ports below 1024 with "port {n} is privileged"
// Call it with "", "hello", "80", and "8080" and print the results.

#![allow(unused)]

fn parse_port(s: &str) -> Result<u16, String> {
    if s.is_empty() {
        return Err("empty input: ".to_string());
    }
    let port: u16 = s.parse().map_err(|e| format!("invalid numberL {e}"))?;
    if port < 1024 {
        return Err(format!("port {port} is privileged"));
    }
    Ok(port)
}
fn main() {
    for input in ["", "hello", "80", "8080"] {
        match parse_port(input) {
            Ok(port) => println!("{input} -> {port}"),
            Err(e) => println!("{input:?} -> {e}"),
        }
    }
}

// Key takeaway: ? with .map_err() is Rust’s replacement for try/except ValueError as e: raise ConfigError(...) from e. Every error path is visible in the return type.
