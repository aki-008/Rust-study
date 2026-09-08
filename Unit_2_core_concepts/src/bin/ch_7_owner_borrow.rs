#![allow(unused)]

//  ⭕ Rust: Single Ownership
fn single_owner() {
    // Rust — every value has exactly ONE owner
    let a = vec![1, 2, 3];
    let b = a; // Ownership MOVES from a to b
    // println!("{:?}", a); // ❌ Compile error: value used after move

    // a no longer exists. b is the sole owner.
    println!("{:?}", b); // ✅ [1, 2, 3]

    // When b goes out of scope, the Vec is freed. Deterministic. No GC.
}

// 1. Each value has exactly ONE owner variable.
// 2. When the owner goes out of scope, the value is dropped (freed).
// 3. Ownership can be transferred (moved) but not duplicated (unless Clone).

//  ⭕ Move Semantics — The Biggest Python Shock
fn move_owner() {
    // Rust — passing to a function MOVES ownership (for non-Copy types)
    fn process(mut data: Vec<i32>) -> Vec<i32> {
        data.push(42);
        data // Must return it to give ownership back!
    }

    let my_vec = vec![1, 2, 3];
    let my_vec = process(my_vec); // Ownership moves in and back out
    println!("{:?}", my_vec); // [1, 2, 3, 42]

    // Or better — borrow instead of moving:
    fn process_borrowed(data: &mut Vec<i32>) {
        data.push(42);
    }

    let mut my_vec = vec![1, 2, 3];
    process_borrowed(&mut my_vec); // Lend it temporarily
    println!("{:?}", my_vec); // [1, 2, 3, 42] — still ours
}

//   Move Semantics vs Reference Counting
// ⭕ Copy vs Move
fn copy_n_move() {
    // Simple types (integers, floats, bools, chars) are COPIED, not moved
    let x = 42;
    let y = x; // x is COPIED to y (both valid)
    println!("x:{x} , y:{y}"); // ✅ 42 42

    // Heap-allocated types (String, Vec, HashMap) are MOVED
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{s1}");  // ❌ Error: value used after move

    // To explicitly copy heap data, use .clone()
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy
    println!("s1:{s1}, s2:{s2}"); // ✅ hello hello (both valid)
}

fn ref_share() {
    use std::collections::HashSet;

    fn remove_duplicates(items: &[i32]) -> Vec<i32> {
        let mut seen = HashSet::new();
        items
            .iter()
            .filter(|&&item| seen.insert(item))
            .copied()
            .collect()
    }

    let original = vec![1, 2, 2, 3, 3, 3, 3];
    let unique = remove_duplicates(&original); // Borrows — can't modify
    // original is guaranteed unchanged — compiler prevented mutation via &
    println!("{:?}", original);
    println!("{:?}", unique);
}

//  ⭕ Borrowing and Lifetimes
// Borrowing = Lending a Book
// Think of ownership like a physical book:

// Python:  Everyone has a photocopy (shared references + GC)
// Rust:    One person owns the book. Others can:
//          - &book     = look at it (immutable borrow, many allowed)
//          - &mut book = write in it (mutable borrow, exclusive)
//          - book      = give it away (move)

fn borrow_rules() {
    // Rule 1: You can have MANY immutable borrows OR ONE mutable borrow (not both)

    let mut data = vec![1, 2, 3];

    // Multiple immutable borrows — fine
    let a = &data;
    let b = &data;
    println!("{:?} {:?}", a, b); // ✅

    // Mutable borrow — must be exclusive
    let c = &mut data;
    c.push(4);
    // println!("{:?}", a);  // ❌ Error: can't use immutable borrow while mutable exists

    // This prevents data races at compile time!
    // Python has no equivalent — it's why Python dict modified-during-iteration crashes at runtime.
}

//  ⭕ Lifetimes — A Brief Introduction

fn lifetime_intro() {
    // Lifetimes answer: "How long does this reference live?"
    // Usually the compiler infers them. You rarely write them explicitly.

    // Simple case — compiler handles it:
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    // The compiler knows: the returned &str lives as long as the input &str

    // When you need explicit lifetimes (rare):
    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() { a } else { b }
    }
    // 'a says: "the return value lives as long as both inputs"

    // For Python developers: Don’t worry about lifetimes initially. The compiler will tell you when you need them, and 95% of the time it infers them automatically. Think of lifetime annotations as hints you give the compiler when it can’t figure out the relationships on its own.
}

//  ⭕ Smart Pointers
// For cases where single ownership is too restrictive, Rust provides smart pointers. These are closer to Python’s reference model — but explicit and opt-in.

fn smt_pointer() {
    // Box<T> — heap allocation with single owner (like Python's normal allocation)
    let boxed = Box::new(42); // Heap-allocated i32

    // Rc<T> — reference counted (like Python's refcount!)
    use std::rc::Rc;
    let shared = Rc::new(vec![1, 2, 3]);
    let clone1 = Rc::clone(&shared); // Increment refcount
    let clone2 = Rc::clone(&shared); // Increment refcount
    // All three point to the same Vec. When all are dropped, Vec is freed.
    // Similar to Python's reference counting, but Rc does NOT handle cycles
    println!(
        "shared: {:?}, clone1: {:?}, clone2: {:?}",
        shared, clone1, clone2
    );
    // use Weak<T> to break cycles (Python's GC handles cycles automatically)

    // Arc<T> — atomic reference counting (Rc for multi-threaded code)
    use std::sync::Arc;
    let thread_safe = Arc::new(vec![1, 2, 3]);
    // Use Arc when sharing across threads (Rc is single-threaded)

    // RefCell<T> — runtime borrow checking (like Python's "anything goes" model)
    use std::cell::RefCell;
    let cell = RefCell::new(42);
    *cell.borrow_mut() = 99; // Mutable borrow at runtime (panics if double-borrowed)
}

fn main() {
    smt_pointer();
    // borrow_rules();
    // ref_share();
    // copy_n_move();
    // move_owner();
    // single_owner();
}
