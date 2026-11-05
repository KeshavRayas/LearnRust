use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rustlab_core::Progress;

pub struct ProgressStore {
    home: PathBuf,
    path: PathBuf,
}

impl ProgressStore {
    pub fn from_env() -> Self {
        let home = std::env::var_os("RUSTLAB_HOME")
            .map(PathBuf::from)
            .or_else(|| dirs::data_local_dir().map(|path| path.join("rustlab")))
            .unwrap_or_else(|| std::env::temp_dir().join("rustlab"));
        Self::at(home)
    }

    pub fn at(home: PathBuf) -> Self {
        let path = home.join("state.json");
        Self { home, path }
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    pub fn load(&self) -> Progress {
        let Ok(raw) = fs::read_to_string(&self.path) else {
            return Progress::new();
        };
        match serde_json::from_str::<Progress>(&raw) {
            Ok(progress) if progress.schema_version == 1 => progress,
            _ => {
                let backup = self.path.with_extension("json.bak");
                let _ = fs::rename(&self.path, backup);
                Progress::new()
            }
        }
    }

    pub fn save(&self, progress: &Progress) -> Result<()> {
        fs::create_dir_all(&self.home)
            .with_context(|| format!("create {}", self.home.display()))?;
        let tmp = self.path.with_extension("json.tmp");
        let raw = serde_json::to_string_pretty(progress)?;
        fs::write(&tmp, raw).with_context(|| format!("write {}", tmp.display()))?;
        if fs::rename(&tmp, &self.path).is_err() {
            let _ = fs::remove_file(&self.path);
            fs::rename(&tmp, &self.path)
                .with_context(|| format!("rename {}", self.path.display()))?;
        }
        Ok(())
    }

    pub fn ensure_writable(&self) -> Result<()> {
        fs::create_dir_all(&self.home)
            .with_context(|| format!("create {}", self.home.display()))?;
        let probe = self.home.join(".write-test");
        fs::write(&probe, b"ok").with_context(|| format!("write {}", probe.display()))?;
        let _ = fs::remove_file(probe);
        Ok(())
    }
}
