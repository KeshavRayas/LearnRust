use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub type TrackId = String;
pub type ProjectId = String;
pub type MilestoneId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub title: String,
    pub projects: Vec<ProjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub pitch: String,
    pub hours: u8,
    pub concepts: Vec<String>,
    pub starter: PathBuf,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: MilestoneId,
    pub title: String,
    pub lesson: PathBuf,
    pub task: PathBuf,
    pub hints: Vec<PathBuf>,
    pub solution: PathBuf,
    pub checks: Vec<CheckSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSpec {
    pub id: String,
    #[serde(flatten)]
    pub kind: CheckKind,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CheckKind {
    CargoTest {
        package: String,
        test: Option<String>,
    },
    Cli {
        package: String,
        args: Vec<String>,
        stdin: Option<String>,
        expect: Expect,
    },
    CompileFail {
        package: String,
        error_code: String,
    },
    Clippy {
        package: String,
        deny: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expect {
    #[serde(default)]
    pub exit_code: i32,
    #[serde(default)]
    pub stdout: Matcher,
    #[serde(default)]
    pub stderr: Matcher,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Matcher {
    #[default]
    Empty,
    Exact {
        value: String,
    },
    Contains {
        value: String,
    },
    Regex {
        value: String,
    },
    Json {
        value: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub stdin: Option<String>,
    pub env: BTreeMap<String, String>,
    pub timeout_secs: u64,
    pub output_cap_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOutput {
    pub status: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
    pub duration_ms: u128,
}

pub trait ProcessRunner {
    fn run(&self, cmd: &CommandSpec) -> anyhow::Result<CommandOutput>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckContext {
    pub project_root: PathBuf,
    pub package_dir: PathBuf,
    pub target_dir: PathBuf,
    pub binary_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub check_id: String,
    pub passed: bool,
    pub message: String,
    pub detail: Option<String>,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckReport {
    pub project: ProjectId,
    pub milestone: MilestoneId,
    pub passed: bool,
    pub results: Vec<CheckResult>,
    pub diagnostics: Vec<Diagnostic>,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: Option<String>,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Diagnostic>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub file: PathBuf,
    pub line_start: usize,
    pub column_start: usize,
    pub line_end: usize,
    pub column_end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progress {
    pub schema_version: u32,
    pub learner_id: uuid::Uuid,
    pub completed: BTreeSet<String>,
    pub current: Option<String>,
    pub attempts: Vec<Attempt>,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            schema_version: 1,
            learner_id: uuid::Uuid::new_v4(),
            completed: BTreeSet::new(),
            current: None,
            attempts: Vec::new(),
        }
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attempt {
    pub project: String,
    pub milestone: String,
    pub at: String,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMarker {
    pub schema_version: u32,
    pub project_id: String,
    pub milestone_id: String,
    pub content_version: String,
}

pub trait ContentProvider {
    fn tracks(&self) -> &[Track];
    fn project(&self, id: &str) -> Option<&Project>;
    fn project_dir(&self, id: &str) -> Option<&Path>;
    fn milestone(&self, project: &str, milestone: &str) -> Option<&Milestone>;
    fn next_milestone(&self, project: &str, completed: &BTreeSet<String>) -> Option<&Milestone>;
}

pub fn milestone_key(project: &str, milestone: &str) -> String {
    format!("{project}/{milestone}")
}

#[cfg(test)]
mod tests;
