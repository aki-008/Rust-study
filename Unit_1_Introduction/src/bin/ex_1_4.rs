// 🏋️ Exercise: FizzBuzz with Expressions (click to expand)
// Challenge: Write FizzBuzz for 1..=30 using Rust’s expression-based match. Each number should print “Fizz”, “Buzz”, “FizzBuzz”, or the number. Use match (n % 3, n % 5) as the expression.

fn main() {
    for n in 1..=30 {
        let result = match (n % 3, n % 5) {
            (0, 0) => String::from("Fizzbuzz"),
            (_, 0) => String::from("Buzz"),
            (0, _) => String::from("Fizz"),
            _ => n.to_string(),
        };
        println!("{}", result)
    }
}

// Key takeaway: match is an expression that returns a value — no need for if/elif/else chains. The _ wildcard replaces Python’s case _: default.
