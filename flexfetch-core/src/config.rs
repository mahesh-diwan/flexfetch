use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "Config::default_modules")]
    pub modules: Vec<String>,

    #[serde(default)]
    pub plugins_dir: Option<PathBuf>,

    #[serde(default = "Config::default_template")]
    pub template: String,

    #[serde(default)]
    pub display: DisplayConfig,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub custom: HashMap<String, CustomModule>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DisplayConfig {
    #[serde(default = "DisplayConfig::default_separator")]
    pub separator: String,

    #[serde(default = "DisplayConfig::default_key_width")]
    pub key_width: usize,

    #[serde(default)]
    pub theme: Option<String>,

    #[serde(default)]
    pub color_title: Option<String>,

    #[serde(default)]
    pub color_keys: Option<String>,

    #[serde(default)]
    pub color_values: Option<String>,

    #[serde(default)]
    pub color_sep: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CacheConfig {
    #[serde(default = "CacheConfig::default_ttl")]
    pub ttl: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomModule {
    pub command: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub shell: Option<String>,
}

impl DisplayConfig {
    pub fn default_separator() -> String {
        ": ".to_string()
    }
    pub fn default_key_width() -> usize {
        8
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            separator: Self::default_separator(),
            key_width: Self::default_key_width(),
            theme: None,
            color_title: None,
            color_keys: None,
            color_values: None,
            color_sep: None,
        }
    }
}

impl CacheConfig {
    pub fn default_ttl() -> u64 {
        60
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            ttl: Self::default_ttl(),
        }
    }
}

impl Config {
    pub fn default_modules() -> Vec<String> {
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "host".into(),
            "kernel".into(),
            "uptime".into(),
            "packages".into(),
            "shell".into(),
            "terminal".into(),
            "de".into(),
            "wm".into(),
            "cpu".into(),
            "memory".into(),
            "gpu".into(),
            "disk".into(),
            "network".into(),
            "battery".into(),
            "locale".into(),
            "resolution".into(),
            "colors".into(),
        ]
    }

    pub fn default_template() -> String {
        "default".into()
    }

    pub fn load(path: Option<&std::path::Path>) -> crate::Result<Self> {
        let config_path = path
            .map(|p| p.to_path_buf())
            .or_else(find_config)
            .ok_or_else(|| crate::Error::Config("no config file found".into()))?;

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| crate::Error::Config(format!("cannot read {:?}: {e}", config_path)))?;

        toml::from_str(&content).map_err(|e| crate::Error::Config(format!("parse error: {e}")))
    }

    pub fn default_for_testing() -> Self {
        Config {
            modules: Config::default_modules(),
            plugins_dir: None,
            template: Config::default_template(),
            display: DisplayConfig::default(),
            cache: CacheConfig::default(),
            custom: HashMap::new(),
        }
    }
}

fn find_config() -> Option<PathBuf> {
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".config")
        });

    let p = xdg.join("flexfetch").join("config.toml");
    if p.exists() {
        Some(p)
    } else {
        None
    }
}
