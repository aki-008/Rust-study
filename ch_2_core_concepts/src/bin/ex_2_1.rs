fn main() {
    let mut names = vec!["Alice".to_string(), "Bob".to_string()];
    let first = &names[0];
    println!("First: {first}");
    names.push("Charlie".to_string());

    let greeting = make_greeting(&names[0]);
    println!("{greeting}");
}

fn make_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}
