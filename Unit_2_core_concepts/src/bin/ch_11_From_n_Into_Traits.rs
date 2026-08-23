#![allow(unused)]

// Type Conversions in Rust
// Rust From/Into
fn from_into() -> Result<(), Box<dyn std::error::Error>> {
    // Rust — From trait defines conversions
    // Implementing From<T> gives you Into<U> automatically!

    struct Celsius(f64);
    struct Fahrenheit(f64);

    impl From<Fahrenheit> for Celsius {
        fn from(f: Fahrenheit) -> Self {
            Celsius((f.0 - 32.0) * 5.0 / 9.0)
        }
    }

    // Now both work:
    let c1 = Celsius::from(Fahrenheit(212.0)); // Explicit From
    let c2: Celsius = Fahrenheit(212.0).into(); // Into (automatically derived)

    // String conversions:
    let s: String = String::from("hello"); // &str → String
    let s: String = "hello".to_string(); // Same thing
    let s: String = "hello".into(); // Also works (From is implemented)

    let num: i64 = 42i32.into(); // i32 → i64 (lossless, so From exists)
    // let small: i32 = 42i64.into();              // ❌ i64 → i32 might lose data — no From

    // For fallible conversions, use TryFrom:
    let n: Result<i32, _> = "42".parse(); // str → i32 (might fail)
    let n: i32 = "42".parse().unwrap(); // Panic if not a number
    let n: i32 = "42".parse()?; // Propagate error with ?
    Ok(())

    // Rule of thumb: Always implement From, never implement Into directly. Implementing From<A> for B gives you Into<B> for A for free.
}

// When to Use From/Into
fn from_into_usage() {
    // Implement From<T> for your types to enable ergonomic API design:

    #[derive(Debug)]
    struct UserId(i64);

    impl From<i64> for UserId {
        fn from(id: i64) -> Self {
            UserId(id)
        }
    }

    // Now functions can accept anything convertible to UserId:
    fn find_user(id: impl Into<UserId>) -> Option<String> {
        let user_id = id.into();
        // ... lookup logic
        Some(format!("User #{:?}", user_id))
    }

    find_user(42i64); // ✅ i64 auto-converts to UserId
    find_user(UserId(42)); // ✅ UserId stays as-is
}

// // TryFrom — Fallible Conversions
// // Not all conversions can succeed. Python raises exceptions; Rust uses TryFrom which returns a Result:
// fn try_from_usage() {
//     use std::num::ParseIntError;
//     let n: Result<i32, ParseIntError> = "42".try_into(); // Ok(42)
//     let n: Result<i32, ParseIntError> = "bad".try_into(); // Err(...)

//     #[derive(Debug)]
//     struct Port(u16);

//     #[derive(Debug)]
//     struct PortError{
//         Zero,
//     }

//     impl TryFrom<u16> for Port {
//         type Error = PortError;

//         fn try_from(value: u16) -> Result<Self, Self::Error> {
//             match value {
//                 0 => Err(PortError::Zero),
//                 1..=65535 => Ok(Port(value)),
//             }
//         }
//     }

//     impl  std::fmt::Display for PortError {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 PortError::Zero => write!(f, "port cannot be Zero"),
//             }
//         }

//     }

//     let p: Result<Port,_> = 8080u16.try_into();
//     let p: Result<Port, _> = 0u16.try_into();
// }

// String Conversion Patterns
// Strings are the most common source of conversion confusion for Python developers:
fn string_conversion() {
    // String → &str (borrowing, free)
    let s = String::from("hello");
    let r: &str = &s; // Automatic Deref coercion
    let r: &str = s.as_str(); // Explicit

    // &str → String (allocating, costs memory)
    let r: &str = "hello";
    let s1 = String::from(r); // From trait
    let s2 = r.to_string(); // ToString trait (via Display)
    let s3: String = r.into(); // Into trait

    // Number → String
    let s = 42.to_string(); // "42" — like Python's str(42)
    let s = format!("{:.2}", 3.14); // "3.14" — like Python's f"{3.14:.2f}"

    // String → Number
    let n: i32 = "42".parse().unwrap(); // like Python's int("42")
    let f: f64 = "3.14".parse().unwrap(); // like Python's float("3.14")

    // Custom types → String (implement Display)
    use std::fmt;

    struct Point {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 1.0, y: 2.0 };
    println!("{p}"); // (1, 2) — like Python's __str__
    let s = p.to_string(); // Also works! Display gives you ToString for free.
}

// Conversion Chains and Error Handling

fn conversion_chain_err_handling() {
    fn parse_config(raw: &str) -> Result<(String, u16), String> {
        let (host, port_str) = raw
            .split_once(':')
            .ok_or_else(|| "missing ':' separator".to_string())?;

        let port: u16 = port_str.parse().map_err(|e| format!("invalid port: {e}"))?;

        if port == 0 {
            return Err("port cannot be zero".to_string());
        }

        Ok((host.to_string(), port))
    }

    match parse_config("localhost:8080") {
        Ok((host, port)) => println!("Connecting to {host}:{port}"),
        Err(e) => eprintln!("Config error: {e}"),
    }
}

fn main() {
    conversion_chain_err_handling();
    // string_conversion();
    // from_into_usage();
    // from_into().unwrap();
}
