mod progress;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use progress::ProgressStore;
use rustlab_check::{evaluate, parse_cargo_diagnostics, plan};
use rustlab_content::ContentIndex;
use rustlab_core::{
    CheckContext, CheckKind, CheckReport, CheckResult, CommandSpec, ContentProvider, Diagnostic,
    Milestone, ProcessRunner, Progress, Project, ProjectMarker, milestone_key,
};
use rustlab_explain::ExplainerIndex;
use rustlab_runner::LocalRunner;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "rustlab",
    version,
    about = "Learn Rust by building real projects"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Doctor {
        #[arg(long)]
        json: bool,
    },
    List {
        #[arg(long)]
        json: bool,
    },
    Start {
        project: Option<String>,
        #[arg(long)]
        dir: Option<PathBuf>,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        json: bool,
    },
    Check {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        milestone: Option<String>,
    },
    Hint {
        #[arg(long, default_value_t = 1)]
        level: usize,
    },
    Solution {
        #[arg(long)]
        write: bool,
    },
    Next {
        #[arg(long)]
        json: bool,
    },
    Reset {
        #[arg(long)]
        yes: bool,
    },
    Export {
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

struct Deps {
    content: Box<dyn ContentProvider>,
    runner: Box<dyn ProcessRunner>,
    progress: ProgressStore,
    content_root: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        let code = if error.chain().any(|cause| {
            cause
                .downcast_ref::<rustlab_content::ContentError>()
                .is_some()
        }) {
            4
        } else {
            1
        };
        std::process::exit(code);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Export { output } => export(output),
        Commands::Doctor { json } => doctor(json),
        Commands::List { json } => list(json),
        Commands::Start {
            project,
            dir,
            force,
            json,
        } => start(project, dir, force, json),
        Commands::Check {
            json,
            project,
            milestone,
        } => check(json, project, milestone),
        Commands::Hint { level } => hint(level),
        Commands::Solution { write } => solution(write),
        Commands::Next { json } => next(json),
        Commands::Reset { yes } => reset(yes),
    }
}

fn build_deps() -> Result<Deps> {
    let content_root = discover_content_root()?;
    let content = ContentIndex::load(&content_root)?;
    Ok(Deps {
        content: Box::new(content),
        runner: Box::new(LocalRunner::new()),
        progress: ProgressStore::from_env(),
        content_root,
    })
}

fn discover_content_root() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("RUSTLAB_CONTENT") {
        let path = PathBuf::from(path);
        if path.join("tracks").is_dir() {
            return Ok(path);
        }
        anyhow::bail!(
            "RUSTLAB_CONTENT does not contain a tracks directory: {}",
            path.display()
        );
    }

    let home_content = ProgressStore::from_env().home().join("content");
    if home_content.join("tracks").is_dir() {
        return Ok(home_content);
    }

    let cwd = std::env::current_dir()?;
    let exe = std::env::current_exe()?;
    let exe_dir = exe.parent().unwrap_or(Path::new("."));
    let candidates = [
        cwd.join("content"),
        exe_dir.join("content"),
        exe_dir.join("..").join("content"),
    ];
    for candidate in candidates {
        if candidate.join("tracks").is_dir() {
            return Ok(candidate);
        }
    }
    anyhow::bail!("content directory not found; set RUSTLAB_CONTENT")
}

fn doctor(json_output: bool) -> Result<()> {
    let mut checks: Vec<(String, bool, String)> = Vec::new();

    checks.push(check_command("rustc", "rustc", &["--version"]));
    checks.push(check_command("cargo", "cargo", &["--version"]));
    checks.push(check_hello_world());

    match discover_content_root().and_then(|root| ContentIndex::load(&root).map(|_| root)) {
        Ok(root) => checks.push((
            "content".to_string(),
            true,
            format!("loaded {}", root.display()),
        )),
        Err(error) => checks.push(("content".to_string(), false, format!("{error:#}"))),
    }

    let progress = ProgressStore::from_env();
    match progress.ensure_writable() {
        Ok(()) => checks.push((
            "home".to_string(),
            true,
            progress.home().display().to_string(),
        )),
        Err(error) => checks.push(("home".to_string(), false, format!("{error:#}"))),
    }

    let ok = checks.iter().all(|(_, passed, _)| *passed);
    if json_output {
        let items: Vec<_> = checks
            .iter()
            .map(|(name, passed, detail)| json!({"name": name, "ok": passed, "detail": detail}))
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({"ok": ok, "checks": items}))?
        );
    } else {
        for (name, passed, detail) in &checks {
            println!("{} {name:<8} {detail}", if *passed { "✓" } else { "✗" });
        }
        if ok {
            println!("\nEverything looks ready.");
        } else {
            println!("\nFix the failed checks before starting a project.");
        }
    }
    if !ok {
        std::process::exit(3);
    }
    Ok(())
}

