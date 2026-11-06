pub fn greet(name: &str) -> String {
    if name.is_empty() {
        "Hello, stranger!".to_string()
    } else {
        format!("Hello, {name}!")
    }
}

pub fn count_to(n: u32) -> Vec<u32> {
    let _ = n;
    Vec::new()
}

pub fn sum(numbers: &[u32]) -> u32 {
    let _ = numbers;
    0
}
