// Collections, strings, and iterators.
//     cargo run --example collections

use std::collections::HashMap;

fn main() {
    // Vec<T>: growable array on the heap.
    let mut v: Vec<i32> = Vec::new();
    for x in [3, 1, 2] {
        v.push(x);
    }
    v.sort();
    println!("v={v:?} len={} first={}", v.len(), v[0]); // v[0] panics if empty

    for (i, x) in v.iter().enumerate() {
        println!("  v[{i}] = {x}");
    }

    // get returns Option, use it when the index might not exist.
    println!("v.get(99) = {:?}", v.get(99));

    // Iterator adapters are lazy. Nothing happens until a consumer runs.
    // adapters: map, filter, take, skip, chain, zip, rev, flat_map
    // consumers: collect, sum, count, fold, for_each, any, all, find
    let evens: Vec<i32> = v.iter().copied().filter(|x| x % 2 == 0).collect();
    println!("evens={evens:?} sum={}", evens.iter().sum::<i32>());

    let total = (1..=10).filter(|n| n % 3 == 0).sum::<i32>();
    println!("sum of 1..=10 divisible by 3 = {total}");

    // HashMap. Insert returns the old value as Option.
    let mut scores = HashMap::new();
    scores.insert("alice", 10);
    scores.insert("bob", 7);
    println!("old alice score = {:?}", scores.insert("alice", 11));

    // The canonical counter idiom: entry API.
    *scores.entry("alice").or_insert(0) += 5;
    *scores.entry("carol").or_insert(0) += 1;

    println!("scores={scores:?}");
    if let Some(s) = scores.get("bob") {
        println!("bob={s}");
    }
    // Iteration order is not sorted and can change between runs.
    for (name, score) in &scores {
        println!("  {name}: {score}");
    }

    // String is UTF-8 and owns a growable buffer. &str is a borrowed slice.
    let mut s = String::from("héllo");
    s.push_str(" world");
    s.push('!');
    println!("{s} | bytes={} chars={}", s.len(), s.chars().count());
    // s[0] is a compile error: a char can span multiple bytes.
    for (i, ch) in s.char_indices().take(3) {
        println!("  byte {i}: {ch}");
    }
    println!("upper={}", s.to_uppercase());
    println!("split={:?}", s.split_whitespace().collect::<Vec<_>>());
    println!("starts_with h = {}", s.starts_with('h'));

    // Owned vs borrowed in function signatures:
    //   fn f(s: &str)         read-only text, accepts String and &str
    //   fn f(s: &mut String)  append to caller's String
    //   fn f(s: String)       take ownership, caller loses it
    fn shout(s: &str) -> String {
        format!("{}!!!", s.to_uppercase().trim())
    }
    println!("{}", shout(&s));
}
