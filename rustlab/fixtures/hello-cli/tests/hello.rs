#[test]
fn prints_hello() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hello-cli"))
        .output()
        .expect("run hello-cli");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
}
