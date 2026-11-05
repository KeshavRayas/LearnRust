use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rustlab_core::{
    CheckKind, CheckSpec, ContentProvider, Matcher, Milestone, Project, Track, milestone_key,
};
use serde::Deserialize;

#[derive(Debug)]
pub struct ContentError {
    pub path: PathBuf,
    pub message: String,
}

impl ContentError {
    fn new(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.message)
    }
}

impl std::error::Error for ContentError {}

#[derive(Debug, Deserialize)]
struct TrackFile {
    id: String,
    title: String,
    projects: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ProjectFile {
    id: String,
    title: String,
    pitch: String,
    hours: u8,
    concepts: Vec<String>,
    starter: PathBuf,
    #[serde(rename = "milestone")]
    milestones: Vec<MilestoneFile>,
}

#[derive(Debug, Deserialize)]
struct MilestoneFile {
    id: String,
    title: String,
    lesson: PathBuf,
    task: PathBuf,
    hints: Vec<PathBuf>,
    solution: PathBuf,
    checks: Vec<CheckSpec>,
}

#[derive(Debug, Clone)]
pub struct ContentIndex {
    root: PathBuf,
    tracks: Vec<Track>,
    projects: BTreeMap<String, Project>,
    project_dirs: BTreeMap<String, PathBuf>,
}

impl ContentIndex {
    pub fn load(root: &Path) -> Result<Self> {
        let root = root
            .canonicalize()
            .with_context(|| format!("content root not found: {}", root.display()))?;
        let tracks_dir = root.join("tracks");
        if !tracks_dir.is_dir() {
            anyhow::bail!(
                "content root has no tracks directory: {}",
                tracks_dir.display()
            );
        }

        let mut track_dirs: Vec<PathBuf> = fs::read_dir(&tracks_dir)
            .with_context(|| format!("read {}", tracks_dir.display()))?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.is_dir())
            .collect();
        track_dirs.sort();

        let mut tracks = Vec::new();
        let mut projects = BTreeMap::new();
        let mut project_dirs = BTreeMap::new();
        let mut track_ids = BTreeSet::new();

        for track_dir in track_dirs {
            let track_path = track_dir.join("track.toml");
            let raw = fs::read_to_string(&track_path)
                .with_context(|| format!("read {}", track_path.display()))?;
            let track_file: TrackFile =
                toml::from_str(&raw).with_context(|| format!("parse {}", track_path.display()))?;

            if track_file.id.trim().is_empty() {
                return Err(ContentError::new(&track_path, "track id is empty").into());
            }
            if !track_ids.insert(track_file.id.clone()) {
                return Err(ContentError::new(
                    &track_path,
                    format!("duplicate track id: {}", track_file.id),
                )
                .into());
            }

            let mut project_ids = Vec::new();
            for project_dir_name in &track_file.projects {
                if project_dir_name.trim().is_empty()
                    || Path::new(project_dir_name).is_absolute()
                    || project_dir_name.split(['/', '\\']).any(|part| part == "..")
                {
                    return Err(ContentError::new(
                        &track_path,
                        format!("invalid project directory: {project_dir_name}"),
                    )
                    .into());
                }
                let project_dir = track_dir.join("projects").join(project_dir_name);
                let project_path = project_dir.join("project.toml");
                let raw = fs::read_to_string(&project_path)
                    .with_context(|| format!("read {}", project_path.display()))?;
                let file: ProjectFile = toml::from_str(&raw)
                    .with_context(|| format!("parse {}", project_path.display()))?;

                validate_project(&project_dir, &project_path, &file)?;

                if projects.contains_key(&file.id) {
                    return Err(ContentError::new(
                        &project_path,
                        format!("duplicate project id: {}", file.id),
                    )
                    .into());
                }

                let project = build_project(&project_dir, &file)?;
                project_ids.push(project.id.clone());
                project_dirs.insert(project.id.clone(), project_dir.clone());
                projects.insert(project.id.clone(), project);
            }

            tracks.push(Track {
                id: track_file.id,
                title: track_file.title,
                projects: project_ids,
            });
        }

