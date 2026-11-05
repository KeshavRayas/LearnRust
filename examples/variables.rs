// Variables, types, mutability, shadowing.
//     cargo run --example variables

const MAX_LEVEL: u32 = 99; // const: compile-time, needs a type, can live anywhere

fn main() {
    // `let` is immutable by default. This is the single biggest difference from
    // most languages you already know.
    let x = 5;
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`

    let mut y = 5; // opt in to mutation
    y += 1; // y == 6

    // Shadowing: a new binding that happens to reuse the name. Can change type.
    let spaces = "   ";
    let spaces = spaces.len();

    // Type annotations are usually inferred. Parsing needs one (or a later use).
    let guess: u32 = "42".parse().expect("not a number");

    // Integer types: i8/i16/i32/i64/i128, u8..u128, plus isize/usize (pointer size).
    // Debug builds panic on overflow; release builds wrap. Be explicit when it matters.
    let wrapped: u8 = 5u8.wrapping_add(255); // 4
    let saturated: u8 = 5u8.saturating_add(255); // 255

    let f: f64 = 3.5;
    let ok: bool = true;
    let letter: char = 'é'; // 4 bytes, one Unicode scalar value, not a byte

    // Tuple: fixed length, mixed types.
    let tup: (i32, f64, char) = (500, 6.4, 'x');
    let (a, b, c) = tup; // destructure
    println!("tup.0 = {}, destructured: {a} {b} {c}", tup.0);

    // Array: fixed length, one type, lives on the stack.
    let arr: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &arr[1..]; // a borrowed view, length known at runtime

    // println! is a macro (note the !). `{}` uses Display, `{:?}` uses Debug.
    println!("x={x} y={y} spaces={spaces} guess={guess} MAX={MAX_LEVEL}");
    println!("wrapped={wrapped} saturated={saturated} f={f} ok={ok} letter={letter}");
    println!("arr={arr:?} first={} slice={slice:?}", arr[0]);
    println!("arr.len()={} slice.len()={}", arr.len(), slice.len());
}
