use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use rustlab_core::{
    CheckContext, CheckKind, CheckResult, CheckSpec, CommandOutput, CommandSpec, Diagnostic,
    DiagnosticLevel, Matcher, Span,
};
use serde_json::Value;

const OUTPUT_CAP: usize = 64 * 1024;

pub fn plan(spec: &CheckSpec, ctx: &CheckContext) -> Result<Vec<CommandSpec>> {
    match &spec.kind {
        CheckKind::CargoTest { package, test } => {
            let mut args = vec!["test".to_string(), "--package".to_string(), package.clone()];
            if let Some(test) = test {
                args.push("--test".to_string());
                args.push(test.clone());
            }
            Ok(vec![cargo_command(
                "cargo",
                args,
                ctx.package_dir.clone(),
                spec.timeout_secs,
                ctx,
            )])
        }
        CheckKind::Cli {
            package,
            args,
            stdin,
            ..
        } => {
            let build = cargo_command(
                "cargo",
                vec![
                    "build".to_string(),
                    "--package".to_string(),
                    package.clone(),
                ],
                ctx.package_dir.clone(),
                spec.timeout_secs,
                ctx,
            );
            let binary = ctx.binary_path.clone().unwrap_or_else(|| {
                let name = if cfg!(windows) {
                    format!("{package}.exe")
                } else {
                    package.clone()
                };
                ctx.target_dir.join("debug").join(name)
            });
            let run = CommandSpec {
                program: binary.to_string_lossy().into_owned(),
                args: args.clone(),
                cwd: ctx.project_root.clone(),
                stdin: stdin.clone(),
                env: BTreeMap::new(),
                timeout_secs: spec.timeout_secs,
                output_cap_bytes: OUTPUT_CAP,
            };
            Ok(vec![build, run])
        }
        CheckKind::CompileFail { package, .. } => Ok(vec![cargo_command(
            "cargo",
            vec![
                "check".to_string(),
                "--package".to_string(),
                package.clone(),
                "--message-format=json".to_string(),
            ],
            ctx.package_dir.clone(),
            spec.timeout_secs,
            ctx,
        )]),
        CheckKind::Clippy { package, deny } => {
            let mut args = vec![
                "clippy".to_string(),
                "--package".to_string(),
                package.clone(),
                "--".to_string(),
            ];
            if deny.is_empty() {
                args.push("-D".to_string());
                args.push("warnings".to_string());
            } else {
                for lint in deny {
                    args.push("-D".to_string());
                    args.push(lint.clone());
                }
            }
            Ok(vec![cargo_command(
                "cargo",
                args,
                ctx.package_dir.clone(),
                spec.timeout_secs,
                ctx,
            )])
        }
    }
}

fn cargo_command(
    program: &str,
    args: Vec<String>,
    cwd: PathBuf,
    timeout_secs: u64,
    ctx: &CheckContext,
) -> CommandSpec {
    let mut command = command(program, args, cwd, timeout_secs);
    command.env.insert(
        "CARGO_TARGET_DIR".to_string(),
        ctx.target_dir.to_string_lossy().into_owned(),
    );
    command
}

fn command(program: &str, args: Vec<String>, cwd: PathBuf, timeout_secs: u64) -> CommandSpec {
    CommandSpec {
        program: program.to_string(),
        args,
        cwd,
        stdin: None,
        env: BTreeMap::new(),
        timeout_secs,
        output_cap_bytes: OUTPUT_CAP,
    }
}

pub fn evaluate(spec: &CheckSpec, outputs: &[CommandOutput]) -> CheckResult {
    let started = Instant::now();
    let (passed, message, detail) = match &spec.kind {
        CheckKind::CargoTest { .. } => {
            let Some(output) = outputs.first() else {
                return result(spec, false, "no command output", None, started);
            };
            if output.timed_out {
                (false, "test timed out".to_string(), Some(detail(output)))
            } else if output.status == Some(0) {
                (true, "tests passed".to_string(), None)
            } else {
                (
                    false,
                    format!("tests failed with exit code {}", status(output)),
                    Some(detail(output)),
                )
            }
        }
        CheckKind::Cli { expect, .. } => {
            let Some(build) = outputs.first() else {
                return result(spec, false, "no build output", None, started);
            };
            if build.timed_out {
                return result(spec, false, "build timed out", Some(detail(build)), started);
            }
            if build.status != Some(0) {
                return result(
                    spec,
                    false,
                    format!("build failed with exit code {}", status(build)),
                    Some(detail(build)),
                    started,
                );
            }
            let Some(run) = outputs.last() else {
                return result(spec, false, "no run output", None, started);
            };
            if run.timed_out {
                return result(spec, false, "program timed out", Some(detail(run)), started);
            }
            if run.status != Some(expect.exit_code) {
                return result(
                    spec,
                    false,
                    format!(
                        "expected exit code {}, got {}",
                        expect.exit_code,
                        status(run)
                    ),
                    Some(detail(run)),
                    started,
                );
            }
            if !matches(&expect.stdout, &run.stdout) {
                return result(
                    spec,
                    false,
                    "stdout did not match".to_string(),
                    Some(detail(run)),
                    started,
                );
            }
            if !matches(&expect.stderr, &run.stderr) {
                return result(
                    spec,
                    false,
                    "stderr did not match".to_string(),
                    Some(detail(run)),
                    started,
                );
            }
            (true, "program output matched".to_string(), None)
        }
        CheckKind::CompileFail { error_code, .. } => {
            if outputs.iter().any(|output| output.timed_out) {
                return result(spec, false, "compiler timed out", None, started);
            }
            let mut diagnostics = Vec::new();
            for output in outputs {
                diagnostics.extend(parse_cargo_diagnostics(&output.stdout, &output.stderr));
            }
            let found = diagnostics.iter().any(|diagnostic| {
                diagnostic.code.as_deref() == Some(error_code.as_str())
                    && matches!(diagnostic.level, DiagnosticLevel::Error)
            });
            if found {
                (true, format!("saw expected error {error_code}"), None)
            } else {
                (
                    false,
                    format!("expected compiler error {error_code}"),
                    Some("the package compiled or produced a different error".to_string()),
                )
            }
        }
        CheckKind::Clippy { .. } => {
            let Some(output) = outputs.first() else {
                return result(spec, false, "no command output", None, started);
            };
            if output.timed_out {
                (false, "clippy timed out".to_string(), Some(detail(output)))
            } else if output.status == Some(0) {
                (true, "clippy passed".to_string(), None)
            } else {
                (
                    false,
                    format!("clippy failed with exit code {}", status(output)),
                    Some(detail(output)),
                )
            }
        }
    };

    result(spec, passed, message, detail, started)
}