fn check_command(name: &str, program: &str, args: &[&str]) -> (String, bool, String) {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => (
            name.to_string(),
            true,
            String::from_utf8_lossy(&output.stdout).trim().to_string(),
        ),
        Ok(output) => (
            name.to_string(),
            false,
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ),
        Err(error) => (name.to_string(), false, error.to_string()),
    }
}

fn check_hello_world() -> (String, bool, String) {
    let result = (|| -> Result<String> {
        let dir = tempfile::tempdir()?;
        let source = dir.path().join("main.rs");
        fs::write(&source, "fn main() { println!(\"hello\"); }")?;
        let exe = dir
            .path()
            .join(if cfg!(windows) { "hello.exe" } else { "hello" });
        let compile = Command::new("rustc")
            .arg("main.rs")
            .arg("-o")
            .arg(&exe)
            .current_dir(dir.path())
            .output()?;
        if !compile.status.success() {
            anyhow::bail!("{}", String::from_utf8_lossy(&compile.stderr).trim());
        }
        let run = Command::new(&exe).output()?;
        if !run.status.success() {
            anyhow::bail!("hello world compiled but did not run");
        }
        Ok("compiled and ran a hello world".to_string())
    })();
    match result {
        Ok(detail) => ("linker".to_string(), true, detail),
        Err(error) => ("linker".to_string(), false, format!("{error:#}")),
    }
}

fn list(json_output: bool) -> Result<()> {
    let deps = build_deps()?;
    let progress = deps.progress.load();

    if json_output {
        let tracks: Vec<_> = deps
            .content
            .tracks()
            .iter()
            .map(|track| {
                let projects: Vec<_> = track
                    .projects
                    .iter()
                    .filter_map(|id| deps.content.project(id))
                    .map(|project| {
                        let total = project.milestones.len();
                        let completed = project
                            .milestones
                            .iter()
                            .filter(|milestone| {
                                progress
                                    .completed
                                    .contains(&milestone_key(&project.id, &milestone.id))
                            })
                            .count();
                        json!({
                            "id": &project.id,
                            "title": &project.title,
                            "completed": completed,
                            "total": total
                        })
                    })
                    .collect();
                json!({"id": &track.id, "title": &track.title, "projects": projects})
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({"tracks": tracks}))?
        );
        return Ok(());
    }

    for track in deps.content.tracks() {
        println!("{}\n", track.title);
        for project_id in &track.projects {
            let Some(project) = deps.content.project(project_id) else {
                continue;
            };
            let total = project.milestones.len();
            let completed = project
                .milestones
                .iter()
                .filter(|milestone| {
                    progress
                        .completed
                        .contains(&milestone_key(&project.id, &milestone.id))
                })
                .count();
            let is_current = project.milestones.iter().any(|milestone| {
                let key = milestone_key(&project.id, &milestone.id);
                progress.current.as_deref() == Some(key.as_str())
            });
            let marker = if is_current {
                ">"
            } else if completed == total {
                "x"
            } else {
                " "
            };
            println!(
                "[{marker}] {:<22} {completed}/{total} milestones",
                project.title
            );
        }
    }
    Ok(())
}

