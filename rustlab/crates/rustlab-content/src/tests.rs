use super::*;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn write_fixture(root: &Path) -> PathBuf {
    let track_dir = root.join("tracks").join("foundations");
    let project = track_dir.join("projects").join("01-demo");
    fs::create_dir_all(project.join("starter").join("src")).unwrap();
    fs::create_dir_all(project.join("lessons")).unwrap();
    fs::create_dir_all(project.join("tasks")).unwrap();
    fs::create_dir_all(project.join("hints")).unwrap();
    fs::create_dir_all(project.join("solutions").join("01")).unwrap();
    fs::create_dir_all(project.join("tests").join("hidden")).unwrap();

    fs::write(
        track_dir.join("track.toml"),
        r#"id = "foundations"
title = "Foundations"
projects = ["01-demo"]
"#,
    )
    .unwrap();

    fs::write(
        project.join("project.toml"),
        r#"id = "demo"
title = "Demo"
pitch = "A demo"
hours = 1
concepts = ["Result"]
starter = "starter"

[[milestone]]
id = "one"
title = "One"
lesson = "lessons/01.md"
task = "tasks/01.md"
hints = ["hints/01.md"]
solution = "solutions/01"

[[milestone.checks]]
id = "test"
kind = "cargo_test"
package = "demo"
test = "check"
"#,
    )
    .unwrap();

    fs::write(
        project.join("starter").join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    fs::write(project.join("starter").join("src").join("lib.rs"), "").unwrap();
    fs::write(project.join("lessons").join("01.md"), "lesson").unwrap();
    fs::write(project.join("tasks").join("01.md"), "task").unwrap();
    fs::write(project.join("hints").join("01.md"), "hint").unwrap();
    fs::write(project.join("solutions").join("01").join("lib.rs"), "").unwrap();
    fs::write(
        project.join("tests").join("hidden").join("check.rs"),
        "#[test]\nfn check() {}\n",
    )
    .unwrap();

    project
}

#[test]
fn loads_and_indexes_a_valid_tree() {
    let temp = tempfile::tempdir().unwrap();
    write_fixture(temp.path());
    let index = ContentIndex::load(temp.path()).unwrap();
    assert_eq!(index.tracks().len(), 1);
    assert_eq!(index.tracks()[0].projects, vec!["demo"]);
    let project = index.project("demo").unwrap();
    assert_eq!(project.milestones.len(), 1);
    assert!(project.starter.ends_with("starter"));
    assert!(index.project_dir("demo").is_some());
    assert!(index.milestone("demo", "one").is_some());
    assert!(index.next_milestone("demo", &BTreeSet::new()).is_some());
}

#[test]
fn rejects_a_missing_hidden_test() {
    let temp = tempfile::tempdir().unwrap();
    let project = write_fixture(temp.path());
    fs::remove_file(project.join("tests").join("hidden").join("check.rs")).unwrap();
    let error = ContentIndex::load(temp.path()).unwrap_err();
    assert!(error.to_string().contains("hidden test"));
}

#[test]
fn rejects_a_path_that_escapes_the_project() {
    let temp = tempfile::tempdir().unwrap();
    let project = write_fixture(temp.path());
    let outside = project.parent().unwrap().join("escape.md");
    fs::write(&outside, "outside").unwrap();
    let manifest = fs::read_to_string(project.join("project.toml")).unwrap();
    let manifest = manifest.replace("lesson = \"lessons/01.md\"", "lesson = \"../escape.md\"");
    fs::write(project.join("project.toml"), manifest).unwrap();
    let error = ContentIndex::load(temp.path()).unwrap_err();
    assert!(error.to_string().contains("escapes project directory"));
}

#[test]
fn rejects_duplicate_milestones() {
    let temp = tempfile::tempdir().unwrap();
    let project = write_fixture(temp.path());
    let manifest = fs::read_to_string(project.join("project.toml")).unwrap();
    let manifest = format!(
        "{manifest}\n[[milestone]]\nid = \"one\"\ntitle = \"Duplicate\"\nlesson = \"lessons/01.md\"\ntask = \"tasks/01.md\"\nhints = [\"hints/01.md\"]\nsolution = \"solutions/01\"\n\n[[milestone.checks]]\nid = \"test2\"\nkind = \"cargo_test\"\npackage = \"demo\"\ntest = \"check\"\n"
    );
    fs::write(project.join("project.toml"), manifest).unwrap();
    let error = ContentIndex::load(temp.path()).unwrap_err();
    assert!(
        error.to_string().contains("duplicate milestone id"),
        "{error}"
    );
}
