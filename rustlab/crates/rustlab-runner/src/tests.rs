use super::*;
use rustlab_core::{CommandOutput, CommandSpec, ProcessRunner};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

fn shell(script: &str) -> (String, Vec<String>) {
    if cfg!(windows) {
        (
            "cmd".to_string(),
            vec!["/C".to_string(), script.to_string()],
        )
    } else {
        ("sh".to_string(), vec!["-c".to_string(), script.to_string()])
    }
}

fn spec(script: &str) -> CommandSpec {
    let (program, args) = shell(script);
    CommandSpec {
        program,
        args,
        cwd: std::env::temp_dir(),
        stdin: None,
        env: BTreeMap::new(),
        timeout_secs: 5,
        output_cap_bytes: 1024,
    }
}

#[test]
fn runs_a_command_and_captures_stdout() {
    let output = LocalRunner::new().run(&spec("echo hello")).unwrap();
    assert_eq!(output.status, Some(0));
    assert!(output.stdout.contains("hello"));
    assert!(!output.timed_out);
}

#[test]
fn reports_a_nonzero_exit_code() {
    let output = LocalRunner::new().run(&spec("exit 3")).unwrap();
    assert_eq!(output.status, Some(3));
}

#[test]
fn kills_a_timed_out_process() {
    let mut command = if cfg!(windows) {
        spec("ping -n 5 127.0.0.1 > nul")
    } else {
        spec("sleep 5")
    };
    command.timeout_secs = 1;
    let output = LocalRunner::new().run(&command).unwrap();
    assert!(output.timed_out);
}

#[test]
fn does_not_wait_for_a_grandchild_that_holds_the_pipe() {
    let command = if cfg!(windows) {
        spec("start /B ping -n 30 127.0.0.1")
    } else {
        spec("sleep 30 &")
    };
    let started = Instant::now();
    let _output = LocalRunner::new().run(&command).unwrap();
    assert!(started.elapsed() < Duration::from_secs(10));
}

#[test]
fn stdin_write_does_not_block_the_timeout() {
    let mut command = if cfg!(windows) {
        spec("ping -n 5 127.0.0.1 > nul")
    } else {
        spec("sleep 5")
    };
    command.timeout_secs = 1;
    command.stdin = Some("x".repeat(200_000));
    let started = Instant::now();
    let output = LocalRunner::new().run(&command).unwrap();
    assert!(output.timed_out);
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn enforces_the_output_cap() {
    let mut command = if cfg!(windows) {
        spec("for /L %i in (1,1,100) do @echo x")
    } else {
        spec("yes x | head -n 100")
    };
    command.output_cap_bytes = 10;
    let error = LocalRunner::new().run(&command).unwrap_err();
    assert!(error.to_string().contains("output exceeded"));
}

#[test]
fn passes_explicit_environment_variables() {
    let mut command = if cfg!(windows) {
        spec("echo %RUSTLAB_TEST%")
    } else {
        spec("printf \"$RUSTLAB_TEST\"")
    };
    command
        .env
        .insert("RUSTLAB_TEST".to_string(), "yes".to_string());
    let output = LocalRunner::new().run(&command).unwrap();
    assert!(output.stdout.contains("yes"));
}

#[test]
fn scrubs_parent_rustlab_variables() {
    unsafe {
        std::env::set_var("RUSTLAB_SECRET", "secret");
    }
    let command = if cfg!(windows) {
        spec("echo [%RUSTLAB_SECRET%]")
    } else {
        spec("printf \"[$RUSTLAB_SECRET]\"")
    };
    let output = LocalRunner::new().run(&command).unwrap();
    assert!(!output.stdout.contains("secret"));
    unsafe {
        std::env::remove_var("RUSTLAB_SECRET");
    }
}

#[test]
fn mock_runner_returns_queued_outputs_in_order() {
    let runner = MockRunner::new(vec![CommandOutput {
        status: Some(0),
        stdout: "first".to_string(),
        stderr: String::new(),
        timed_out: false,
        duration_ms: 1,
    }]);
    let output = runner.run(&spec("echo first")).unwrap();
    assert_eq!(output.stdout, "first");
    assert_eq!(runner.commands().len(), 1);
    assert!(runner.run(&spec("echo second")).is_err());
}
