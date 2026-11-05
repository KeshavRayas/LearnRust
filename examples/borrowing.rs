// Borrowing: references without taking ownership.
//     cargo run --example borrowing
//
// Rules the compiler enforces at any one time, for one value:
//   - any number of immutable references (&T), OR
//   - exactly one mutable reference (&mut T), never both.
// These are checked at compile time, and the borrow ends at its last use
// (non-lexical lifetimes), so the scope can be wider than the borrow.

fn main() {
    let mut s = String::from("hello");

    // Many readers at once.
    let r1 = &s;
    let r2 = &s;
    println!("{r1} / {r2}");

    // r1 and r2 were last used above, so their borrows are over. Now a writer fits.
    let r3 = &mut s;
    r3.push_str(", world");
    println!("{r3}");

    // Readers and a writer alive together is an error:
    // let a = &s;
    // let b = &mut s;
    // println!("{a} {b}"); // error[E0502]: cannot borrow `s` as mutable because
    //                      //               it is also borrowed as immutable
    //
    // Two writers is also an error:
    // let b1 = &mut s;
    // let b2 = &mut s;
    // println!("{b1} {b2}"); // error[E0499]

    // Mutation requires the binding itself to be mut.
    // let frozen = String::from("x");
    // frozen.push('y'); // error[E0596]: cannot borrow as mutable

    // Slices borrow too. The borrow lives as long as the slice is used.
    let word = first_word(&s);
    println!("first word: {word}");
    // s.clear(); // error[E0502]: cannot borrow `s` as mutable while `word` exists
    println!("still borrowed: {word}");

    // After the last use of `word`, mutation is fine again.
    s.push('!');
    println!("{s}");

    // A function may take a mutable reference and change the caller's value.
    append_bang(&mut s);
    println!("after append_bang: {s}");

    // Dangling references are rejected at compile time:
    // fn dangle() -> &String { let s = String::from("x"); &s } // error[E0106]
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn append_bang(s: &mut String) {
    s.push('!');
}
