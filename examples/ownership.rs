// Ownership: the rule that makes Rust memory-safe without a garbage collector.
//     cargo run --example ownership
//
// Three rules:
//   1. Every value has exactly one owner.
//   2. There can only be one owner at a time.
//   3. When the owner goes out of scope, the value is dropped (freed).

fn main() {
    // String owns a buffer on the heap. The variable owns the String.
    let s1 = String::from("hello");
    let s2 = s1; // MOVE, not copy. s1 is now invalid.

    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
    println!("s2 = {s2}");

    // Clone makes a deep copy, so both stay valid. Not free.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Simple stack-only types implement Copy, so assignment copies.
    // Copy types: integers, floats, bool, char, tuples/arrays of Copy types.
    let n1 = 5;
    let n2 = n1;
    println!("n1={n1} n2={n2}");

    // Passing to a function moves ownership in.
    let s = String::from("takes ownership");
    takes_ownership(s);
    // println!("{s}"); // moved into the function

    // Returning gives ownership back out.
    let s = gives_ownership();
    let s = and_gives_back(s);
    println!("returned: {s}");

    // Borrowing (next example) avoids all this moving.
    let len = calculate_length(&s);
    println!("len = {len}");

    // Ownership is also why this can't happen:
    // let a = String::from("x");
    // let b = &a;
    // drop(a);          // error[E0505]: cannot move out of `a` because it is borrowed
    // println!("{b}");
}

fn takes_ownership(s: String) {
    println!("inside: {s}");
} // s is dropped here, memory freed

fn gives_ownership() -> String {
    String::from("yours")
}

fn and_gives_back(s: String) -> String {
    s
}

// Takes &str, not &String. Accept the most general form you can.
fn calculate_length(s: &str) -> usize {
    s.len()
}
