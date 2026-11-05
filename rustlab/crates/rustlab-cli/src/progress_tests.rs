#[cfg(test)]
mod tests {
    use crate::progress::ProgressStore;
    use rustlab_core::Progress;
    use std::fs;

    #[test]
    fn saves_and_loads_progress() {
        let temp = tempfile::tempdir().unwrap();
        let store = ProgressStore::at(temp.path().to_path_buf());
        let mut progress = Progress::new();
        progress.completed.insert("demo/one".to_string());
        progress.current = Some("demo/two".to_string());
        store.save(&progress).unwrap();
        let loaded = store.load();
        assert_eq!(loaded.completed, progress.completed);
        assert_eq!(loaded.current, progress.current);
    }

    #[test]
    fn backs_up_a_corrupt_file() {
        let temp = tempfile::tempdir().unwrap();
        let store = ProgressStore::at(temp.path().to_path_buf());
        fs::create_dir_all(temp.path()).unwrap();
        let path = store.home().join("state.json");
        fs::write(&path, "not json").unwrap();
        let loaded = store.load();
        assert!(loaded.completed.is_empty());
        assert!(path.with_extension("json.bak").exists());
    }

    #[test]
    fn creates_a_writable_home() {
        let temp = tempfile::tempdir().unwrap();
        let store = ProgressStore::at(temp.path().join("nested").join("home"));
        store.ensure_writable().unwrap();
        assert!(store.home().is_dir());
    }
}
