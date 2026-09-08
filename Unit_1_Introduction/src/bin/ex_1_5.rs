// 🏋️ Exercise: Word Frequency Counter (click to expand)
// Challenge: Write a function that takes a &str sentence and returns a HashMap<String, usize> of word frequencies (case-insensitive). In Python this is Counter(s.lower().split()). Translate it to Rust.

#![allow(unused)]
use std::collections::HashMap;

fn word_freq(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for n in text.split_whitespace() {
        let key = n.to_lowercase();
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy fox";
    let freq = word_freq(text);
    for (word, count) in &freq {
        println!("{word}: {count}");
    }
}
