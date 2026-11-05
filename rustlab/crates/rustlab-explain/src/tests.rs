use super::*;
use std::collections::BTreeSet;
use std::path::Path;

use rustlab_core::{Diagnostic, DiagnosticLevel};

fn index() -> ExplainerIndex {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("content")
        .join("errors.json");
    ExplainerIndex::load(&path).expect("load errors.json")
}

#[test]
fn loads_all_25_codes_without_duplicates() {
    let index = index();
    assert_eq!(index.errors.len(), 25);
    let mut codes = BTreeSet::new();
    for explainer in &index.errors {
        assert!(codes.insert(explainer.code.clone()));
        assert!(
            explainer.body.len() >= 120,
            "{} body too short",
            explainer.code
        );
        assert!(
            explainer.body.len() <= 350,
            "{} body too long",
            explainer.code
        );
        assert!(
            explainer
                .link
                .starts_with("https://doc.rust-lang.org/error_codes/"),
            "{} has a bad link",
            explainer.code
        );
        assert!(
            explainer
                .link
                .ends_with(&format!("{}.html", explainer.code))
        );
    }
}

#[test]
fn finds_a_known_code_and_skips_unknowns() {
    let index = index();
    assert!(index.explain("E0382").is_some());
    assert!(index.explain("NOPE").is_none());
}

#[test]
fn explain_first_skips_warnings_and_finds_children() {
    let index = index();
    let warning = Diagnostic {
        level: DiagnosticLevel::Warning,
        code: Some("E0382".to_string()),
        message: "warning".to_string(),
        spans: vec![],
        children: vec![],
        suggestions: vec![],
    };
    let error = Diagnostic {
        level: DiagnosticLevel::Error,
        code: Some("E0502".to_string()),
        message: "error".to_string(),
        spans: vec![],
        children: vec![],
        suggestions: vec![],
    };
    assert_eq!(
        index
            .explain_first(&[warning, error])
            .map(|entry| entry.code.as_str()),
        Some("E0502")
    );

    let parent = Diagnostic {
        level: DiagnosticLevel::Error,
        code: None,
        message: "parent".to_string(),
        spans: vec![],
        children: vec![Diagnostic {
            level: DiagnosticLevel::Error,
            code: Some("E0382".to_string()),
            message: "child".to_string(),
            spans: vec![],
            children: vec![],
            suggestions: vec![],
        }],
        suggestions: vec![],
    };
    assert_eq!(
        index
            .explain_first(&[parent])
            .map(|entry| entry.code.as_str()),
        Some("E0382")
    );
}