fn start(
    requested: Option<String>,
    dir: Option<PathBuf>,
    force: bool,
    json_output: bool,
) -> Result<()> {
    let deps = build_deps()?;
    let mut progress = deps.progress.load();
    let project = pick_project(&deps, &progress, requested.as_deref())?;
    let milestone = first_incomplete_milestone(project, &progress)
        .or_else(|| project.milestones.first())
        .context("project has no milestones")?;

    let cwd = std::env::current_dir()?;
    let target = match dir {
        Some(path) if path.is_absolute() => path,
        Some(path) => cwd.join(path),
        None => cwd.join(&project.id),
    };

    if target.exists() && !force {
        eprintln!(
            "error: {} already exists; pass --force to overwrite",
            target.display()
        );
        std::process::exit(2);
    }

    copy_dir(&project.starter, &target)?;
    let marker_dir = target.join(".rustlab");
    fs::create_dir_all(&marker_dir)?;
    let marker = ProjectMarker {
        schema_version: 1,
        project_id: project.id.clone(),
        milestone_id: milestone.id.clone(),
        content_version: env!("CARGO_PKG_VERSION").to_string(),
    };
    fs::write(
        marker_dir.join("project.json"),
        serde_json::to_string_pretty(&marker)?,
    )?;

    progress.current = Some(milestone_key(&project.id, &milestone.id));
    deps.progress.save(&progress)?;

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "project": &project.id,
                "milestone": &milestone.id,
                "path": &target
            }))?
        );
    } else {
        println!("Started {} at {}\n", project.title, target.display());
        print_task(milestone)?;
        println!("\nWhen you are ready, run: rustlab check");
    }
    Ok(())
}

fn pick_project<'a>(
    deps: &'a Deps,
    progress: &Progress,
    requested: Option<&str>,
) -> Result<&'a Project> {
    if let Some(id) = requested {
        return deps
            .content
            .project(id)
            .with_context(|| format!("unknown project: {id}"));
    }
    let mut first = None;
    for track in deps.content.tracks() {
        for id in &track.projects {
            let Some(project) = deps.content.project(id) else {
                continue;
            };
            if first.is_none() {
                first = Some(project);
            }
            if project.milestones.iter().any(|milestone| {
                !progress
                    .completed
                    .contains(&milestone_key(&project.id, &milestone.id))
            }) {
                return Ok(project);
            }
        }
    }
    first.context("no projects found in content")
}

fn first_incomplete_milestone<'a>(
    project: &'a Project,
    progress: &Progress,
) -> Option<&'a Milestone> {
    project.milestones.iter().find(|milestone| {
        !progress
            .completed
            .contains(&milestone_key(&project.id, &milestone.id))
    })
}

fn check(
    json_output: bool,
    project_arg: Option<String>,
    milestone_arg: Option<String>,
) -> Result<()> {
    let deps = build_deps()?;
    let mut progress = deps.progress.load();
    let milestone_requested = milestone_arg.is_some();
    let target = resolve_target(&deps, &progress, project_arg, milestone_arg)?;
    let project = deps
        .content
        .project(&target.project_id)
        .with_context(|| format!("unknown project: {}", target.project_id))?;
    recover_injections(&target.root)?;
    if !milestone_requested
        && project.milestones.iter().all(|milestone| {
            progress
                .completed
                .contains(&milestone_key(&project.id, &milestone.id))
        })
    {
        if json_output {
            let last = project
                .milestones
                .last()
                .context("project has no milestones")?;
            let report = CheckReport {
                project: project.id.clone(),
                milestone: last.id.clone(),
                passed: true,
                results: vec![CheckResult {
                    check_id: "project-complete".to_string(),
                    passed: true,
                    message: "project complete".to_string(),
                    detail: None,
                    duration_ms: 0,
                }],
                diagnostics: Vec::new(),
                duration_ms: 0,
            };
            println!("{}", serde_json::to_string_pretty(&report)?);
        } else {
            println!("{} is complete. Nice work.", project.title);
        }
        return Ok(());
    }
    let milestone = deps
        .content
        .milestone(&target.project_id, &target.milestone_id)
        .with_context(|| {
            format!(
                "unknown milestone: {}/{}",
                target.project_id, target.milestone_id
            )
        })?;

    let _injections = inject_hidden_tests(
        deps.content.as_ref(),
        &target.project_id,
        &target.root,
        milestone,
    )?;
    let ctx = CheckContext {
        project_root: target.root.clone(),
        package_dir: target.root.clone(),
        target_dir: target.root.join("target"),
        binary_path: None,
    };

    let started = std::time::Instant::now();
    let mut results = Vec::new();
    let mut diagnostics = collect_diagnostics(&deps, &ctx, milestone)?;
    if milestone.checks.is_empty() {
        anyhow::bail!("milestone has no checks");
    }
    'checks: for spec in &milestone.checks {
        let check_started = std::time::Instant::now();
        let commands = plan(spec, &ctx)?;
        let mut outputs = Vec::new();
        for command in commands {
            match deps.runner.run(&command) {
                Ok(output) => {
                    diagnostics.extend(parse_cargo_diagnostics(&output.stdout, &output.stderr));
                    outputs.push(output);
                }
                Err(error) => {
                    results.push(CheckResult {
                        check_id: spec.id.clone(),
                        passed: false,
                        message: format!("command failed: {error}"),
                        detail: None,
                        duration_ms: check_started.elapsed().as_millis(),
                    });
                    continue 'checks;
                }
            }
        }
        let mut result = evaluate(spec, &outputs);
        result.duration_ms = check_started.elapsed().as_millis();
        results.push(result);
    }

    let passed = !results.is_empty() && results.iter().all(|result| result.passed);
    let report = CheckReport {
        project: project.id.clone(),
        milestone: milestone.id.clone(),
        passed,
        results,
        diagnostics,
        duration_ms: started.elapsed().as_millis(),
    };

    let key = milestone_key(&project.id, &milestone.id);
    progress.attempts.push(rustlab_core::Attempt {
        project: project.id.clone(),
        milestone: milestone.id.clone(),
        at: time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| "unknown".to_string()),
        passed,
    });
    if passed {
        progress.completed.insert(key.clone());
        let next = deps
            .content
            .next_milestone(&project.id, &progress.completed)
            .map(|next| next.id.clone());
        progress.current = next
            .as_ref()
            .map(|milestone| milestone_key(&project.id, milestone));
        if let Some(next) = next {
            update_marker(&target.root, &project.id, &next)?;
        }
    }
    deps.progress.save(&progress)?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_report(&report, &deps.content_root);
    }

    if !passed {
        drop(_injections);
        std::process::exit(1);
    }
    Ok(())
}

