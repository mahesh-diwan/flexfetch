# Task 1: Workspace scaffold + core trait types

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `sysfetch-core/Cargo.toml`
- Create: `sysfetch-cli/Cargo.toml`
- Create: `sysfetch-lua/Cargo.toml`
- Create: `sysfetch-core/src/lib.rs`
- Create: `sysfetch-core/src/module.rs`
- Create: `sysfetch-core/src/context.rs`
- Create: `sysfetch-core/src/error.rs`

## Step 1: Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = ["sysfetch-core", "sysfetch-cli", "sysfetch-lua"]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
```

## Step 2: sysfetch-core/Cargo.toml

```toml
[package]
name = "sysfetch-core"
version.workspace = true
edition.workspace = true

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
tera = "1"
rayon = "1"
mlua = { version = "0.10", optional = true, features = ["lua54"] }
chrono = "0.4"
walkdir = "2"

[features]
default = []
lua = ["dep:mlua"]
```

## Step 3: sysfetch-cli/Cargo.toml

```toml
[package]
name = "sysfetch-cli"
version.workspace = true
edition.workspace = true

[dependencies]
sysfetch-core = { path = "../sysfetch-core", features = ["lua"] }
clap = { version = "4", features = ["derive"] }
serde_json = "1"
```

## Step 4: sysfetch-lua/Cargo.toml

```toml
[package]
name = "sysfetch-lua"
version.workspace = true
edition.workspace = true

[dependencies]
sysfetch-core = { path = "../sysfetch-core" }
mlua = { version = "0.10", features = ["lua54"] }
```

## Step 5: sysfetch-core/src/error.rs

```rust
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse(String),
    Config(String),
    Template(String),
    Lua(String),
    Module { name: &'static str, source: Box<Error> },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O: {e}"),
            Error::Parse(s) => write!(f, "parse: {s}"),
            Error::Config(s) => write!(f, "config: {s}"),
            Error::Template(s) => write!(f, "template: {s}"),
            Error::Lua(s) => write!(f, "lua: {s}"),
            Error::Module { name, source } => write!(f, "module {name}: {source}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self { Error::Io(e) }
}

pub type Result<T> = std::result::Result<T, Error>;
```

## Step 6: sysfetch-core/src/module.rs

```rust
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InfoValue {
    Scalar(String),
    Map(HashMap<String, String>),
    List(Vec<String>),
    Table(Vec<HashMap<String, String>>),
}

impl InfoValue {
    pub fn scalar(s: impl Into<String>) -> Self {
        InfoValue::Scalar(s.into())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            InfoValue::Scalar(s) => s.is_empty(),
            InfoValue::Map(m) => m.is_empty(),
            InfoValue::List(l) => l.is_empty(),
            InfoValue::Table(t) => t.is_empty(),
        }
    }
}

pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, ctx: &Context) -> crate::Result<InfoValue>;
}

pub struct SystemInfo {
    pub entries: Vec<(&'static str, InfoValue)>,
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo { entries: Vec::new() }
    }

    pub fn add(&mut self, name: &'static str, value: InfoValue) {
        self.entries.push((name, value));
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (name, value) in &self.entries {
            map.insert(name.to_string(), serde_json::to_value(value).unwrap_or_default());
        }
        serde_json::Value::Object(map)
    }
}
```

## Step 7: sysfetch-core/src/context.rs

```rust
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
```

## Step 8: sysfetch-core/src/lib.rs

```rust
pub mod module;
pub mod context;
pub mod error;

pub use module::{Module, InfoValue, SystemInfo};
pub use context::Context;
pub use error::{Error, Result};
```

## Step 9: Verify compilation

Run: `cargo build -p sysfetch-core`

## Step 10: Commit

```bash
git add -A && git commit -m "feat: scaffold workspace + core traits"
git push
```
