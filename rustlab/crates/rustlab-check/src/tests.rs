use super::*;
use rustlab_core::{CheckKind, CheckSpec, CommandOutput, Expect};

fn spec(kind: CheckKind) -> CheckSpec {
    CheckSpec {
        id: "check".to_string(),
        kind,
        timeout_secs: 5,
    }
}

fn output(status: i32, stdout: &str, stderr: &str) -> CommandOutput {
    CommandOutput {
        status: Some(status),
        stdout: stdout.to_string(),
        stderr: stderr.to_string(),
        timed_out: false,
        duration_ms: 1,
    }
}

#[test]
fn matchers_cover_the_contract() {
    assert!(matches(&Matcher::Empty, "  "));
    assert!(matches(
        &Matcher::Exact {
            value: "hello".to_string()
        },
        " hello "
    ));
    assert!(matches(
        &Matcher::Contains {
            value: "ell".to_string()
        },
        "hello"
    ));
    assert!(matches(
        &Matcher::Regex {
            value: r"^h.*o$".to_string()
        },
        "hello"
    ));
    assert!(matches(
        &Matcher::Json {
            value: serde_json::json!({"ok": true})
        },
        r#"{"ok": true}"#
    ));
    assert!(!matches(
        &Matcher::Contains {
            value: "nope".to_string()
        },
        "hello"
    ));
}

#[test]
fn plans_a_cargo_test() {
    let ctx = CheckContext {
        project_root: PathBuf::from("."),
        package_dir: PathBuf::from("."),
        target_dir: PathBuf::from("target"),
        binary_path: None,
    };
    let commands = plan(
        &spec(CheckKind::CargoTest {
            package: "demo".to_string(),
            test: Some("check".to_string()),
        }),
        &ctx,
    )
    .unwrap();
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].program, "cargo");
    assert_eq!(
        commands[0].args,
        vec!["test", "--package", "demo", "--test", "check"]
    );
}

#[test]
fn evaluates_cargo_test_output() {
    let spec = spec(CheckKind::CargoTest {
        package: "demo".to_string(),
        test: None,
    });
    assert!(evaluate(&spec, &[output(0, "", "")]).passed);
    assert!(!evaluate(&spec, &[output(101, "boom", "")]).passed);
}

#[test]
fn evaluates_cli_output() {
    let spec = spec(CheckKind::Cli {
        package: "demo".to_string(),
        args: vec![],
        stdin: None,
        expect: Expect {
            exit_code: 0,
            stdout: Matcher::Contains {
                value: "6.21".to_string(),
            },
            stderr: Matcher::Empty,
        },
    });
    let build = output(0, "", "");
    assert!(evaluate(&spec, &[build.clone(), output(0, "6.21 mi", "")]).passed);
    assert!(!evaluate(&spec, &[build, output(1, "6.21 mi", "")]).passed);
}

#[test]
fn parses_compiler_diagnostics() {
    let line = r#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"E0382"},"message":"use of moved value","spans":[{"file_name":"src/main.rs","line_start":3,"column_start":5,"line_end":3,"column_end":9}],"children":[]}}"#;
    let diagnostics = parse_cargo_diagnostics(line, "");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].code.as_deref(), Some("E0382"));
    assert_eq!(diagnostics[0].spans[0].line_start, 3);
}
