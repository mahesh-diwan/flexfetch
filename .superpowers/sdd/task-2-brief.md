# Task 2: Config + template engine

**Files:**
- Create: `sysfetch-core/src/config.rs`
- Create: `sysfetch-core/src/template.rs`
- Create: `templates/default.tera` (empty stub)
- Modify: `sysfetch-core/src/lib.rs` (add mod declarations)

**Depends on:** Task 1 (Module trait, Context, Error types)

## Step 1: config.rs

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

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
    pub color_keys: Option<String>,

    #[serde(default)]
    pub color_values: Option<String>,
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

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            separator: ": ".to_string(),
            key_width: 8,
            color_keys: None,
            color_values: None,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig { ttl: 60 }
    }
}

impl Config {
    pub fn default_modules() -> Vec<String> {
        vec![
            "title".into(), "separator".into(),
            "os".into(), "host".into(), "kernel".into(),
            "uptime".into(), "packages".into(),
            "shell".into(), "terminal".into(), "de".into(),
            "cpu".into(), "memory".into(), "disk".into(),
            "colors".into(),
        ]
    }

    pub fn default_template() -> String {
        "default".into()
    }

    pub fn load(path: Option<&std::path::Path>) -> crate::Result<Self> {
        let config_path = path.map(|p| p.to_path_buf())
            .or_else(find_config)
            .ok_or_else(|| crate::Error::Config("no config file found".into()))?;

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| crate::Error::Config(format!("cannot read {:?}: {e}", config_path)))?;

        toml::from_str(&content)
            .map_err(|e| crate::Error::Config(format!("parse error: {e}")))
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
    if p.exists() { Some(p) } else { None }
}
```

## Step 2: template.rs

```rust
use tera::{Tera, Context as TeraContext};

pub struct TeraEngine {
    tera: Tera,
    template_name: String,
}

impl TeraEngine {
    pub fn new_default() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template("default", include_str!("../../templates/default.tera"))
            .expect("default template is valid");
        TeraEngine { tera, template_name: "default".to_string() }
    }

    pub fn render(&self, info: &crate::SystemInfo, config: &crate::Config) -> crate::Result<String> {
        let mut ctx = TeraContext::new();
        for (name, value) in &info.entries {
            let json_val = serde_json::to_value(value)
                .map_err(|e| crate::Error::Template(format!("serialize {name}: {e}")))?;
            ctx.insert(name, &json_val);
        }
        ctx.insert("display_separator", &config.display.separator);
        ctx.insert("display_key_width", &config.display.key_width);
        self.tera.render(&self.template_name, &ctx)
            .map_err(|e| crate::Error::Template(e.to_string()))
    }
}
```

## Step 3: Create templates/default.tera (empty)
```bash
mkdir -p templates
touch templates/default.tera
```

## Step 4: Update lib.rs — add config and template modules

```rust
pub mod module;
pub mod context;
pub mod config;
pub mod template;
pub mod error;

pub use module::{Module, InfoValue, SystemInfo};
pub use context::Context;
pub use config::Config;
pub use template::TeraEngine;
pub use error::{Error, Result};
```

## Step 5: Verify compilation

Run: `cargo build -p sysfetch-core`

## Step 6: Commit

```bash
git add -A && git commit -m "feat: config loader + template engine"
git push
```
