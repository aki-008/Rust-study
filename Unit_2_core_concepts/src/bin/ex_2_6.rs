#![allow(unused)]

// 🏋️ Exercise: Derive and Custom Debug (click to expand)
// Challenge: Create a User struct with fields name: String, email: String, and password_hash: String. Derive Clone and PartialEq, but implement Debug manually so it prints the name and email but redacts the password (shows "***" instead).

use std::fmt;
#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password_hash", &"****")
            .finish()
    }
}

fn main() {
    let user = User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
        password_hash: "a1b2c3d4e5f6".into(),
    };
    println!("{user:?}");
    // Output: User { name: "Alice", email: "alice@example.com", password_hash: "***" }
}