        Ok(Self {
            root,
            tracks,
            projects,
            project_dirs,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl ContentProvider for ContentIndex {
    fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    fn project(&self, id: &str) -> Option<&Project> {
        self.projects.get(id)
    }

    fn project_dir(&self, id: &str) -> Option<&Path> {
        self.project_dirs.get(id).map(PathBuf::as_path)
    }

    fn milestone(&self, project: &str, milestone: &str) -> Option<&Milestone> {
        self.projects
            .get(project)?
            .milestones
            .iter()
            .find(|m| m.id == milestone)
    }

    fn next_milestone(&self, project: &str, completed: &BTreeSet<String>) -> Option<&Milestone> {
        self.projects
            .get(project)?
            .milestones
            .iter()
            .find(|m| !completed.contains(&milestone_key(project, &m.id)))
    }
}

fn build_project(project_dir: &Path, file: &ProjectFile) -> Result<Project> {
    let starter = project_dir.join(&file.starter);
    let milestones = file
        .milestones
        .iter()
        .map(|m| Milestone {
            id: m.id.clone(),
            title: m.title.clone(),
            lesson: project_dir.join(&m.lesson),
            task: project_dir.join(&m.task),
            hints: m.hints.iter().map(|p| project_dir.join(p)).collect(),
            solution: project_dir.join(&m.solution),
            checks: m.checks.clone(),
        })
        .collect();

    Ok(Project {
        id: file.id.clone(),
        title: file.title.clone(),
        pitch: file.pitch.clone(),
        hours: file.hours,
        concepts: file.concepts.clone(),
        starter,
        milestones,
    })
}

fn validate_project(project_dir: &Path, project_path: &Path, file: &ProjectFile) -> Result<()> {
    if file.id.trim().is_empty() {
        return Err(ContentError::new(project_path, "project id is empty").into());
    }
    if file.milestones.is_empty() {
        return Err(ContentError::new(project_path, "project has no milestones").into());
    }

    let canonical_project = project_dir
        .canonicalize()
        .with_context(|| format!("project dir not found: {}", project_dir.display()))?;

    let starter = project_dir.join(&file.starter);
    if !starter.is_dir() {
        return Err(ContentError::new(project_path, "starter directory is missing").into());
    }
    if !starter.join("Cargo.toml").is_file() {
        return Err(ContentError::new(project_path, "starter has no Cargo.toml").into());
    }
    ensure_inside(&canonical_project, &starter, project_path)?;

    let package_names = package_names(&starter)?;

    let mut milestone_ids = BTreeSet::new();
    let mut test_names = BTreeSet::new();
    for milestone in &file.milestones {
        if milestone.id.trim().is_empty() {
            return Err(ContentError::new(project_path, "milestone id is empty").into());
        }
        if !milestone_ids.insert(milestone.id.clone()) {
            return Err(ContentError::new(
                project_path,
                format!("duplicate milestone id: {}", milestone.id),
            )
            .into());
        }
        if milestone.checks.is_empty() {
            return Err(ContentError::new(
                project_path,
                format!("milestone {} has no checks", milestone.id),
            )
            .into());
        }

        let solution = project_dir.join(&milestone.solution);
        if !solution.is_dir() {
            return Err(ContentError::new(
                project_path,
                format!("solution must be a directory: {}", solution.display()),
            )
            .into());
        }

        for path in std::iter::once(&milestone.lesson)
            .chain(std::iter::once(&milestone.task))
            .chain(milestone.hints.iter())
            .chain(std::iter::once(&milestone.solution))
        {
            let absolute = project_dir.join(path);
            if !absolute.exists() {
                return Err(ContentError::new(
                    project_path,
                    format!("missing file: {}", absolute.display()),
                )
                .into());
            }
            ensure_inside(&canonical_project, &absolute, project_path)?;
        }

        let mut check_ids = BTreeSet::new();
        for check in &milestone.checks {
            if check.id.trim().is_empty() {
                return Err(ContentError::new(project_path, "check id is empty").into());
            }
            if !check_ids.insert(check.id.clone()) {
                return Err(ContentError::new(
                    project_path,
                    format!("duplicate check id: {}", check.id),
                )
                .into());
            }
            if let CheckKind::CargoTest {
                test: Some(test), ..
            } = &check.kind
            {
                if test.trim().is_empty() {
                    return Err(ContentError::new(
                        project_path,
                        format!("check {} has an empty test name", check.id),
                    )
                    .into());
                }
                if !is_safe_test_name(test) {
                    return Err(ContentError::new(
                        project_path,
                        format!("check {} has an unsafe test name: {test}", check.id),
                    )
                    .into());
                }
                if !test_names.insert(test.clone()) {
                    return Err(ContentError::new(
                        project_path,
                        format!("duplicate hidden test target: {test}"),
                    )
                    .into());
                }
            }
            validate_check(
                project_dir,
                project_path,
                &package_names,
                &milestone.id,
                check,
            )?;
        }
    }

    Ok(())
}

fn validate_check(
    project_dir: &Path,
    project_path: &Path,
    packages: &BTreeSet<String>,
    milestone_id: &str,
    check: &CheckSpec,
) -> Result<()> {
    match &check.kind {
        CheckKind::CargoTest { package, test } => {
            if !packages.contains(package) {
                return Err(ContentError::new(
                    project_path,
                    format!("check {} names unknown package {}", check.id, package),
                )
                .into());
            }
            if let Some(test) = test {
                let hidden = project_dir
                    .join("tests")
                    .join("hidden")
                    .join(format!("{test}.rs"));
                if !hidden.is_file() {
                    return Err(ContentError::new(
                        project_path,
                        format!(
                            "check {} expects hidden test {}",
                            check.id,
                            hidden.display()
                        ),
                    )
                    .into());
                }
            }
        }
        CheckKind::Cli {
            package, expect, ..
        } => {
            if !packages.contains(package) {
                return Err(ContentError::new(
                    project_path,
                    format!("check {} names unknown package {}", check.id, package),
                )
                .into());
            }
            validate_matcher(project_path, &check.id, &expect.stdout)?;
            validate_matcher(project_path, &check.id, &expect.stderr)?;
        }
        CheckKind::CompileFail {
            package,
            error_code,
        } => {
            if !packages.contains(package) {
                return Err(ContentError::new(
                    project_path,
                    format!("check {} names unknown package {}", check.id, package),
                )
                .into());
            }
            if error_code.trim().is_empty() {
                return Err(ContentError::new(
                    project_path,
                    format!("check {} has an empty error code", check.id),
                )
                .into());
            }
        }
        CheckKind::Clippy { package, .. } => {
            if !packages.contains(package) {
                return Err(ContentError::new(
                    project_path,
                    format!("check {} names unknown package {}", check.id, package),
                )
                .into());
            }
        }
    }

    let _ = milestone_id;
    Ok(())
}

fn validate_matcher(path: &Path, check_id: &str, matcher: &Matcher) -> Result<()> {
    if let Matcher::Regex { value } = matcher {
        regex::Regex::new(value).map_err(|error| {
            ContentError::new(path, format!("check {check_id} has invalid regex: {error}"))
        })?;
    }
    if let Matcher::Contains { value } = matcher
        && value.is_empty()
    {
        return Err(ContentError::new(
            path,
            format!("check {check_id} has an empty contains matcher"),
        )
        .into());
    }
    Ok(())
}

fn package_names(starter: &Path) -> Result<BTreeSet<String>> {
    let raw = fs::read_to_string(starter.join("Cargo.toml"))?;
    let value: toml::Value = toml::from_str(&raw)?;
    let mut names = BTreeSet::new();
    if let Some(name) = value
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(toml::Value::as_str)
    {
        names.insert(name.to_string());
    }
    Ok(names)
}

fn ensure_inside(root: &Path, path: &Path, error_path: &Path) -> Result<()> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("path not found: {}", path.display()))?;
    if !canonical.starts_with(root) {
        return Err(ContentError::new(
            error_path,
            format!("path escapes project directory: {}", path.display()),
        )
        .into());
    }
    Ok(())
}

fn is_safe_test_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

#[cfg(test)]
mod tests;
