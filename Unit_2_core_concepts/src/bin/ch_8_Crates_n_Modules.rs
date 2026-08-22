#![allow(unused)]

//  ⭕ Rust Module System
fn rs_modules() {
    // Rust — mod declarations create the module tree, files provide content

    // src/
    // ├── main.rs             # Crate root — declares modules
    // ├── utils/
    // │   ├── mod.rs           # Module declaration (like __init__.py)
    // │   ├── helpers.rs
    // │   └── validators.rs
    // └── models/
    //     ├── mod.rs
    //     ├── user.rs
    //     └── product.rs

    // // In src/main.rs:
    // mod utils; // Tells Rust to look for src/utils/mod.rs
    // mod models; // Tells Rust to look for src/models/mod.rs

    // use models::user::User;
    // use utils::helpers::format_name;

    // // In src/utils/mod.rs:
    // pub mod helpers; // Declares and re-exports helpers.rs
    // pub mod validators; // Declares and re-exports validators.rs

    // Python equivalent: Think of mod.rs as __init__.py — it declares what the module exports. The crate root (main.rs / lib.rs) is like your top-level package __init__.py.
}

//  ⭕ Visibility — Private by Default
fn visibility_scope() {
    // Rust — private is enforced by the compiler
    pub struct User {
        pub name: String, // Public — anyone can access
        age: i32,         // Private — only this module can access
    }

    impl User {
        pub fn new(name: &str, age: i32) -> Self {
            User {
                name: name.to_string(),
                age,
            }
        }

        pub fn age(&self) -> i32 {
            // Public getter
            self.age
        }

        fn validate(&self) -> bool {
            // Private method
            self.age > 0
        }
    }

    // Outside the module:
    let user = User::new("Alice", 30);
    println!("{}", user.name); // ✅ Public
    // println!("{}", user.age);      // ❌ Compile error: field is private
    println!("{}", user.age()); // ✅ Public method (getter)
}

fn main() {
    // visibility_scope();
    // rs_modules();
}
