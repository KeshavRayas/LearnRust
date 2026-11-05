// Traits and generics.
//     cargo run --example traits_generics

use std::fmt;

trait Animal {
    // Required method.
    fn name(&self) -> &str;

    // Default method: implementors get it for free and can override it.
    fn sound(&self) -> String {
        format!("{} makes a sound", self.name())
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn sound(&self) -> String {
        format!("{} says woof", self.name())
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
}

// Implementing a std trait. You can also impl traits for your own types only
// (orphan rule), which is why you see newtype wrappers.
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

// Generic function with a trait bound. Static dispatch: the compiler generates
// a specialized copy per concrete type (monomorphization).
fn announce<T: Animal>(a: &T) -> String {
    format!("{}: {}", a.name(), a.sound())
}

// impl Trait in argument position is sugar for the same thing.
fn announce2(a: &impl Animal) -> String {
    a.sound()
}

// Trait object: dynamic dispatch through a vtable, allows mixed types.
// Prefer generics unless you need a heterogeneous collection or smaller code.
fn announce_all(animals: &[Box<dyn Animal>]) {
    for a in animals {
        println!("{}", a.sound());
    }
}

fn biggest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in &list[1..] {
        if item > max {
            max = item;
        }
    }
    max
}

// Deriving common traits. These are the ones you'll type most.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Config {
    retries: u8,
    verbose: bool,
}

fn main() {
    let d = Dog {
        name: "Rex".into(), // String::from("Rex")
    };
    let c = Cat {
        name: "Milo".into(),
    };

    println!("{}", announce(&d));
    println!("{}", announce(&c)); // Cat uses the default sound()
    println!("{}", announce2(&c));
    println!("display: {d}");

    announce_all(&[
        Box::new(Dog { name: "A".into() }),
        Box::new(Cat { name: "B".into() }),
    ]);

    println!("biggest ints   = {}", biggest(&[3, 9, 2]));
    println!("biggest strs   = {}", biggest(&["a", "z", "m"]));

    let cfg = Config::default();
    let cfg2 = Config {
        verbose: true,
        ..cfg // struct update syntax, remaining fields from cfg
    };
    println!("{cfg:?} {cfg2:?} equal={}", cfg == cfg2);

    // From/Into conversions are defined in pairs: impl From<A> for B gives you
    // B::from(a) and a.into() for free.
    let name: String = "Ferris".into();
    let name2 = String::from("Ferris");
    println!("{name} {name2}");

    // Iterator: implement Iterator and you get map/filter/collect for free.
    let countdown = Countdown { n: 3 };
    println!("countdown: {:?}", countdown.collect::<Vec<_>>());
}

struct Countdown {
    n: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            Some(self.n + 1)
        }
    }
}
