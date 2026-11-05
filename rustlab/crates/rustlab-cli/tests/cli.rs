use std::path::Path;
use std::process::Command;

#[test]
fn help_lists_the_solution_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_rustlab"))
        .arg("--help")
        .output()
        .expect("run rustlab --help");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("solution"));
}

#[test]
fn doctor_json_reports_ok() {
    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content");
    let home = tempfile::tempdir().expect("temp home");
    let output = Command::new(env!("CARGO_BIN_EXE_rustlab"))
        .env("RUSTLAB_CONTENT", content)
        .env("RUSTLAB_HOME", home.path())
        .args(["doctor", "--json"])
        .output()
        .expect("run rustlab doctor");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("\"ok\": true"));
}