fn result(
    spec: &CheckSpec,
    passed: bool,
    message: impl Into<String>,
    detail: Option<String>,
    started: Instant,
) -> CheckResult {
    CheckResult {
        check_id: spec.id.clone(),
        passed,
        message: message.into(),
        detail: detail.map(|value| truncate(&value, 8 * 1024)),
        duration_ms: started.elapsed().as_millis(),
    }
}

fn status(output: &CommandOutput) -> String {
    output
        .status
        .map(|code| code.to_string())
        .unwrap_or_else(|| "terminated".to_string())
}

fn detail(output: &CommandOutput) -> String {
    let mut text = String::new();
    if !output.stdout.trim().is_empty() {
        text.push_str(&output.stdout);
    }
    if !output.stderr.trim().is_empty() {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&output.stderr);
    }
    text
}

fn truncate(value: &str, limit: usize) -> String {
    if value.len() <= limit {
        return value.to_string();
    }
    let mut end = limit;
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}\n... truncated", &value[..end])
}

pub fn matches(matcher: &Matcher, value: &str) -> bool {
    let value = value.trim();
    match matcher {
        Matcher::Empty => value.is_empty(),
        Matcher::Exact { value: expected } => value == expected.trim(),
        Matcher::Contains { value: needle } => value.contains(needle),
        Matcher::Regex { value: pattern } => regex::Regex::new(pattern)
            .map(|regex| regex.is_match(value))
            .unwrap_or(false),
        Matcher::Json { value: expected } => serde_json::from_str::<Value>(value)
            .map(|actual| actual == *expected)
            .unwrap_or(false),
    }
}

pub fn parse_cargo_diagnostics(stdout: &str, stderr: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for line in stdout.lines().chain(stderr.lines()) {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if value.get("reason").and_then(Value::as_str) != Some("compiler-message") {
            continue;
        }
        let Some(message) = value.get("message") else {
            continue;
        };
        let diagnostic = parse_diagnostic(message);
        let key = format!(
            "{:?}|{}|{}",
            diagnostic.code,
            diagnostic.message,
            diagnostic
                .spans
                .first()
                .map(|span| format!("{}:{}", span.file.display(), span.line_start))
                .unwrap_or_default()
        );
        if seen.insert(key) {
            diagnostics.push(diagnostic);
        }
    }

    diagnostics.sort_by_key(|diagnostic| match diagnostic.level {
        DiagnosticLevel::Error => 0,
        DiagnosticLevel::Warning => 1,
        DiagnosticLevel::Note => 2,
        DiagnosticLevel::Help => 3,
    });
    diagnostics.truncate(20);
    diagnostics
}

fn parse_diagnostic(value: &Value) -> Diagnostic {
    let level = match value.get("level").and_then(Value::as_str) {
        Some("error") => DiagnosticLevel::Error,
        Some("warning") => DiagnosticLevel::Warning,
        Some("help") => DiagnosticLevel::Help,
        _ => DiagnosticLevel::Note,
    };
    let code = value
        .get("code")
        .and_then(|code| code.get("code"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let message = value
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("compiler message")
        .to_string();

    let spans = value
        .get("spans")
        .and_then(Value::as_array)
        .map(|spans| {
            spans
                .iter()
                .map(|span| Span {
                    file: PathBuf::from(
                        span.get("file_name")
                            .and_then(Value::as_str)
                            .unwrap_or("<unknown>"),
                    ),
                    line_start: span.get("line_start").and_then(Value::as_u64).unwrap_or(0)
                        as usize,
                    column_start: span
                        .get("column_start")
                        .and_then(Value::as_u64)
                        .unwrap_or(0) as usize,
                    line_end: span.get("line_end").and_then(Value::as_u64).unwrap_or(0) as usize,
                    column_end: span.get("column_end").and_then(Value::as_u64).unwrap_or(0)
                        as usize,
                })
                .collect()
        })
        .unwrap_or_default();

    let mut suggestions = Vec::new();
    let mut children = Vec::new();
    if let Some(items) = value.get("children").and_then(Value::as_array) {
        for child in items {
            let parsed = parse_diagnostic(child);
            for span in child
                .get("spans")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(replacement) = span.get("suggested_replacement").and_then(Value::as_str)
                {
                    suggestions.push(replacement.to_string());
                }
            }
            children.push(parsed);
        }
    }

    Diagnostic {
        level,
        code,
        message,
        spans,
        children,
        suggestions,
    }
}

#[cfg(test)]
mod tests;
