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
        // Best-effort listing: an entry that fails to read (e.g. permission
        // denied, or a procfs entry vanishing mid-iteration) is dropped rather
        // than failing the whole listing — matches how sysfs/procfs tools
        // behave. The dir itself missing is still an `Err`.
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

    /// Register a file; every ancestor directory is implicitly registered, so
    /// `read_dir`/`exists`/`is_dir` on any level of the chain behave exactly as
    /// they would against a real filesystem.
    pub fn file(mut self, path: impl Into<PathBuf>, content: impl Into<String>) -> Self {
        let path = path.into();
        self.register_ancestors(&path);
        self.files.insert(path, content.into());
        self
    }

    /// Register a directory explicitly (e.g. empty dirs like `/proc` entries).
    pub fn dir(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.register_ancestors(&path);
        self.dirs.insert(path);
        self
    }

    /// Insert every ancestor of `path` (excluding the filesystem root) into
    /// `dirs`, so nested paths behave like a real tree.
    fn register_ancestors(&mut self, path: &Path) {
        let mut parent = path.parent();
        while let Some(p) = parent {
            if p.as_os_str().is_empty() {
                break;
            }
            self.dirs.insert(p.to_path_buf());
            parent = p.parent();
        }
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
        // Contract fidelity with RealFs: reading an unregistered dir is `Err`,
        // never a silently-empty `Ok` — otherwise a test could pass against the
        // mock while production takes the missing-dir branch.
        if !self.dirs.contains(path) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "mock directory not registered",
            ));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_read_dir_missing_is_err() {
        // Contract fidelity with RealFs: a missing dir must be `Err`, never a
        // silently-empty `Ok` — otherwise a test could pass against the mock
        // while production takes the missing-dir branch.
        let fs = MockFs::new().file("/etc/os-release", "NAME=\"Test\"\n");
        assert!(fs.read_dir(Path::new("/nonexistent")).is_err());
    }

    #[test]
    fn mock_ancestors_are_dirs() {
        // Registering a nested file implicitly creates its whole ancestor
        // chain, matching how a real filesystem behaves.
        let fs = MockFs::new().file("/sys/class/power_supply/BAT0/capacity", "79\n");
        assert!(fs.is_dir(Path::new("/sys/class/power_supply")));
        assert!(fs.is_dir(Path::new("/sys/class/power_supply/BAT0")));
        assert!(fs.exists(Path::new("/sys/class/power_supply/BAT0/capacity")));
        let children = fs.read_dir(Path::new("/sys/class/power_supply")).unwrap();
        assert_eq!(
            children,
            vec![PathBuf::from("/sys/class/power_supply/BAT0")]
        );
    }

    #[test]
    fn mock_read_dir_lists_direct_children_only() {
        let fs = MockFs::new()
            .file("/sys/class/drm/card0/modes", "1920x1080\n")
            .file("/sys/class/drm/card1/modes", "3840x2160\n")
            .file("/sys/class/drm/card0/device/vendor", "0x1002\n");
        // Only direct children of /sys/class/drm are listed; the nested
        // device/ subdir is not a sibling.
        let children = fs.read_dir(Path::new("/sys/class/drm")).unwrap();
        assert_eq!(
            children,
            vec![
                PathBuf::from("/sys/class/drm/card0"),
                PathBuf::from("/sys/class/drm/card1"),
            ]
        );
    }

    #[test]
    fn mock_read_to_string_missing_is_err() {
        let fs = MockFs::new();
        assert!(fs.read_to_string(Path::new("/proc/absent")).is_err());
    }
}
