// Lifetimes: how the compiler tracks borrow validity.
//     cargo run --example lifetimes
//
// A lifetime is a name for a region of code. It has no runtime cost and no
// runtime representation. You annotate it when the compiler cannot infer which
// input a returned reference came from.

// Elision fails here. Two input references, neither is &self, so the compiler
// will not guess which one the output borrows from. This is the classic E0106:
//
//     fn longest(x: &str, y: &str) -> &str
//         // error[E0106]: missing lifetime specifier
//
// Annotate it. 'a is the region where both inputs are valid, so the output is
// valid at most as long as the shorter-lived input.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// A struct holding a reference needs a lifetime parameter. It can never outlive
// the data it points at.
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    // Elision rule for methods: the output gets the lifetime of &self.
    // So this returns &'a str, not tied to `note`.
    fn announce(&self, note: &str) -> &str {
        println!("attention: {note}");
        self.part
    }
}

// 'static means "valid for the whole program". String literals are &'static str.
fn static_str() -> &'static str {
    "forever"
}

fn main() {
    let a = String::from("long string is long");
    let b = "xyz";
    println!("{}", longest(&a, b));
    println!("{}", longest(&a, b));

    let e = Excerpt { part: &a[..4] };
    println!("{e:?} -> {}", e.announce("read this"));

    // Lifetimes are why this does not compile. The result would point at `inner`,
    // which dies at the end of the block.
    let s;
    {
        let inner = String::from("inner");
        // s = longest(&a, &inner); // error[E0597]: `inner` does not live long enough
        s = longest(&a, "literal"); // ok: literal is 'static
        drop(inner); // a borrow of `inner` would already be dead here
    }
    println!("{s}");

    println!("{}", static_str());

    // Common lifetime errors and what they actually mean:
    //
    // E0106 "missing lifetime specifier"
    //   A function returns a reference but takes more than one reference.
    //   Tell the compiler which input it comes from.
    //
    // E0597 "does not live long enough"
    //   You are returning or storing a borrow of something that is dropped
    //   before the borrow ends. Return an owned value instead.
    //
    // E0716 "temporary value dropped while borrowed"
    //   Usually a method call returning a reference to a temporary:
    //     let s = String::from("x").as_str(); // temporary dies at end of statement
    //   Bind the owned value to a variable first, then borrow it.
}