fn collect_diagnostics(
    deps: &Deps,
    ctx: &CheckContext,
    milestone: &Milestone,
) -> Result<Vec<Diagnostic>> {
    let mut packages = BTreeSet::new();
    for spec in &milestone.checks {
        if let Some(package) = check_package(&spec.kind) {
            packages.insert(package.to_string());
        }
    }

    let mut diagnostics = Vec::new();
    for package in packages {
        let mut env = BTreeMap::new();
        env.insert(
            "CARGO_TARGET_DIR".to_string(),
            ctx.target_dir.to_string_lossy().into_owned(),
        );
        let command = CommandSpec {
            program: "cargo".to_string(),
            args: vec![
                "check".to_string(),
                "--package".to_string(),
                package,
                "--message-format=json".to_string(),
            ],
            cwd: ctx.package_dir.clone(),
            stdin: None,
            env,
            timeout_secs: 30,
            output_cap_bytes: 64 * 1024,
        };
        if let Ok(output) = deps.runner.run(&command) {
            diagnostics.extend(parse_cargo_diagnostics(&output.stdout, &output.stderr));
        }
    }
    Ok(diagnostics)
}

fn check_package(kind: &CheckKind) -> Option<&str> {
    match kind {
        CheckKind::CargoTest { package, .. }
        | CheckKind::Cli { package, .. }
        | CheckKind::CompileFail { package, .. }
        | CheckKind::Clippy { package, .. } => Some(package),
    }
}

fn resolve_target(
    deps: &Deps,
    progress: &Progress,
    project_arg: Option<String>,
    milestone_arg: Option<String>,
) -> Result<Target> {
    let cwd = std::env::current_dir()?;
    let marker = find_marker(&cwd);
    let marker_root = marker.as_ref().map(|(root, _)| root.clone());
    let marker_project = marker.as_ref().map(|(_, marker)| marker.project_id.clone());
    let marker_milestone = marker
        .as_ref()
        .map(|(_, marker)| marker.milestone_id.clone());

    let project_id = project_arg
        .or_else(|| marker_project.clone())
        .context("no project selected; run rustlab start or pass --project")?;

    let marker_same = marker_project.as_deref() == Some(project_id.as_str());
    let root = if marker_same {
        marker_root.unwrap_or_else(|| cwd.clone())
    } else {
        let candidate = cwd.join(&project_id);
        if candidate.join(".rustlab").join("project.json").is_file() {
            candidate
        } else {
            anyhow::bail!(
                "run inside the project directory or start it first: {}",
                candidate.display()
            );
        }
    };

    let project = deps
        .content
        .project(&project_id)
        .with_context(|| format!("unknown project: {project_id}"))?;

    let marker_completed = marker_milestone
        .as_ref()
        .map(|milestone| {
            progress
                .completed
                .contains(&milestone_key(&project_id, milestone))
        })
        .unwrap_or(false);
    let current_from_progress = progress.current.as_deref().and_then(|key| {
        let (project, milestone) = key.split_once('/')?;
        (project == project_id).then(|| milestone.to_string())
    });

    let milestone_id = milestone_arg
        .or_else(|| {
            if marker_same && !marker_completed {
                marker_milestone.clone()
            } else {
                None
            }
        })
        .or(current_from_progress)
        .or_else(|| {
            first_incomplete_milestone(project, progress).map(|milestone| milestone.id.clone())
        })
        .or_else(|| {
            project
                .milestones
                .first()
                .map(|milestone| milestone.id.clone())
        })
        .context("project has no milestones")?;

    Ok(Target {
        project_id,
        milestone_id,
        root,
    })
}

