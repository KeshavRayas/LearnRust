// Structs, enums, methods, and pattern matching.
//     cargo run --example structs_enums

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated function (no self). Used like Point::new(1, 2).
    fn new(x: i32, y: i32) -> Self {
        Self { x, y } // field init shorthand
    }

    fn origin() -> Self {
        Self::new(0, 0)
    }

    // Method: takes &self, can read but not mutate.
    fn dist_sq(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    // Mutating method: needs &mut self and a mut binding.
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    // Consuming method: takes self, moves the value.
    fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// Enums can carry data, and each variant can hold different data.
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
    Empty,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            // Matching a reference binds references to the fields.
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rect { w, h } => w * h,
            Shape::Empty => 0.0,
        }
    }
}

fn main() {
    let mut p = Point::new(3, 4);
    p.translate(1, -1);
    println!("{p:?} dist_sq={}", p.dist_sq());
    println!("origin: {:?}", Point::origin());
    println!("into_tuple: {:?}", p.clone().into_tuple());
    println!("p still usable after clone: {p:?}");

    // Tuple struct and unit struct.
    struct Meters(f64);
    struct Marker;
    let d = Meters(1.5);
    let _m = Marker;
    println!("d = {} m", d.0);

    // Match is exhaustive: every case must be handled, or you get E0004.
    for s in [
        Shape::Circle { radius: 1.0 },
        Shape::Rect { w: 2.0, h: 2.0 },
        Shape::Empty,
    ] {
        match &s {
            // Guards add conditions. `|` combines patterns.
            Shape::Rect { w, h } if w == h => println!("square {w}x{h} area={}", s.area()),
            Shape::Circle { radius } if *radius == 0.0 => println!("degenerate circle"),
            other => println!("{other:?} area={}", other.area()),
        }
    }

    // if let: match one pattern, ignore the rest.
    let maybe = Some(7);
    if let Some(v) = maybe {
        println!("if let got {v}");
    }

    // Option is an enum: Some(T) or None. No null in safe Rust.
    println!("unwrap_or: {}", half(3).unwrap_or(0)); // None -> 0
    println!("map: {:?}", maybe.map(|v| v * 2));
    println!(
        "? on Option in a fn returning Option: {:?}",
        maybe.and_then(half)
    );

    // while let: loop until the pattern stops matching.
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }
}

fn half(x: i32) -> Option<i32> {
    if x % 2 == 0 { Some(x / 2) } else { None }
}
