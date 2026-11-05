use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use rustlab_core::{Diagnostic, DiagnosticLevel};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Explainer {
    pub code: String,
    pub title: String,
    pub body: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
struct ExplainerFile {
    schema_version: u32,
    errors: Vec<Explainer>,
}

#[derive(Debug, Clone, Default)]
pub struct ExplainerIndex {
    errors: Vec<Explainer>,
}

impl ExplainerIndex {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let file: ExplainerFile =
            serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
        if file.schema_version != 1 {
            anyhow::bail!(
                "unsupported explainer schema version: {}",
                file.schema_version
            );
        }
        Ok(Self {
            errors: file.errors,
        })
    }

    pub fn explain(&self, code: &str) -> Option<&Explainer> {
        self.errors.iter().find(|entry| entry.code == code)
    }

    pub fn explain_first(&self, diagnostics: &[Diagnostic]) -> Option<&Explainer> {
        for diagnostic in diagnostics {
            if matches!(diagnostic.level, DiagnosticLevel::Error)
                && let Some(code) = &diagnostic.code
                && let Some(explainer) = self.explain(code)
            {
                return Some(explainer);
            }
            if let Some(explainer) = self.explain_first(&diagnostic.children) {
                return Some(explainer);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests;
