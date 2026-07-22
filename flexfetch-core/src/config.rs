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

    #[serde(default)]
    pub modules_config: HashMap<String, ModuleConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModuleConfig {
    pub color_keys: Option<String>,
    pub color_values: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum LogoMode {
    #[default]
    Ascii,
    Block,
    Image,
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

    #[serde(default)]
    pub gradient: bool,

    #[serde(default)]
    pub gradient_colors: Option<Vec<String>>,

    #[serde(default)]
    pub logo_mode: LogoMode,
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
            gradient: false,
            gradient_colors: None,
            logo_mode: LogoMode::default(),
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
        // Start with defaults
        let mut config = Config::default_for_testing();

        // Layer 1: User config ($XDG_CONFIG_HOME/flexfetch/config.toml)
        if let Some(user_config) = find_user_config() {
            if let Ok(content) = std::fs::read_to_string(&user_config) {
                if let Ok(merged) = toml::from_str::<Config>(&content) {
                    config = merge_config(config, merged);
                }
            }
        }

        // Layer 2: Project config (./flexfetch.toml)
        if let Ok(cwd) = std::env::current_dir() {
            let project_config = cwd.join("flexfetch.toml");
            if project_config.exists() {
                if let Ok(content) = std::fs::read_to_string(&project_config) {
                    if let Ok(merged) = toml::from_str::<Config>(&content) {
                        config = merge_config(config, merged);
                    }
                }
            }
        }

        // Layer 3: Explicit path (CLI --config)
        if let Some(explicit) = path {
            if let Ok(content) = std::fs::read_to_string(explicit) {
                if let Ok(merged) = toml::from_str::<Config>(&content) {
                    config = merge_config(config, merged);
                }
            }
        }

        Ok(config)
    }

    pub fn default_for_testing() -> Self {
        Config {
            modules: Config::default_modules(),
            plugins_dir: None,
            template: Config::default_template(),
            display: DisplayConfig::default(),
            cache: CacheConfig::default(),
            custom: HashMap::new(),
            modules_config: HashMap::new(),
        }
    }
}

fn find_user_config() -> Option<PathBuf> {
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

fn merge_config(base: Config, override_config: Config) -> Config {
    Config {
        modules: if override_config.modules != Config::default_modules() {
            override_config.modules
        } else {
            base.modules
        },
        template: if override_config.template != "default" {
            override_config.template
        } else {
            base.template
        },
        plugins_dir: override_config.plugins_dir.or(base.plugins_dir),
        display: DisplayConfig {
            separator: if override_config.display.separator != ": " {
                override_config.display.separator
            } else {
                base.display.separator
            },
            key_width: if override_config.display.key_width != 8 {
                override_config.display.key_width
            } else {
                base.display.key_width
            },
            theme: override_config.display.theme.or(base.display.theme),
            color_title: override_config
                .display
                .color_title
                .or(base.display.color_title),
            color_keys: override_config
                .display
                .color_keys
                .or(base.display.color_keys),
            color_values: override_config
                .display
                .color_values
                .or(base.display.color_values),
            color_sep: override_config.display.color_sep.or(base.display.color_sep),
            gradient: override_config.display.gradient || base.display.gradient,
            gradient_colors: override_config
                .display
                .gradient_colors
                .or(base.display.gradient_colors),
            logo_mode: override_config.display.logo_mode,
        },
        cache: override_config.cache,
        custom: if !override_config.custom.is_empty() {
            override_config.custom
        } else {
            base.custom
        },
        modules_config: if !override_config.modules_config.is_empty() {
            override_config.modules_config
        } else {
            base.modules_config
        },
    }
}