struct Target {
    project_id: String,
    milestone_id: String,
    root: PathBuf,
}

fn find_marker(start: &Path) -> Option<(PathBuf, ProjectMarker)> {
    let mut dir = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    loop {
        let marker_path = dir.join(".rustlab").join("project.json");
        if marker_path.is_file()
            && let Ok(raw) = fs::read_to_string(&marker_path)
            && let Ok(marker) = serde_json::from_str::<ProjectMarker>(&raw)
        {
            return Some((dir.clone(), marker));
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

fn update_marker(root: &Path, project_id: &str, milestone_id: &str) -> Result<()> {
    let directory = root.join(".rustlab");
    fs::create_dir_all(&directory)?;
    let marker = ProjectMarker {
        schema_version: 1,
        project_id: project_id.to_string(),
        milestone_id: milestone_id.to_string(),
        content_version: env!("CARGO_PKG_VERSION").to_string(),
    };
    fs::write(
        directory.join("project.json"),
        serde_json::to_string_pretty(&marker)?,
    )?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct InjectionRecord {
    target: PathBuf,
    backup: Option<PathBuf>,
}

struct InjectionGuard {
    root: PathBuf,
    records: Vec<InjectionRecord>,
}

impl Drop for InjectionGuard {
    fn drop(&mut self) {
        restore_injections(&self.root, &self.records);
    }
}

fn injections_manifest(root: &Path) -> PathBuf {
    root.join(".rustlab").join("injections.json")
}

fn restore_injections(root: &Path, records: &[InjectionRecord]) {
    let tests_dir = root.join("tests");
    let backups_dir = root.join(".rustlab").join("backups");
    let tests_dir = tests_dir.canonicalize().unwrap_or(tests_dir);
    let backups_dir = backups_dir.canonicalize().unwrap_or(backups_dir);

    for record in records {
        if !is_valid_injection_target(&record.target, &tests_dir) {
            continue;
        }
        if let Some(backup) = &record.backup {
            let backup_ok = backup
                .parent()
                .and_then(|parent| parent.canonicalize().ok())
                .as_deref()
                == Some(backups_dir.as_path())
                && backup.is_file();
            if backup_ok {
                let _ = fs::remove_file(&record.target);
                let _ = fs::rename(backup, &record.target);
            }
        } else {
            let _ = fs::remove_file(&record.target);
        }
    }
    let _ = fs::remove_file(injections_manifest(root));
}

fn is_valid_injection_target(target: &Path, tests_dir: &Path) -> bool {
    target.extension().and_then(|extension| extension.to_str()) == Some("rs")
        && target
            .parent()
            .and_then(|parent| parent.canonicalize().ok())
            .as_deref()
            == Some(tests_dir)
}

fn is_safe_test_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

fn recover_injections(root: &Path) -> Result<()> {
    let manifest = injections_manifest(root);
    if !manifest.is_file() {
        return Ok(());
    }
    let raw = fs::read_to_string(&manifest)?;
    if let Ok(records) = serde_json::from_str::<Vec<InjectionRecord>>(&raw) {
        restore_injections(root, &records);
    } else {
        let _ = fs::remove_file(&manifest);
    }
    Ok(())
}

fn inject_hidden_tests(
    content: &dyn ContentProvider,
    project_id: &str,
    project_root: &Path,
    milestone: &Milestone,
) -> Result<InjectionGuard> {
    let content_project = content
        .project_dir(project_id)
        .with_context(|| format!("missing content directory for {project_id}"))?;
    let tests_dir = project_root.join("tests");
    let backup_dir = project_root.join(".rustlab").join("backups");
    let mut records = Vec::new();
    let mut pending = Vec::new();

    for check in &milestone.checks {
        let CheckKind::CargoTest {
            test: Some(name), ..
        } = &check.kind
        else {
            continue;
        };
        if !is_safe_test_name(name) {
            anyhow::bail!("invalid hidden test name: {name}");
        }
        let source = content_project
            .join("tests")
            .join("hidden")
            .join(format!("{name}.rs"));
        if !source.is_file() {
            anyhow::bail!("hidden test not found: {}", source.display());
        }
        fs::create_dir_all(&tests_dir)?;
        let target = tests_dir.join(format!("{name}.rs"));
        let backup = if target.exists() {
            fs::create_dir_all(&backup_dir)?;
            let backup = backup_dir.join(format!("{}.bak", uuid::Uuid::new_v4()));
            fs::copy(&target, &backup)?;
            Some(backup)
        } else {
            None
        };
        records.push(InjectionRecord {
            target: target.clone(),
            backup,
        });
        pending.push((source, target));
    }

    if !records.is_empty() {
        let manifest = injections_manifest(project_root);
        if let Some(parent) = manifest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&manifest, serde_json::to_string_pretty(&records)?)?;
        for (source, target) in pending {
            fs::write(&target, fs::read(&source)?)?;
        }
    }

    Ok(InjectionGuard {
        root: project_root.to_path_buf(),
        records,
    })
}

fn print_report(report: &CheckReport, content_root: &Path) {
    for result in &report.results {
        println!(
            "{} {}  {}",
            if result.passed { "✓" } else { "✗" },
            result.check_id,
            result.message
        );
        if !result.passed
            && let Some(detail) = &result.detail
        {
            let cleaned = clean_detail(detail);
            for line in cleaned.lines().take(24) {
                println!("    {line}");
            }
        }
    }

    if !report.passed
        && let Ok(explainers) = ExplainerIndex::load(&content_root.join("errors.json"))
        && let Some(explainer) = explainers.explain_first(&report.diagnostics)
    {
        println!("\n{}: {}", explainer.code, explainer.title);
        println!("{}", explainer.body);
        println!("{}", explainer.link);
    }

    let failed = report
        .results
        .iter()
        .filter(|result| !result.passed)
        .count();
    if report.passed {
        println!("\nAll checks passed. Run: rustlab next");
    } else {
        println!(
            "\n{failed} of {} checks failed. Run `rustlab hint` for a nudge.",
            report.results.len()
        );
    }
}

fn clean_detail(detail: &str) -> String {
    detail
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            !line.starts_with("Compiling ")
                && !line.starts_with("Finished ")
                && !line.starts_with("Running ")
                && !line.starts_with("Blocking ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn hint(level: usize) -> Result<()> {
    let deps = build_deps()?;
    let progress = deps.progress.load();
    let target = resolve_target(&deps, &progress, None, None)?;
    let milestone = deps
        .content
        .milestone(&target.project_id, &target.milestone_id)
        .context("milestone not found")?;
    let index = level.max(1) - 1;
    let Some(path) = milestone.hints.get(index) else {
        println!("No hint at level {level}.");
        return Ok(());
    };
    let text = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    println!("{text}");
    Ok(())
}

fn solution(write: bool) -> Result<()> {
    let deps = build_deps()?;
    let progress = deps.progress.load();
    let target = resolve_target(&deps, &progress, None, None)?;
    recover_injections(&target.root)?;
    let project = deps
        .content
        .project(&target.project_id)
        .context("project not found")?;
    let milestone = deps
        .content
        .milestone(&target.project_id, &target.milestone_id)
        .context("milestone not found")?;

    let attempted = progress.attempts.iter().any(|attempt| {
        attempt.project == project.id && attempt.milestone == milestone.id && !attempt.passed
    });
    if !attempted {
        anyhow::bail!("run rustlab check first; the solution unlocks after a failed attempt");
    }

    if write {
        copy_dir(&milestone.solution, &target.root)?;
        println!("Applied the solution for {}/{}.", project.id, milestone.id);
        return Ok(());
    }

    println!("Solution for {}/{}:\n", project.id, milestone.id);
    print_solution_dir(&milestone.solution, &milestone.solution)?;
    Ok(())
}

fn print_solution_dir(directory: &Path, base: &Path) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(directory)?
        .filter_map(|entry| entry.ok())
        .collect();
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            print_solution_dir(&path, base)?;
        } else if let Ok(content) = fs::read_to_string(&path) {
            let relative = path.strip_prefix(base).unwrap_or(&path);
            println!("--- {} ---", relative.display());
            println!("{content}\n");
        }
    }
    Ok(())
}

fn next(json_output: bool) -> Result<()> {
    let deps = build_deps()?;
    let mut progress = deps.progress.load();
    let target = resolve_target(&deps, &progress, None, None)?;
    let project = deps
        .content
        .project(&target.project_id)
        .context("project not found")?;
    let current = deps
        .content
        .milestone(&target.project_id, &target.milestone_id)
        .context("milestone not found")?;
    let current_key = milestone_key(&project.id, &current.id);

    if !progress.completed.contains(&current_key) {
        let attempted = progress
            .attempts
            .iter()
            .any(|attempt| attempt.project == project.id && attempt.milestone == current.id);
        if attempted {
            if json_output {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({
                        "project": &project.id,
                        "milestone": &current.id,
                        "blocked": true,
                        "task": fs::read_to_string(&current.task).unwrap_or_default()
                    }))?
                );
            } else {
                println!(
                    "{} is not complete yet. Run `rustlab check` first.",
                    project.title
                );
            }
            return Ok(());
        }

        if json_output {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "project": &project.id,
                    "milestone": &current.id,
                    "task": fs::read_to_string(&current.task).unwrap_or_default()
                }))?
            );
        } else {
            print_task(current)?;
        }
        return Ok(());
    }

    let Some(milestone) = deps
        .content
        .next_milestone(&project.id, &progress.completed)
    else {
        if json_output {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "project": &project.id,
                    "complete": true
                }))?
            );
        } else {
            println!("{} is complete. Nice work.", project.title);
        }
        return Ok(());
    };

    progress.current = Some(milestone_key(&project.id, &milestone.id));
    update_marker(&target.root, &project.id, &milestone.id)?;
    deps.progress.save(&progress)?;

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "project": &project.id,
                "milestone": &milestone.id,
                "task": fs::read_to_string(&milestone.task).unwrap_or_default()
            }))?
        );
    } else {
        print_task(milestone)?;
    }
    Ok(())
}

