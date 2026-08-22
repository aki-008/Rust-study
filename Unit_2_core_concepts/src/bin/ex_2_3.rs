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
