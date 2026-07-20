use crate::cache::Cache;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Context {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub debug: bool,
    pub cache: Mutex<Cache>,
}

impl Context {
    pub fn new(config_dir: PathBuf, cache_dir: PathBuf, debug: bool) -> Self {
        let cache = Cache::new(cache_dir.clone(), 60);
        Context {
            config_dir,
            cache_dir,
            debug,
            cache: Mutex::new(cache),
        }
    }
}
