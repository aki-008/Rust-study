fn main() {
    let mut names = vec!["Alice".to_string(), "Bob".to_string()];
    let first = &names[0];
    println!("First: {first}"); // Use borrow BEFORE mutating
    names.push("Charlie".to_string()); // Now safe — no live immutable borrow

    let greeting = make_greeting(&names[0]); // Pass reference, not owned
    println!("{greeting}");
}

fn make_greeting(name: &str) -> String { // Accept &str, not String
    format!("Hello, {name}!")
}