fn print_task(milestone: &Milestone) -> Result<()> {
    println!("{}", milestone.title);
    let task = fs::read_to_string(&milestone.task)
        .with_context(|| format!("read {}", milestone.task.display()))?;
    let task = task
        .lines()
        .filter(|line| !line.trim_start().starts_with("```"))
        .collect::<Vec<_>>()
        .join("\n");
    println!("\n{task}");
    Ok(())
}

fn reset(yes: bool) -> Result<()> {
    let deps = build_deps()?;
    let progress = deps.progress.load();
    let target = resolve_target(&deps, &progress, None, None)?;
    recover_injections(&target.root)?;
    let project = deps
        .content
        .project(&target.project_id)
        .context("project not found")?;

    if !yes {
        println!(
            "This will overwrite files in {} with the starter. Continue? [y/N]",
            target.root.display()
        );
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        if !matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    copy_dir(&project.starter, &target.root)?;
    println!("Reset {} to the starter.", target.root.display());
    Ok(())
}

fn export(output: Option<PathBuf>) -> Result<()> {
    let progress = ProgressStore::from_env().load();
    let raw = serde_json::to_string_pretty(&progress)?;
    if let Some(path) = output {
        fs::write(&path, raw).with_context(|| format!("write {}", path.display()))?;
        println!("Wrote {}", path.display());
    } else {
        println!("{raw}");
    }
    Ok(())
}

fn copy_dir(source: &Path, target: &Path) -> Result<()> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source).with_context(|| format!("read {}", source.display()))? {
        let entry = entry?;
        let path = entry.path();
        let destination = target.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &destination)?;
        } else {
            let contents = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
            fs::write(&destination, contents)
                .with_context(|| format!("write {}", destination.display()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod progress_tests;
