use crate::cache::Cache;
use crate::config::CustomModule;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Context {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub debug: bool,
    pub cache: Mutex<Cache>,
    pub custom_modules: HashMap<String, CustomModule>,
}

impl Context {
    pub fn new(
        config_dir: PathBuf,
        cache_dir: PathBuf,
        debug: bool,
        custom_modules: HashMap<String, CustomModule>,
    ) -> Self {
        let cache = Cache::new(cache_dir.clone(), 60);
        Context {
            config_dir,
            cache_dir,
            debug,
            cache: Mutex::new(cache),
            custom_modules,
        }
    }

    /// Read a file through the context abstraction. Modules should use this
    /// instead of std::fs::read_to_string to enable testing with mock data.
    pub fn read_file(&self, path: impl AsRef<std::path::Path>) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_returns_content() {
        let dir = std::env::temp_dir().join(format!("ff-ctx-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.txt");
        std::fs::write(&path, "hello").unwrap();
        let ctx = Context::new(dir.clone(), dir.clone(), false, Default::default());
        assert_eq!(ctx.read_file(&path).unwrap(), "hello");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
