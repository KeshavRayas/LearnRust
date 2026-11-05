use std::process::Command;

#[test]
fn cli_converts() {
    let output = Command::new(env!("CARGO_BIN_EXE_unit-converter"))
        .args(["10", "km", "mi"])
        .output()
        .expect("run unit-converter");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "6.21 mi");

    let output = Command::new(env!("CARGO_BIN_EXE_unit-converter"))
        .args(["10", "furlongs", "mi"])
        .output()
        .expect("run unit-converter");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown unit: furlongs"));
}
