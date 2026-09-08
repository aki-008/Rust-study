// 🏋️ Exercise: First Rust Program (click to expand)
// Challenge: Create a new Rust project and write a program that:

// Declares a variable name with your name (type &str)
// Declares a mutable variable count starting at 0
// Uses a for loop from 1..=5 to increment count and print "Hello, {name}! (count: {count})"
// After the loop, print whether count is even or odd using a match expression

fn main() {
    let name: &str = "nik";
    let mut count = 0u32;

    for _ in 1..=5 {
        count += 1;
        println!("Hello, {name}! (count:{count}");
    }

    let parity = match count % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("Final count {count} is {parity}");
}
