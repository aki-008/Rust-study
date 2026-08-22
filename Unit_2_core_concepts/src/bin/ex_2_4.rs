#![allow(unused)]

use std::iter::Sum;

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    body: String,
}
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!(
            "{} - {}...",
            self.title,
            &self.body[..20.min(self.body.len())],
        )
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn main() {
    let article = Article {
        title: "Rust is great".into(),
        body: "Here is why Rust beats Python for systems ....".into(),
    };
    let tweet = Tweet {
        username: "rustacean".into(),
        content: "Just shipped my first crate!".into(),
    };

    notify(&article);
    notify(&tweet);
}
