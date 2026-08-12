//! Read-side filesystem seam.
//!
//! Module collectors read system data (`/proc`, `/sys`, `/etc`, package DBs)
//! through `Context::read_file` / `read_dir` / `exists` / `is_dir`, which
//! delegate to this trait. Production uses [`RealFs`]; unit tests inject a
//! [`MockFs`] with fake file contents so a collector can be tested without
//! touching the real machine.
//!
//! The interface is deliberately small: four operations cover the entire read
//! surface the modules use. Symlink resolution (`gpu` driver fallback) and
//! cache-file mtimes (`weather`, `autotheme`) stay on `std::fs` — they are
//! edge cases, not system-data reads.

#[cfg(test)]
use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};

/// The read-side filesystem contract modules rely on.
pub trait FileSystem: Send + Sync {
    /// Read a file's entire contents as UTF-8 (missing file → `Err`).
    fn read_to_string(&self, path: &Path) -> io::Result<String>;
    /// List the direct children of a directory (missing dir → `Err`).
    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
    /// Whether a path exists at all.
    fn exists(&self, path: &Path) -> bool;
    /// Whether a path exists and is a directory.
    fn is_dir(&self, path: &Path) -> bool;
}

/// Production adapter: delegates straight to `std::fs`.
#[derive(Default)]
pub struct RealFs;

impl FileSystem for RealFs {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }
}

/// In-memory filesystem for unit tests. Register files (and directories) up
/// front; every registered file's parent is implicitly a directory, so
/// `read_dir("/sys/class/net")` sees `eth0` once
/// `file("/sys/class/net/eth0/address", …)` is registered.
#[cfg(test)]
#[derive(Default)]
pub struct MockFs {
    files: HashMap<PathBuf, String>,
    dirs: HashSet<PathBuf>,
}

#[cfg(test)]
impl MockFs {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a file; its parent chain is implicitly registered as dirs.
    pub fn file(mut self, path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        let path = path.into();
        if let Some(parent) = path.parent() {
            self.dirs.insert(parent.to_path_buf());
        }
        self.files.insert(path, content.into());
        self
    }

    /// Register a directory explicitly (e.g. empty dirs like `/proc` entries).
    pub fn dir(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        if let Some(parent) = path.parent() {
            self.dirs.insert(parent.to_path_buf());
        }
        self.dirs.insert(path);
        self
    }
}

#[cfg(test)]
impl FileSystem for MockFs {
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files
            .get(path)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "mock file not registered"))
    }

    fn read_dir(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut out: Vec<PathBuf> = self
            .files
            .keys()
            .chain(self.dirs.iter())
            .filter(|p| p.parent() == Some(path))
            .cloned()
            .collect();
        out.sort();
        out.dedup();
        Ok(out)
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path) || self.dirs.contains(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        self.dirs.contains(path)
    }
}

/// Build a `Context` around a `MockFs` for module unit tests. Uses temp dirs
/// for config/cache so no real user files are touched.
#[cfg(test)]
pub fn test_ctx(fs: MockFs) -> crate::Context {
    crate::Context::with_fs(
        std::env::temp_dir().join("flexfetch-test-config"),
        std::env::temp_dir().join("flexfetch-test-cache"),
        false,
        HashMap::new(),
        Box::new(fs),
    )
}
