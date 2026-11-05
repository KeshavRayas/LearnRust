# Rust notes

Working notes for learning Rust, written against the project in this folder.
Rust 1.99, edition 2024, Windows.

Every section has a runnable example in `examples/`. Run them in this order:

```powershell
cargo run --example hello_world
cargo run --example variables
cargo run --example ownership
cargo run --example borrowing
cargo run --example structs_enums
cargo run --example errors
cargo run --example collections
cargo run --example traits_generics
cargo run --example lifetimes
cargo run --example concurrency
```

`cargo check` is the fast loop while writing. `cargo run` compiles and runs.

Contents

1. [Toolchain and project layout](#1-toolchain-and-project-layout)
2. [Mental model](#2-mental-model)
3. [Cargo](#3-cargo)
4. [Syntax essentials](#4-syntax-essentials)
5. [Ownership](#5-ownership)
6. [Borrowing](#6-borrowing)
7. [Lifetimes](#7-lifetimes)
8. [Structs, enums, impl](#8-structs-enums-impl)
9. [Pattern matching](#9-pattern-matching)
10. [Error handling](#10-error-handling)
11. [Traits and generics](#11-traits-and-generics)
12. [Collections and iterators](#12-collections-and-iterators)
13. [Smart pointers](#13-smart-pointers)
14. [Modules and crates](#14-modules-and-crates)
15. [Testing](#15-testing)
16. [Concurrency](#16-concurrency)
17. [Compiler errors decoded](#17-compiler-errors-decoded)
18. [Idioms that matter](#18-idioms-that-matter)
19. [Debugging workflow](#19-debugging-workflow)
20. [Cheatsheet](#20-cheatsheet)
21. [Four week plan](#21-four-week-plan)
22. [What to read next](#22-what-to-read-next)

---

## 1. Toolchain and project layout

Three tools:

- `rustup` installs and updates toolchains. `rustup show`, `rustup update`.
- `rustc` compiles a single file. You rarely call it directly.
- `cargo` is the build tool and package manager. You will use this every day.

Useful extras, already included with a normal install:

```powershell
rustup component add rustfmt clippy
cargo fmt                 # format the whole project
cargo clippy -- -D warnings   # lint, treat warnings as errors
```

On Windows the default toolchain is `stable-x86_64-pc-windows-msvc`, which needs
the Visual Studio C++ build tools for the linker. If you ever see
`error: linker 'link.exe' not found`, install "Desktop development with C++" in
the Visual Studio Installer. The `stable-x86_64-pc-windows-gnu` toolchain avoids
that but the MSVC one is the normal choice.

Project layout:

```
LearnRust/
├── Cargo.toml        # package manifest: name, version, deps
├── Cargo.lock        # exact resolved versions (commit it for binaries)
├── src/
│   └── main.rs       # binary crate root
├── examples/         # runnable examples: cargo run --example NAME
│   └── hello_world.rs
└── target/           # build output, never commit
```

`src/lib.rs` makes it a library crate, and you can have both `lib.rs` and
`main.rs`. Code in `main.rs` is not importable; put reusable code in `lib.rs`.

The VS Code extension `rust-analyzer` gives inline errors, go to definition, and
inlay hints for lifetimes and inferred types. Turn inlay hints on if you are
learning, they show what the compiler inferred.

## 2. Mental model

Rust is compiled ahead of time to a native binary. No runtime, no garbage
collector, no VM.

Things that are different from Python, JS, Java, Go, and C#:

- Values have a single owner. When the owner goes out of scope the value is freed
  deterministically. No `finally`, no GC, no manual `free`.
- Borrowing is checked at compile time. You cannot have a dangling pointer or a
  data race in safe code.
- No `null`. Use `Option<T>`.
- No exceptions. Recoverable failures return `Result<T, E>`; unrecoverable bugs
  `panic!`.
- No inheritance. Share behavior with traits, compose with structs.
- No function overloading and no default arguments. Use generics, traits, or a
  builder.
- Expressions are values. `if`, `match`, and blocks all return values, so you
  write `let x = if cond { 1 } else { 2 };`.
- Immutable by default. Mutation is opt in with `mut`.
- Errors are values you handle, not control flow that unwinds the stack.

The compiler is strict and the error messages are usually better than the code
that caused them. Read the whole message, especially the `help:` part.

## 3. Cargo

Commands you need:

| Command | What it does |
| --- | --- |
| `cargo new name` | new binary package in `name/` |
| `cargo new name --lib` | new library package |
| `cargo init` | turn the current directory into a package |
| `cargo check` | type check, no codegen. Fastest feedback. |
| `cargo build` | debug build in `target/debug/` |
| `cargo build --release` | optimized build in `target/release/` |
| `cargo run` | build and run the binary |
| `cargo run --example borrowing` | build and run an example |
| `cargo run -- arg1 arg2` | pass args to your program, not cargo |
| `cargo test` | unit, integration, and doc tests |
| `cargo fmt` | format |
| `cargo clippy` | lints |
| `cargo doc --open` | build and open docs for your crate and deps |
| `cargo add serde --features derive` | add a dependency and pick features |
| `cargo tree` | dependency graph |
| `cargo clean` | delete `target/` |

`Cargo.toml`:

```toml
[package]
name = "learn_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
# serde = { version = "1", features = ["derive"] }

[profile.release]
opt-level = 3
lto = "thin"
```

Editions are per package and opt in to language changes. 2024 is current. They
are not compiler versions, all editions compile with the same `rustc`.

Debug builds are fast to compile and slow to run, and they panic on integer
overflow. Release builds optimize and wrap on overflow. Profiling a debug build
is a common beginner mistake.

## 4. Syntax essentials

```rust
fn main() {
    // bindings
    let x = 1;              // immutable
    let mut y = 2;           // mutable
    let z: u64 = 3;          // annotated type
    const LIMIT: u32 = 10;   // const, needs a type, inlined at use sites
    static NAME: &str = "x"; // static, one fixed address

    // shadowing creates a new binding and can change the type
    let spaces = "   ";
    let spaces = spaces.len();
    let _ = (spaces, LIMIT, NAME); // underscore prefix silences unused warnings
}
```

Scalar types: `i8 i16 i32 i64 i128 isize`, `u8 u16 u32 u64 u128 usize`,
`f32 f64`, `bool`, `char`. The default integer is `i32`, default float `f64`.
`usize` is the index and length type.

Compound types: tuples `(T, U)`, arrays `[T; N]`, slices `&[T]`, strings
`String` and `&str`.

Functions:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // last expression is the return value, no semicolon
}

fn nothing() {
    // returns ()
}
```

The semicolon matters. `a + b` returns the value. `a + b;` returns `()` and
fails to type check with E0308, "expected i32, found ()".

Control flow is expression oriented:

```rust
let label = if n % 2 == 0 { "even" } else { "odd" };

let n = loop {
    break 42; // loop can break with a value
};

for i in 0..5 { }        // 0,1,2,3,4
for x in &vec { }        // by reference
let mut i = 0;
while i < 5 { i += 1; }
```

`0..5` is a `Range`, exclusive. `0..=5` is inclusive. Ranges are iterators, so
they work with `for`, `map`, `sum`, and friends.

Naming conventions: `snake_case` for functions, variables, modules; `CamelCase`
for types and enum variants; `SCREAMING_SNAKE_CASE` for consts and statics.

## 5. Ownership

Three rules:

1. Every value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Assignment moves ownership for heap types:

```rust
let s1 = String::from("hello");
let s2 = s1;          // move: s1 is now invalid
// println!("{s1}");  // E0382: borrow of moved value
```

A `String` is a pointer, length, and capacity on the stack, pointing at a UTF-8
buffer on the heap. Copying the pointer would give you two owners and a double
free. So Rust moves and invalidates the source. Nothing is copied.

Types that are cheap and safe to copy implement `Copy`, and assignment copies
instead of moving: all integers, floats, `bool`, `char`, and tuples/arrays of
`Copy` types. `String`, `Vec<T>`, `Box<T>` are not `Copy`.

`Clone` is an explicit deep copy:

```rust
let s3 = s2.clone();
```

Clone is not a fix for the borrow checker. If you find yourself cloning to make
an error go away, stop and ask who should own the value.

Passing to a function moves. Returning moves back out.

```rust
fn consume(s: String) { }  // takes ownership
fn borrow(s: &str) { }     // reads without owning
```

Drop order: locals are dropped in reverse declaration order at the end of their
scope. Struct fields are dropped in declaration order. You rarely need to think
about this, except when you implement `Drop`.

## 6. Borrowing

Borrowing is how you use a value without taking ownership. Enforced rule for a
given value at a given time:

- any number of shared references `&T`, or
- exactly one mutable reference `&mut T`.
- Never both at once.

```rust
let mut s = String::from("hi");

let a = &s;
let b = &s;
println!("{a} {b}");   // both readers alive

let c = &mut s;        // ok: a and b are no longer used
c.push('!');
```

Borrows end at their last use, not at the end of the block. That is non-lexical
lifetimes (NLL). It is why the code above compiles even though `a` and `b` are
in the same scope as `c`.

Mutating through a reference needs both `mut` on the binding and `&mut`.

```rust
let s = String::from("hi");
// s.push('!');       // E0596, s is not declared mut
```

Slices borrow part of a collection and keep the whole collection borrowed:

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
// v.push(4);         // E0502, v is borrowed by first
println!("{first}");
v.push(4);            // ok, first is dead
```

Why the rule exists: a `&mut` promises exclusive access. If an `&` could see the
value at the same time, you could read a value while it is being reallocated or
replaced.

Common beginner loop error:

```rust
let mut v = vec![1, 2, 3];
// for x in &v { v.push(*x); }   // E0502: cannot borrow v as mutable
//                               // because it is also borrowed as immutable
```

Fix: collect into a new `Vec` first, or index with a copied length, or use
`v.clone()` when you genuinely want a snapshot.

`&str` and `&String`: `&String` derefs to `&str`, and `&str` accepts both. Write
`&str` in parameters unless you need `String` methods that allocate.

```rust
fn len(s: &str) -> usize { s.len() }
len("literal");           // &'static str
len(&String::from("x"));  // &String coerces to &str
```

## 7. Lifetimes

A lifetime is the region where a reference is valid. It is a compile-time
concept with zero runtime cost.

You only write lifetimes when the compiler cannot infer the relationship
between inputs and outputs.

Elision rules, in order:

1. Each elided input reference gets its own lifetime parameter.
2. If there is exactly one input lifetime, the output gets it.
3. If there is `&self` or `&mut self`, the output gets the lifetime of `self`.

So `fn first_word(s: &str) -> &str` works by rule 2, but
`fn longest(x: &str, y: &str) -> &str` fails by rule 1 and needs `'a`.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read `'a` as "some region that both inputs outlive". The returned reference is
valid for at most the shorter of the two.

Structs holding references need a lifetime:

```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

`'static` means valid for the entire program. String literals are `&'static str`.
`T: 'static` means T contains no borrows that could expire, not that the value
lives forever. Owned types like `String` are `'static`.

Error to recognize: "temporary value dropped while borrowed".

```rust
// let s = String::from("x").as_str();  // E0716
let owned = String::from("x");
let s = owned.as_str();                 // bind the owner first
```

The fix is almost always to keep the owned value in a variable, or return an
owned value instead of a reference.

If lifetime annotations get complicated, that is usually a design signal. Try
owning the data (`String` instead of `&str`) at the boundary, or restructure so
the borrow is short.

## 8. Structs, enums, impl

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self { Self { x, y } }   // associated fn
    fn dist_sq(&self) -> i32 { self.x * self.x + self.y * self.y }
    fn translate(&mut self, dx: i32) { self.x += dx; }
    fn into_tuple(self) -> (i32, i32) { (self.x, self.y) }
}
```

`Self` is the type being implemented. Methods take `self`, `&self`, or
`&mut self`. Associated functions have no self and are called as
`Point::new(...)`.

Enums carry data and each variant can differ:

```rust
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Empty,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

`Option<T>` and `Result<T, E>` are ordinary enums in the standard library. There
is nothing magic about them.

Useful derives: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`,
`PartialOrd`, `Ord`. Derive `Copy` only for small types where copying is
genuinely fine.

Tuple structs and newtypes:

```rust
struct Meters(f64);
let d = Meters(1.5);
println!("{}", d.0);

struct UserId(u64);   // newtype: type safety with zero runtime cost
```

## 9. Pattern matching

`match` is exhaustive. The compiler rejects a missing case (E0004), which is why
adding an enum variant breaks every match you forgot to update.

```rust
match value {
    0 => println!("zero"),
    1..=9 => println!("digit"),
    n if n < 0 => println!("negative {n}"),
    n => println!("other {n}"),
}
```

Patterns you will use:

```rust
let (a, b) = (1, 2);
let Point { x, y: renamed } = p;
let [first, .., last] = arr;
let Some(x) = opt else { return };   // let else, diverges on no match
if let Some(v) = opt { }
while let Some(v) = stack.pop() { }
matches!(value, Some(_))             // boolean test
```

`ref` is rarely needed since matching a reference binds references by default
(match ergonomics). In edition 2024, binding modifiers changed slightly; just
match on `&value` and let the compiler bind by reference.

Guards (`if cond`) run after the pattern matches and can use the bindings. `@`
binds and tests at the same time: `n @ 1..=9 => ...`.

## 10. Error handling

Two kinds of failure:

- bugs and broken invariants: `panic!`, `unwrap`, `expect`, indexing out of
  bounds, integer division by zero. These abort the current thread (unwind by
  default).
- expected failures: file missing, parse error, network down. Return
  `Result<T, E>`.

`Option<T>` is for absence, not failure. `Result<T, E>` is for failure with a
reason.

```rust
fn parse_positive(s: &str) -> Result<u32, String> {
    let n: i64 = s.parse().map_err(|e| format!("bad number: {e}"))?;
    if n < 0 { return Err(format!("negative: {n}")); }
    Ok(n as u32)
}
```

The `?` operator: on `Err`, returns early with that error converted through
`From`. On `Ok`, unwraps the value. It only works in functions returning
`Result` (or `Option`, or `Try`).

`main` can return a `Result`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: i32 = "42".parse()?;
    println!("{n}");
    Ok(())
}
```

Boxed error trait objects are fine for applications and quick scripts. For a
library, define a concrete error enum so callers can match on it.

```rust
#[derive(Debug)]
enum AppError { Parse(std::num::ParseIntError), Negative(i64) }

impl std::fmt::Display for AppError { /* ... */ }
impl std::error::Error for AppError { /* ... */ }
impl From<std::num::ParseIntError> for AppError { /* ... */ }
```

That boilerplate is what `thiserror` generates for libraries. Applications
usually use `anyhow`:

```rust
use anyhow::{Context, Result};

fn load() -> Result<Config> {
    let text = std::fs::read_to_string("config.toml").context("reading config")?;
    Ok(toml::from_str(&text)?)
}
```

`unwrap` is acceptable in tests, examples, and prototypes. `expect("why this
cannot fail")` is better than `unwrap` because the panic message explains the
assumption.

Never use `unwrap` on user input or I/O in a long-running program.

## 11. Traits and generics

A trait is a set of methods a type can implement. Think interface, but you can
implement it for types you did not define (with restrictions).

```rust
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> String { format!("{} makes a sound", self.name()) }
}

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
}
```

Bounds:

```rust
fn print_all<T: std::fmt::Display>(items: &[T]) { }

fn print_all2<T>(items: &[T])
where
    T: std::fmt::Display + Clone,
{ }

fn takes_any(x: &impl std::fmt::Display) { }   // argument position sugar
fn returns_any() -> impl Iterator<Item = u32> { 0..10 }  // opaque concrete type
```

Static dispatch with `<T: Trait>` generates a specialized copy per type: fast,
bigger binary. Dynamic dispatch with `&dyn Trait` or `Box<dyn Trait>` uses a
vtable: one copy of the code, a pointer indirection, and you can mix types in one
collection. Use generics by default, `dyn` when you need heterogeneity or to cut
code size.

`dyn Trait` requires the trait to be object safe: no generic methods, no
`Self` by value in method signatures, no associated constants.

Traits you will use constantly:

- `Debug` (`{:?}`) and `Display` (`{}`). Derive Debug, implement Display.
- `Clone` and `Copy`.
- `PartialEq` / `Eq` for `==`.
- `Hash` for HashMap keys.
- `Default` for `T::default()`.
- `From<T>` / `Into<T>` for conversions. Implement `From`, get `Into` free.
- `TryFrom` for fallible conversions.
- `Iterator`, which gives you all adapters for free.
- `AsRef<str>` / `AsRef<Path>` for flexible arguments.
- `Deref` and `DerefMut` for smart pointer behavior. Do not implement Deref just
  to reuse methods; it is for pointer-like types.

Orphan rule: you can implement a trait for a type only if the trait or the type
is local to your crate. Newtype wrappers let you work around it.

Blanket impls apply to everything matching a bound, for example
`impl<T: Display> ToString for T`.

## 12. Collections and iterators

| Type | Use |
| --- | --- |
| `Vec<T>` | growable array, the default list |
| `VecDeque<T>` | push/pop both ends |
| `HashMap<K, V>` | key lookups, unordered |
| `BTreeMap<K, V>` | sorted keys |
| `HashSet<T>` / `BTreeSet<T>` | membership, dedupe |
| `String` | owned UTF-8 text |
| `&str` | borrowed text |

`Vec` essentials: `push`, `pop`, `get`, `iter`, `iter_mut`, `sort`,
`sort_by_key`, `retain`, `dedup`, `contains`, `windows`, `chunks`, `extend`,
`capacity`, `with_capacity`.

`HashMap` essentials:

```rust
let mut m = std::collections::HashMap::new();
m.insert("a", 1);
m.get("a");                    // Option<&i32>
m.get_mut("a");                // Option<&mut i32>
*m.entry("b").or_insert(0) += 1;   // insert if missing, then mutate
m.remove("a");
for (k, v) in &m { }
```

Iterators are lazy. Adapters build a chain, a consumer runs it.

```rust
let v: Vec<i32> = (1..=10)
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .collect();
```

Adapters: `map`, `filter`, `filter_map`, `flat_map`, `flatten`, `take`,
`skip`, `take_while`, `skip_while`, `chain`, `zip`, `rev`, `enumerate`,
`inspect`, `peekable`.

Consumers: `collect`, `sum`, `product`, `count`, `fold`, `reduce`, `for_each`,
`any`, `all`, `find`, `position`, `min`, `max`, `min_by_key`, `max_by_key`,
`last`, `nth`.

`iter()` yields `&T`. `iter_mut()` yields `&mut T`. `into_iter()` yields `T` and
consumes the collection.

`collect` often needs a target type. Either annotate the binding or use a
turbofish:

```rust
let set: std::collections::HashSet<_> = nums.iter().collect();
let v = nums.iter().map(|n| n * 2).collect::<Vec<_>>();
```

Strings are UTF-8, so indexing with `s[0]` is a compile error. Use `.chars()`,
`.bytes()`, `.char_indices()`, or a byte-safe slice when you know the boundaries.
`len()` is bytes, not characters.

```rust
let s = String::from("héllo");
s.len();                    // 6 bytes
s.chars().count();          // 5 characters
s.split_whitespace();
s.trim();
s.replace("l", "L");
s.parse::<i32>();
format!("{s} {n}");
```

Building strings in a loop: `push_str`, `push`, or collect an iterator of
`&str` and `join`.

## 13. Smart pointers

| Type | Use |
| --- | --- |
| `Box<T>` | heap allocation, recursive types, `Box<dyn Trait>` |
| `Rc<T>` | shared ownership, single thread |
| `Arc<T>` | shared ownership, across threads |
| `RefCell<T>` | interior mutability, checked at runtime |
| `Mutex<T>` | interior mutability across threads, locks |
| `Cell<T>` | copyable interior mutability, no borrow checking |

Recursive enum without indirection is an infinite size error (E0072):

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

`Rc<T>` is a reference-counted shared owner. `Rc::clone(&rc)` increments the
count, it does not deep copy. It is not `Send`, so it cannot cross threads.

`RefCell<T>` moves borrow checking to runtime. `borrow()` and `borrow_mut()`
return guards. Violating the rules panics instead of failing to compile. The
common combination `Rc<RefCell<T>>` gives you shared mutable state in one thread.
It is a legitimate tool and also a way to smuggle bad design past the compiler.
Reach for it when a graph or observer pattern genuinely needs it.

`Mutex<T>` is the thread version. `lock()` returns `Result` because a panicking
thread poisons the mutex. The guard releases the lock when dropped.

`Deref` makes `Box<T>` and `String` behave like `&T` in method calls and coercions.
`Drop` runs cleanup when a value goes out of scope.

## 14. Modules and crates

A crate is one compilation unit: a binary or a library. A package is a
`Cargo.toml` with one or more crates. Modules organize code inside a crate.

```rust
mod geometry {                 // inline module
    pub struct Point { pub x: f64, pub y: f64 }

    pub fn area(p: &Point) -> f64 { p.x * p.y }

    mod inner { }              // private to geometry
}

use geometry::Point;
```

In separate files, `mod geometry;` in `main.rs` looks for `src/geometry.rs` or
`src/geometry/mod.rs`. Submodules go in `src/geometry/shapes.rs` and are
declared with `mod shapes;` inside `geometry.rs`.

Visibility: private by default. `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`.

Paths: `crate::` is the current crate root, `super::` is the parent module,
`self::` is the current module.

External crates: add to `Cargo.toml` or `cargo add`, then `use crate_name::Type;`.
The crate name uses underscores in code even if the package name uses hyphens.

Workspaces group multiple packages under one `Cargo.toml` with a shared
`target/` and `Cargo.lock`. Useful once a project has a library plus a CLI.

## 15. Testing

Unit tests live next to the code and can test private items.

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "boom")]
    fn panics_with_message() {
        panic!("boom");
    }

    #[test]
    fn returns_result() -> Result<(), String> {
        if add(1, 1) == 2 { Ok(()) } else { Err("math is broken".into()) }
    }
}
```

Integration tests go in `tests/` and use the crate as an external user would:
`use learn_rust::something;`. They only work for library crates.

Doc tests are code blocks in `///` comments. `cargo test` compiles and runs them,
which keeps examples honest.

```powershell
cargo test
cargo test adds_positive          # filter by name
cargo test -- --nocapture         # show println output
cargo test -- --ignored           # run #[ignore] tests
```

Assertions: `assert!`, `assert_eq!`, `assert_ne!`, plus
`assert_eq!(a, b, "context {}", x)`.

`#[cfg(test)]` keeps the test module out of release builds.

## 16. Concurrency

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

let h = thread::spawn(|| println!("hi"));
h.join().unwrap();                     // Err if the thread panicked

let msg = String::from("owned");
thread::spawn(move || println!("{msg}"));   // move captures into the thread
```

Threads need `'static` closures because the thread may outlive the caller's
stack. `move` handles that by transferring ownership. `thread::scope` lifts the
`'static` requirement and joins automatically at the end of the scope.

Channels:

```rust
let (tx, rx) = mpsc::channel();
thread::spawn(move || tx.send(42).unwrap());
for value in rx { println!("{value}"); }   // ends when all senders drop
```

Shared state:

```rust
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || {
    let mut n = c.lock().unwrap();
    *n += 1;
});
```

`Send` means a type can move to another thread. `Sync` means `&T` can be shared
across threads. Both are auto traits: the compiler derives them from the fields.
`Rc` and `RefCell` are not thread safe and fail to compile when used across
threads, which is the point.

Deadlocks still exist. Do not lock the same mutex twice in one thread, and keep
lock scopes short.

Async Rust is a different model, built on futures and an executor. `async fn`
returns a `Future` that does nothing until awaited. You need a runtime such as
`tokio`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    let body = reqwest::get("https://example.com").await.unwrap().text().await.unwrap();
    println!("{}", body.len());
}
```

Do not block inside async code (`std::thread::sleep`, blocking I/O, CPU-heavy
loops). Use `tokio::time::sleep`, async I/O, and `spawn_blocking` for CPU work.
Learn sync threads first; async adds a runtime, pinning, and `Send` bounds that
will confuse you before ownership is automatic.

## 17. Compiler errors decoded

**E0382 borrow of moved value.** You used a value after giving it away. Fix:
borrow (`&`), clone if you really need two owners, or restructure so the value is
returned.

```rust
let s = String::from("x");
let t = s;
println!("{s}");   // E0382
```

**E0502 cannot borrow as mutable because it is also borrowed as immutable.**
A reader is still alive. Move the mutation after the last use of the reader, or
scope the borrow.

**E0499 cannot borrow as mutable more than once.** Two `&mut` at the same time.
Usually a loop or two struct fields borrowed via methods. Split the struct or
restructure.

**E0596 cannot borrow as mutable.** The binding is not `mut`, or you have `&T`
where you need `&mut T`.

**E0505 cannot move out of `x` because it is borrowed.** Drop the borrow first,
or clone the borrowed data.

**E0106 missing lifetime specifier.** A function returns a reference and takes
more than one reference, or a struct holds a reference without a lifetime.
Annotate which input the output comes from.

**E0597 `x` does not live long enough.** You return or store a borrow of a local.
Return an owned value.

**E0716 temporary value dropped while borrowed.** You borrowed from a temporary.
Bind the owned value to a variable first.

**E0308 mismatched types.** Read the "expected / found" pair. Common causes:
missing `&`, `String` vs `&str`, integer type mismatch, `()` vs a value because
of a stray semicolon.

**E0277 trait bound not satisfied.** The type does not implement the trait. Read
which trait and which type, then derive it or implement it. `{:?}` needs `Debug`,
`{}` needs `Display`, `+` needs `Add`, sorting needs `Ord`.

**E0433 / E0432 unresolved path or import.** Missing `use`, wrong module path,
dependency not added, or a private item.

**E0061 wrong number of arguments.** No default arguments and no overloading.

**E0004 non-exhaustive match.** Add the missing variant, or use `_ => ...`.
Adding an enum variant and watching matches fail is a feature.

Borrow checker heuristics that actually work:

1. Shrink the borrow: put the `&` in a smaller block or after the mutation.
2. Borrow a field, not the whole struct, when the borrow checker complains about
   two fields.
3. Return owned data at API boundaries instead of references.
4. Split a function that both borrows and mutates into two.
5. Use indices instead of references in loops over a collection you mutate.
6. Do not clone first. Understand who owns the value, then decide.

## 18. Idioms that matter

- Take `&str` instead of `&String`, `&[T]` instead of `&Vec<T>`. Accept the
  most general borrowed form.
- Return `String` when the caller needs to own it, `&str` when borrowing is
  enough.
- Use `?` instead of nested `match`.
- Use `if let` and `let else` for one-variant cases.
- Use iterators for transforms, plain `for` for side effects. Both are idiomatic.
- `Vec::with_capacity(n)` when you know the size.
- Use `entry` for map insert-or-update, not `contains_key` plus `insert`.
- Derive `Debug` on nearly every type. It costs nothing and saves debugging time.
- Implement `Display` for user-facing output, keep `Debug` for developers.
- Use newtypes for ids and units (`struct UserId(u64)`, `struct Meters(f64)`).
- Prefer `&[T]` and slices over `Vec<T>` in read-only function parameters.
- Avoid `unwrap` in library code. Use `expect` with a reason if the invariant is
  real.
- `#[must_use]` on functions whose return value matters.
- Keep `unsafe` in a small module with a comment explaining the invariant.
- Run `cargo clippy` before calling code done. It catches real bugs, not just
  style.
- `dbg!(x)` prints file, line, and value, and returns the value, so you can drop
  it into an expression temporarily.
- Do not fight the borrow checker with `Rc<RefCell<>>`. It usually means the
  ownership model is wrong.

## 19. Debugging workflow

1. `cargo check` after every small change. It is much faster than `cargo build`.
2. Read errors from the top. One real error often causes ten follow-on errors.
3. Read the `help:` and `note:` lines, they usually contain the fix.
4. Use `rust-analyzer` in the editor for inline errors and type hints.
5. `dbg!(value);` for quick inspection. Remove it before committing.
6. `RUST_BACKTRACE=1 cargo run` to get a backtrace from a panic.
   In PowerShell: `$env:RUST_BACKTRACE=1; cargo run`.
7. `cargo clippy -- -D warnings` for a stricter pass.
8. `cargo expand` (needs install) shows macro output, useful when a derive or
   macro error makes no sense.
9. Reduce to a minimal example. Delete code until the error changes.
10. When a lifetime error is confusing, add explicit lifetimes and let the
    compiler tell you which bound fails.

## 20. Cheatsheet

Formatting:

```rust
println!("{} {:?} {:#?}", display, debug, pretty_debug);
println!("{name} {age}");          // inline captured variables
println!("{value:>8.2}");          // width, alignment, precision
println!("{:#x} {:b} {:o}", 255, 5, 8);
eprintln!("to stderr");
let s = format!("{a}-{b}");
```

Conversions:

```rust
let s: String = "x".to_string();
let s: String = "x".into();
let s: String = String::from("x");
let r: &str = &s;
let n: i32 = "42".parse()?;
let n = 42 as f64;              // lossy numeric cast, no error
let n: u8 = 300u16.try_into()?; // checked, returns Result
let bytes = s.as_bytes();
let s = String::from_utf8(bytes.to_vec())?;
```

Common methods:

```rust
opt.is_some() / is_none() / unwrap_or(x) / map(f) / and_then(f) / ok_or(e)
res.is_ok() / is_err() / unwrap_or(x) / map(f) / map_err(f) / ok()
v.len() / is_empty() / contains(&x) / sort() / sort_by_key(|x| x.0)
m.get(&k) / m.entry(k).or_insert(v) / m.remove(&k)
s.trim() / split(c) / split_whitespace() / starts_with(p) / replace(a, b)
```

Ownership quick answers:

- "value used after move" → borrow or clone
- "cannot borrow as mutable" → check `mut`, check live readers
- "does not live long enough" → return owned data, or keep the owner alive
- "temporary dropped" → bind the owned value first
- "trait bound not satisfied" → derive or implement the trait
- "expected `&str`, found `String`" → add `&`, or take `String`

## 21. Four week plan

Week 1, syntax and ownership. Read chapters 1 to 6 of the Rust Book. Run
`variables`, `ownership`, `borrowing`. Write your own versions from scratch
without looking. Exercises: reverse a string, count words, FizzBuzz, a
temperature converter, a guessing game with `rand`.

Week 2, data modeling. Chapters 7 to 10. Run `structs_enums`, `errors`,
`collections`. Exercises: a todo list in memory with `Vec` and `HashMap`, a
simple stack calculator, parse a CSV into structs, implement `Display` for your
types.

Week 3, abstraction. Chapters 11 to 13. Run `traits_generics`, `lifetimes`.
Exercises: a generic `Stack<T>`, a `Shape` trait with three implementations, a
function that returns `impl Iterator<Item = &str>`, a small library crate with
unit tests and one integration test.

Week 4, real programs. Chapters 14 to 19. Run `concurrency`. Exercises: a CLI
that reads a file and counts word frequency (use `clap`), a multi-threaded sum
over a `Vec<u64>` with `Arc<Mutex<_>>` and with channels, a small web server from
the Book's final project, then rebuild one tool you already use in another
language.

The rule that makes this work: type every example yourself, then break it on
purpose and read the error. Delete a `&`, remove a `mut`, move a value, and see
which error appears. You learn the compiler's model by provoking it.

## 22. What to read next

- The Rust Book, <https://doc.rust-lang.org/book/>. Read it in order, do the
  exercises. Chapters 4, 6, 8, 10, and 13 are the ones people reread.
- Rustlings, <https://github.com/rust-lang/rustlings>. Small exercises that
  compile and fail until fixed. Good complement to the Book.
- The standard library docs, <https://doc.rust-lang.org/std/>. Search is fast,
  examples are runnable.
- Rust by Example, <https://doc.rust-lang.org/rust-by-example/>.
- Hacker News and r/rust for library news. Do not read advanced async or unsafe
  material before the basics click.
- "Learn Rust With Entirely Too Many Linked Lists" for ownership and smart
  pointers, once you are comfortable with the basics.
- The `nomicon` (<https://doc.rust-lang.org/nomicon/>) only when you start
  writing `unsafe`.

Local docs, no internet needed:

```powershell
rustup doc              # opens the offline books and std docs
```
