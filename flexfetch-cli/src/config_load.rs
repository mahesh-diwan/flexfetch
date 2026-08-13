use flexfetch_core::{Config, Context};
use std::path::PathBuf;

pub struct LoadedConfig {
    pub config: Config,
    pub ctx: Context,
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
}

/// Load config and create context.
pub fn load(config_path: Option<&std::path::Path>, flash: bool, debug: bool) -> LoadedConfig {
    let config_dir = crate::tools::config_dir();
    let cache_dir = flexfetch_core::get_cache_dir();
    let config = if flash {
        Config::default_for_testing()
    } else {
        Config::load(config_path).unwrap_or_else(|_| Config::default_for_testing())
    };
    let ctx = Context::new(
        config_dir.clone(),
        cache_dir.clone(),
        debug,
        config.custom.clone(),
    );
    // Honor the config's cache_ttl key (default 60 s) for slow modules.
    ctx.set_cache_ttl(config.cache_ttl);
    LoadedConfig {
        config,
        ctx,
        config_dir,
        cache_dir,
    }
}
