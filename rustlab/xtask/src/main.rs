use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rustlab_check::{evaluate, plan};
use rustlab_content::ContentIndex;
use rustlab_core::{
    CheckContext, CheckKind, CheckResult, ContentProvider, Milestone, ProcessRunner, Project,
    milestone_key,
};
use rustlab_runner::LocalRunner;

#[derive(Parser)]
#[command(name = "xtask")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Validate,
    E2e {
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        milestone: Option<String>,
    },
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match Cli::parse().command {
        Commands::Validate => validate(),
        Commands::E2e { project, milestone } => e2e(project, milestone),
    }
}

fn content_root() -> Result<PathBuf> {
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
    let cwd = std::env::current_dir()?;
    let candidate = cwd.join("content");
    if candidate.join("tracks").is_dir() {
        return Ok(candidate);
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidate = manifest.join("..").join("content");
    if candidate.join("tracks").is_dir() {
        return Ok(candidate);
    }
    anyhow::bail!("content directory not found from {}", cwd.display())
}

fn validate() -> Result<()> {
    let root = content_root()?;
    let content = ContentIndex::load(&root)?;
    let runner = LocalRunner::new();
    let mut rows = Vec::new();

    for track in content.tracks() {
        for project_id in &track.projects {
            let project = content
                .project(project_id)
                .with_context(|| format!("missing project {project_id}"))?;
            for milestone in &project.milestones {
                let solution = run_case(&content, project, milestone, Case::Solution, &runner)?
                    .expect("solution always runs");
                let starter = run_case(&content, project, milestone, Case::Starter, &runner)?
                    .expect("starter always runs");
                let broken = run_case(&content, project, milestone, Case::Broken, &runner)?;

                let solution_ok = solution.iter().all(|result| result.passed);
                let starter_fails = starter.iter().any(|result| !result.passed);
                let broken_fails = broken
                    .as_ref()
                    .map(|results| results.iter().any(|result| !result.passed));
                let broken_ok = broken_fails.unwrap_or(true);

                rows.push((
                    project.id.clone(),
                    milestone.id.clone(),
                    solution_ok,
                    starter_fails,
                    broken_fails,
                ));

                if !solution_ok || !starter_fails || !broken_ok {
                    println!(
                        "FAIL {}/{} solution={} starter_fails={} broken_fails={:?}",
                        project.id, milestone.id, solution_ok, starter_fails, broken_fails
                    );
                    print_results("solution", &solution);
                    print_results("starter", &starter);
                    if let Some(results) = &broken {
                        print_results("broken", results);
                    }
                }
            }
        }
    }

    println!(
        "\n{:<20} {:<14} {:<10} {:<10} {:<10}",
        "project", "milestone", "solution", "starter", "broken"
    );
    let mut failed = false;
    for (project, milestone, solution, starter, broken) in &rows {
        let broken_label = match broken {
            Some(true) => "pass",
            Some(false) => "FAIL",
            None => "n/a",
        };
        if !*solution || !*starter || matches!(broken, Some(false)) {
            failed = true;
        }
        println!(
            "{:<20} {:<14} {:<10} {:<10} {:<10}",
            project,
            milestone,
            if *solution { "pass" } else { "FAIL" },
            if *starter { "fail" } else { "FAIL" },
            broken_label
        );
    }

    if failed {
        anyhow::bail!("content validation failed");
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Case {
    Solution,
    Starter,
    Broken,
}

fn run_case(
    content: &ContentIndex,
    project: &Project,
    milestone: &Milestone,
    case: Case,
    runner: &LocalRunner,
) -> Result<Option<Vec<CheckResult>>> {
    let project_dir = content
        .project_dir(&project.id)
        .context("missing project directory")?;
    let temp = tempfile::tempdir()?;
    let workspace = temp.path().join("project");
    copy_dir(&project.starter, &workspace)?;

    match case {
        Case::Solution => {
            copy_dir(&milestone.solution, &workspace)?;
        }
        Case::Starter => {}
        Case::Broken => {
            let broken = broken_dir(project_dir, milestone);
            if !broken.is_dir() {
                return Ok(None);
            }
            copy_dir(&broken, &workspace)?;
        }
    }

    copy_hidden_tests(project_dir, &workspace, milestone)?;
    let ctx = CheckContext {
        project_root: workspace.clone(),
        package_dir: workspace.clone(),
        target_dir: workspace.join("target"),
        binary_path: None,
    };

    let mut results = Vec::new();
    'checks: for spec in &milestone.checks {
        let commands = plan(spec, &ctx)?;
        let mut outputs = Vec::new();
        for command in commands {
            match runner.run(&command) {
                Ok(output) => outputs.push(output),
                Err(error) => {
                    results.push(CheckResult {
                        check_id: spec.id.clone(),
                        passed: false,
                        message: format!("command failed: {error}"),
                        detail: None,
                        duration_ms: 0,
                    });
                    continue 'checks;
                }
            }
        }
        results.push(evaluate(spec, &outputs));
    }
    Ok(Some(results))
}

fn broken_dir(project_dir: &Path, milestone: &Milestone) -> PathBuf {
    let direct = project_dir.join("broken").join(&milestone.id);
    if direct.is_dir() {
        return direct;
    }
    let name = milestone
        .solution
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(&milestone.id);
    project_dir.join("broken").join(name)
}

fn copy_hidden_tests(project_dir: &Path, workspace: &Path, milestone: &Milestone) -> Result<()> {
    let tests = workspace.join("tests");
    fs::create_dir_all(&tests)?;
    for check in &milestone.checks {
        let CheckKind::CargoTest {
            test: Some(name), ..
        } = &check.kind
        else {
            continue;
        };
        let source = project_dir
            .join("tests")
            .join("hidden")
            .join(format!("{name}.rs"));
        if !source.is_file() {
            anyhow::bail!("hidden test not found: {}", source.display());
        }
        fs::write(tests.join(format!("{name}.rs")), fs::read(&source)?)?;
    }
    Ok(())
}

fn print_results(label: &str, results: &[CheckResult]) {
    for result in results {
        println!(
            "  {label}: {} {} {}",
            if result.passed { "pass" } else { "FAIL" },
            result.check_id,
            result.message
        );
    }
}

fn e2e(project_filter: Option<String>, milestone_filter: Option<String>) -> Result<()> {
    let root = content_root()?;
    let content = ContentIndex::load(&root)?;
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()?;
    let status = Command::new("cargo")
        .args(["build", "--quiet", "--package", "rustlab-cli"])
        .current_dir(&workspace_root)
        .status()?;
    if !status.success() {
        anyhow::bail!("failed to build rustlab-cli");
    }
    let binary = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace_root.join("target"))
        .join("debug")
        .join(if cfg!(windows) {
            "rustlab.exe"
        } else {
            "rustlab"
        });
    let temp = tempfile::tempdir()?;
    let home = temp.path().join("home");
    let workspace = temp.path().join("project");

    let project = match project_filter.as_deref() {
        Some(id) => content
            .project(id)
            .with_context(|| format!("unknown project: {id}"))?,
        None => content
            .tracks()
            .iter()
            .flat_map(|track| track.projects.iter())
            .find_map(|id| content.project(id))
            .context("no project found")?,
    };

    let start = Command::new(&binary)
        .env("RUSTLAB_HOME", &home)
        .env("RUSTLAB_CONTENT", &root)
        .args(["start", &project.id, "--dir"])
        .arg(&workspace)
        .output()?;
    if !start.status.success() {
        anyhow::bail!(
            "start failed:\n{}{}",
            String::from_utf8_lossy(&start.stdout),
            String::from_utf8_lossy(&start.stderr)
        );
    }

    let project_dir = content
        .project_dir(&project.id)
        .context("missing project directory")?;

    let first = project
        .milestones
        .first()
        .context("project has no milestones")?;
    let initial = cli_command(&binary, &home, &root, &workspace, &["check", "--json"]).output()?;
    if initial.status.success() {
        anyhow::bail!("starter check unexpectedly passed");
    }
    copy_dir(&first.solution, &workspace)?;
    let passed = cli_command(&binary, &home, &root, &workspace, &["check", "--json"]).output()?;
    if !passed.status.success() {
        anyhow::bail!(
            "marker check failed:\n{}{}",
            String::from_utf8_lossy(&passed.stdout),
            String::from_utf8_lossy(&passed.stderr)
        );
    }
    let advanced = cli_command(&binary, &home, &root, &workspace, &["next"]).output()?;
    if !advanced.status.success() {
        anyhow::bail!(
            "next failed:\n{}{}",
            String::from_utf8_lossy(&advanced.stdout),
            String::from_utf8_lossy(&advanced.stderr)
        );
    }
    let advanced_stdout = String::from_utf8_lossy(&advanced.stdout);
    if let Some(expected) = project.milestones.get(1)
        && !advanced_stdout.contains(&expected.title)
    {
        anyhow::bail!(
            "next did not print the next task ({}): {advanced_stdout}",
            expected.title
        );
    }
    let next_check =
        cli_command(&binary, &home, &root, &workspace, &["check", "--json"]).output()?;
    if next_check.status.success() {
        anyhow::bail!("check after next should run the next milestone and fail");
    }

    let mut ran = 0;
    for milestone in &project.milestones {
        if let Some(filter) = &milestone_filter
            && &milestone.id != filter
        {
            continue;
        }
        ran += 1;

        copy_dir(&milestone.solution, &workspace)?;
        let report = run_cli_check(
            &binary,
            &home,
            &root,
            &workspace,
            &project.id,
            &milestone.id,
        )?;
        if !report.passed {
            anyhow::bail!(
                "solution failed for {}/{}: {}",
                project.id,
                milestone.id,
                report
                    .results
                    .iter()
                    .map(|r| format!("{}: {}", r.check_id, r.message))
                    .collect::<Vec<_>>()
                    .join("; ")
            );
        }

        let state = fs::read_to_string(home.join("state.json"))?;
        let progress: rustlab_core::Progress = serde_json::from_str(&state)?;
        let key = milestone_key(&project.id, &milestone.id);
        if !progress.completed.contains(&key) {
            anyhow::bail!("progress did not record {key}");
        }

        let broken = broken_dir(project_dir, milestone);
        if broken.is_dir() {
            copy_dir(&broken, &workspace)?;
            let failed = Command::new(&binary)
                .env("RUSTLAB_HOME", &home)
                .env("RUSTLAB_CONTENT", &root)
                .args([
                    "check",
                    "--json",
                    "--project",
                    project.id.as_str(),
                    "--milestone",
                    milestone.id.as_str(),
                ])
                .current_dir(&workspace)
                .output()?;
            if failed.status.success() {
                anyhow::bail!("broken variant passed for {}/{}", project.id, milestone.id);
            }
            let report: rustlab_core::CheckReport = serde_json::from_slice(&failed.stdout)
                .with_context(|| {
                    format!(
                        "parse failed check report for {}/{}: {}",
                        project.id,
                        milestone.id,
                        String::from_utf8_lossy(&failed.stdout)
                    )
                })?;
            if report.passed {
                anyhow::bail!(
                    "broken variant reported passed for {}/{}",
                    project.id,
                    milestone.id
                );
            }
        }

        copy_dir(&milestone.solution, &workspace)?;
        println!("ok {}/{}", project.id, milestone.id);
    }

    if ran == 0 {
        anyhow::bail!("no milestone matched the filter");
    }

    Ok(())
}

fn cli_command(
    binary: &Path,
    home: &Path,
    content: &Path,
    workspace: &Path,
    args: &[&str],
) -> Command {
    let mut command = Command::new(binary);
    command
        .env("RUSTLAB_HOME", home)
        .env("RUSTLAB_CONTENT", content)
        .args(args)
        .current_dir(workspace);
    command
}

fn run_cli_check(
    binary: &Path,
    home: &Path,
    content: &Path,
    workspace: &Path,
    project: &str,
    milestone: &str,
) -> Result<rustlab_core::CheckReport> {
    let output = Command::new(binary)
        .env("RUSTLAB_HOME", home)
        .env("RUSTLAB_CONTENT", content)
        .args([
            "check",
            "--json",
            "--project",
            project,
            "--milestone",
            milestone,
        ])
        .current_dir(workspace)
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "check failed:\n{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    serde_json::from_slice(&output.stdout).context("parse check report")
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
