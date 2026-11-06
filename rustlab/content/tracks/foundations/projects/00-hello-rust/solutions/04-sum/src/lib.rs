pub fn greet(name: &str) -> String {
    if name.is_empty() {
        "Hello, stranger!".to_string()
    } else {
        format!("Hello, {name}!")
    }
}

pub fn count_to(n: u32) -> Vec<u32> {
    let mut numbers = Vec::new();
    for i in 1..=n {
        numbers.push(i);
    }
    numbers
}

pub fn sum(numbers: &[u32]) -> u32 {
    let mut total = 0;
    for number in numbers {
        total += *number;
    }
    total
}
