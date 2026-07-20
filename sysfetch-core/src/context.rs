use std::path::PathBuf;

pub struct Context {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub debug: bool,
}

impl Context {
    pub fn new(config_dir: PathBuf, cache_dir: PathBuf, debug: bool) -> Self {
        Context { config_dir, cache_dir, debug }
    }
}
