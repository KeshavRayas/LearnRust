use super::*;
use std::path::PathBuf;

#[test]
fn check_report_round_trips() {
    let report = CheckReport {
        project: "demo".to_string(),
        milestone: "one".to_string(),
        passed: false,
        results: vec![CheckResult {
            check_id: "test".to_string(),
            passed: false,
            message: "failed".to_string(),
            detail: Some("boom".to_string()),
            duration_ms: 12,
        }],
        diagnostics: vec![Diagnostic {
            level: DiagnosticLevel::Error,
            code: Some("E0382".to_string()),
            message: "use of moved value".to_string(),
            spans: vec![Span {
                file: PathBuf::from("src/main.rs"),
                line_start: 1,
                column_start: 1,
                line_end: 1,
                column_end: 4,
            }],
            children: vec![],
            suggestions: vec![],
        }],
        duration_ms: 34,
    };
    let raw = serde_json::to_string(&report).unwrap();
    let loaded: CheckReport = serde_json::from_str(&raw).unwrap();
    assert_eq!(loaded.project, "demo");
    assert!(!loaded.passed);
    assert_eq!(loaded.diagnostics[0].code.as_deref(), Some("E0382"));
}

#[test]
fn progress_starts_empty() {
    let progress = Progress::new();
    assert_eq!(progress.schema_version, 1);
    assert!(progress.completed.is_empty());
    assert!(progress.current.is_none());
}

#[test]
fn milestone_keys_are_stable() {
    assert_eq!(milestone_key("demo", "one"), "demo/one");
}
