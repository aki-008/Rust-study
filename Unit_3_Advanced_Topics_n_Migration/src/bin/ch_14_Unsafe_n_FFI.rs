#![allow(unused)]

// unsafe in Rust is an escape hatch — it tells the compiler “I’m doing something you can’t verify, but I promise it’s correct.” Python has no equivalent because Python never gives you direct memory access.

use rayon::result;

// What unsafe Allows
fn unsafe_usage() {
    // unsafe lets you do FIVE things that safe Rust forbids:
    // 1. Dereference raw pointers
    // 2. Call unsafe functions/methods
    // 3. Access mutable static variables
    // 4. Implement unsafe traits
    // 5. Access union fields

    // Example: calling a C function
    unsafe extern "C" {
        // unsafe
        fn abs(inputs: i32) -> i32;
    }

    fn extern_usage() {
        // SAFETY: abs() is a well-defined C standard library function.
        let result = unsafe { abs(-42) }; // Safe Rust can't verify C code
        println!("{result}"); // 42
    }

    // 1. FFI — calling C libraries (most common reason)
    // 2. Performance-critical inner loops (rare)
    // 3. Data structures the borrow checker can't express (rare)

    // As a Python developer, you'll mostly encounter unsafe in:
    // - PyO3 internals (Python ↔ Rust bridge)
    // - C library bindings
    // - Low-level system calls

    // Rule of thumb: if you're writing application code (not library code),
    // you should almost never need unsafe. If you think you do, ask in the
    // Rust community first — there's usually a safe alternative.
}

// PyO3: Rust Extensions for Python

// PyO3 is the bridge between Python and Rust. It lets you write Rust functions and classes that are callable from Python — perfect for replacing slow Python hotspots.

fn pyo3_usage() {
    /*
    Unit_3_Advanced_Topics_n_Migration\src\bin\Extension
     */

    use std::process::Command;
    let output = Command::new("uv")
        .args(["run", "python", "test.py"])
        .current_dir(r"Extension")
        .output()
        .expect("failed to execute command");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("exit status: {}", output.status);
}

// FFI Safety Patterns
fn ffi_docs() {
    let markdown_text = r#"
    FFI Safety Patterns

    When exposing Rust to Python (via PyO3 or raw C FFI), these rules prevent the most common bugs:

    Never let a panic cross the FFI boundary — a Rust panic unwinding into Python (or C) is undefined behavior. PyO3 handles this automatically for #[pyfunction], but raw extern "C" functions need explicit protection:

    #[no_mangle]
    pub extern "C" fn raw_ffi_function() -> i32 {
        match std::panic::catch_unwind(|| {
            // actual logic
            42
        }) {
            Ok(result) => result,
            Err(_) => -1,  // Return error code instead of panicking into C/Python
        }
    }
    #[repr(C)] for shared structs — if Python/C reads struct fields directly, you must use #[repr(C)] to guarantee C-compatible layout. If you’re passing opaque pointers (which PyO3 does for #[pyclass]), it’s not needed.

    extern "C" — required for raw FFI functions so the calling convention matches what C/Python expects. PyO3’s #[pyfunction] handles this for you.

    PyO3 advantage: PyO3 wraps most of these safety concerns for you — panic catching, type conversion, GIL management. Prefer PyO3 over raw FFI unless you have a specific reason not to."#;

    termimad::print_text(markdown_text);
}

// Rust Built-in Testing
// src/calculator.rs — tests live in the SAME file!
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Tests go in a #[cfg(test)] module — only compiled during `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(1.0, 0.0).is_err());
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[99];
    }

    // Testing Quick Reference
    // pytest	                      Rust	                                 Notes
    // assert x == y	         | assert_eq!(x, y)	            | Equality
    // assert x != y	         | assert_ne!(x, y)	            | Inequality
    // assert condition	         | assert!(condition)	        | Boolean
    // assert condition, "msg"	 | assert!(condition, "msg")    | With message
    // pytest.raises(E)	         | #[should_panic]	            | Expect panic
    // @pytest.fixture	         | Setup in test or helper fn   | No built-in fixtures
    // @pytest.mark.parametrize	 | rstest crate	                | Parameterized tests
    // conftest.py	             | tests/common/mod.rs	        | Shared test helpers
    // pytest.skip()	         | #[ignore]	                | Skip a test
    // tmp_path fixture	         | tempfile crate	            | Temporary directories
}

// Parameterized Tests with rstest
fn rstest() {
    // // Cargo.toml: rstest = "0.23"

    // use rstest::rstest;

    // // Like @pytest.mark.parametrize
    // #[rstest]
    // #[case(1, 2, 3)]
    // #[case(0, 0, 0)]
    // #[case(-1, -1, -2)]
    // #[case(100, 200, 300)]
    // fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    //     assert_eq!(add(a, b), expected);
    // }

    // // Like @pytest.fixture
    // use rstest::fixture;

    // #[fixture]
    // fn sample_data() -> Vec<i32> {
    //     vec![1, 2, 3, 4, 5]
    // }

    // #[rstest]
    // fn test_sum(sample_data: Vec<i32>) {
    //     assert_eq!(sample_data.iter().sum::<i32>(), 15);
    // }
}

// Mocking
fn mocking_usage() {
    // // Rust — mocking with mockall crate
    // // Cargo.toml: mockall = "0.13"

    // use mockall::{automock, predicate::*};

    // #[automock]                          // Generates MockDatabase automatically
    // trait Database {
    //     fn get_user(&self, id: i64) -> Option<User>;
    // }

    // fn fetch_user_name(db: &dyn Database, id: i64) -> Option<String> {
    //     db.get_user(id).map(|u| u.name)
    // }

    // #[test]
    // fn test_fetch_user() {
    //     let mut mock = MockDatabase::new();
    //     mock.expect_get_user()
    //         .with(eq(1))                   // assert_called_with(1)
    //         .times(1)                      // assert_called_once
    //         .returning(|_| Some(User { name: "Alice".into() }));

    //     let result = fetch_user_name(&mock, 1);
    //     assert_eq!(result, Some("Alice".to_string()));
    // }
}
fn main() {
    mocking_usage();
    // rstest();
    // ffi_docs();
    // pyo3_usage();
    // unsafe_usage();
}
