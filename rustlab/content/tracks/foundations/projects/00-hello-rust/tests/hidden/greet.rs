use hello_rust::greet;

#[test]
fn greets_a_name() {
    assert_eq!(greet("Rust"), "Hello, Rust!");
}

#[test]
fn greets_a_stranger() {
    assert_eq!(greet(""), "Hello, stranger!");
}
