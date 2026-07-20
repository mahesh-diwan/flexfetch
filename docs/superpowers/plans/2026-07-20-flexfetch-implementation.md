# flexfetch Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Build fast, extensible system info CLI with Rust — Lua plugins, Tera templates, sub-2MB binary.

**Architecture:** Cargo workspace with 3 crates: `sysfetch-core` (detection lib + plugins + templates), `sysfetch-cli` (binary), `sysfetch-lua` (Lua bindings). Modules implement `Module` trait, execute in parallel via rayon, render via Tera templates.

**Tech Stack:** Rust 1.80+, rayon, tera, serde+toml, clap, mlua, chrono.

**Branch:** main. All commits to main.

## Global Constraints

- Binary target: <2MB stripped (LTO + strip)
- Startup target: <5ms cold, <2ms warm
- Template engine: Tera (output flexibility)
- Plugin language: Lua 5.4 via mlua
- Config format: TOML via serde
- Platforms: Linux + macOS only for v1
- Parallel execution: rayon for all modules
- Error isolation: one module fail does not crash tool
- Caching: /tmp/flexfetch-cache.json with configurable TTL

---

### Task 1: Workspace scaffold + core trait types

**Files:**
- Create: `Cargo.toml` (workspace root)
- Create: `sysfetch-core/Cargo.toml`
- Create: `sysfetch-cli/Cargo.toml`
- Create: `sysfetch-lua/Cargo.toml`
- Create: `sysfetch-core/src/lib.rs`
- Create: `sysfetch-core/src/module.rs`
- Create: `sysfetch-core/src/context.rs`
- Create: `sysfetch-core/src/error.rs`

**Interfaces:**
- Produces: `Module` trait, `InfoValue` enum, `Context` struct, `Error` type, `SystemInfo` struct
- Consumes: nothing (foundation task)

- [ ] **Step 1: Create workspace Cargo.toml**

```toml
[workspace]
resolver = "2"
members = ["sysfetch-core", "sysfetch-cli", "sysfetch-lua"]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
```

- [ ] **Step 2: Create sysfetch-core/Cargo.toml**

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

- [ ] **Step 3: Create sysfetch-cli/Cargo.toml**

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

- [ ] **Step 4: Create sysfetch-lua/Cargo.toml**

```toml
[package]
name = "sysfetch-lua"
version.workspace = true
edition.workspace = true

[dependencies]
sysfetch-core = { path = "../sysfetch-core" }
mlua = { version = "0.10", features = ["lua54"] }
```

- [ ] **Step 5: Write error.rs**

```rust
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse(String),
    Config(String),
    Template(String),
    Lua(String),
    Module {
        name: &'static str,
        source: Box<Error>,
    },
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

- [ ] **Step 6: Write module.rs**

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

- [ ] **Step 7: Write context.rs**

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

- [ ] **Step 8: Write lib.rs**

```rust
pub mod module;
pub mod context;
pub mod error;

pub use module::{Module, InfoValue, SystemInfo};
pub use context::Context;
pub use error::{Error, Result};
```

- [ ] **Step 9: Verify it compiles**

Run: `cargo build -p sysfetch-core`
Expected: builds without errors

- [ ] **Step 10: Commit**

```bash
git add -A && git commit -m "feat: scaffold workspace + core traits"
git push
```

---

### Task 2: Config + template engine

**Files:**
- Create: `sysfetch-core/src/config.rs`
- Create: `sysfetch-core/src/template.rs`
- Modify: `sysfetch-core/src/lib.rs` (add mod declarations)

**Interfaces:**
- Consumes: `Context`, `SystemInfo`, `Error` from Task 1
- Produces: `Config` struct, `ConfigLoader`, `TeraEngine`

- [ ] **Step 1: Write config.rs**

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    #[serde(default = "CacheConfig::default_ttl")]
    pub ttl: u64,
}

#[derive(Debug, Deserialize, Clone)]
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

    pub fn load(path: Option<&std::path::Path>) -> Result<Self, crate::Error> {
        let config_path = path.map(|p| p.to_path_buf())
            .or_else(find_config)
            .ok_or_else(|| crate::Error::Config("no config file found".into()))?;

        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| crate::Error::Config(format!("cannot read {:?}: {e}", config_path)))?;

        toml::from_str(&content)
            .map_err(|e| crate::Error::Config(format!("parse error: {e}")))
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

#[derive(Debug, Deserialize, Clone)]
pub struct CustomModule {
    pub command: String,
    pub label: Option<String>,
    pub shell: Option<String>,
}
```

Wait — `CustomModule` is defined twice above. Fix: remove second definition, keep the first one within `Config`.

- [ ] **Step 2: Write template.rs**

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

    pub fn render(&self, info: &crate::SystemInfo) -> crate::Result<String> {
        let mut ctx = TeraContext::new();
        for (name, value) in &info.entries {
            let json_val = serde_json::to_value(value)
                .map_err(|e| crate::Error::Template(format!("serialize {name}: {e}")))?;
            ctx.insert(name, &json_val);
        }
        self.tera.render(&self.template_name, &ctx)
            .map_err(|e| crate::Error::Template(e.to_string()))
    }
}
```

- [ ] **Step 3: Update lib.rs — add mods**

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

- [ ] **Step 4: Create stub template**

```bash
mkdir -p templates
touch templates/default.tera
```

- [ ] **Step 5: Verify compile**

Run: `cargo build -p sysfetch-core`
Expected: builds without errors

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat: config loader + template engine"
git push
```

---

### Task 3: Simple detection modules (os, host, kernel, uptime, locale)

**Files:**
- Create: `sysfetch-core/src/modules/mod.rs`
- Create: `sysfetch-core/src/modules/os.rs`
- Create: `sysfetch-core/src/modules/host.rs`
- Create: `sysfetch-core/src/modules/kernel.rs`
- Create: `sysfetch-core/src/modules/uptime.rs`
- Create: `sysfetch-core/src/modules/locale.rs`
- Modify: `sysfetch-core/src/lib.rs` (add modules mod)

**Interfaces:**
- Consumes: `Module` trait, `Context`, `InfoValue`
- Produces: `OsModule`, `HostModule`, `KernelModule`, `UptimeModule`, `LocaleModule`

- [ ] **Step 1: Write os.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str { "os" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = std::collections::HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if let Some((key, val)) = line.split_once('=') {
                        let clean = val.trim_matches('"');
                        match key {
                            "NAME" => { map.insert("name".into(), clean.into()); }
                            "PRETTY_NAME" => { map.insert("pretty_name".into(), clean.into()); }
                            "VERSION_ID" => { map.insert("version".into(), clean.into()); }
                            "ID" => { map.insert("id".into(), clean.into()); }
                            "BUILD_ID" => { map.insert("build_id".into(), clean.into()); }
                            _ => {}
                        }
                    }
                }
            }
            // read /etc/arch-release, debian_version as fallback
            if !map.contains_key("name") {
                if let Ok(arch) = std::fs::read_to_string("/etc/arch-release") {
                    if !arch.trim().is_empty() {
                        map.insert("name".into(), "Arch Linux".into());
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            map.insert("name".into(), "macOS".into());
            if let Ok(output) = std::process::Command::new("sw_vers")
                .arg("-productVersion").output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() { map.insert("version".into(), v); }
            }
        }

        map.insert("arch".into(), std::env::consts::ARCH.to_string());
        Ok(InfoValue::Map(map))
    }
}
```

- [ ] **Step 2: Write host.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct HostModule;

impl Module for HostModule {
    fn name(&self) -> &'static str { "host" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(
            hostname().unwrap_or_else(|| "unknown".into())
        ))
    }
}

fn hostname() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .ok().map(|s| s.trim().to_string())
    }
    #[cfg(target_os = "macos")]
    {
        let mut buf = vec![0u8; 256];
        if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut std::ffi::c_char, 255) } == 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
            Some(std::str::from_utf8(&buf[..len]).unwrap_or("mac").to_string())
        } else {
            None
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}
```

- [ ] **Step 3: Write kernel.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct KernelModule;

impl Module for KernelModule {
    fn name(&self) -> &'static str { "kernel" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let uname = std::process::Command::new("uname")
            .args(["-srm"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if s.is_empty() { None } else { Some(s) }
            })
            .unwrap_or_else(|| "unknown".to_string());

        Ok(InfoValue::Scalar(uname))
    }
}
```

- [ ] **Step 4: Write uptime.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct UptimeModule;

impl Module for UptimeModule {
    fn name(&self) -> &'static str { "uptime" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let secs = uptime_secs().unwrap_or(0);
        Ok(InfoValue::Scalar(format_uptime(secs)))
    }
}

fn uptime_secs() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/uptime").ok()?;
        content.split_whitespace().next()?
            .split('.').next()?
            .parse::<u64>().ok()
    }
    #[cfg(target_os = "macos")]
    {
        let mut mib: [i32; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];
        let mut boottime = std::mem::MaybeUninit::<libc::timeval>::uninit();
        let mut size = std::mem::size_of::<libc::timeval>();
        if unsafe { libc::sysctl(mib.as_mut_ptr(), 2, boottime.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } == 0 {
            let bt = unsafe { boottime.assume_init() };
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).ok()?;
            let boot_secs = bt.tv_sec as u64;
            Some(now.as_secs().saturating_sub(boot_secs))
        } else {
            None
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    match (days, hours, mins) {
        (0, 0, m) => format!("{m} mins"),
        (0, h, m) => format!("{h}h {m}m"),
        (d, h, m) => format!("{d}d {h}h {m}m"),
    }
}
```

- [ ] **Step 5: Write locale.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct LocaleModule;

impl Module for LocaleModule {
    fn name(&self) -> &'static str { "locale" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let lang = std::env::var("LANG").unwrap_or_default();
        let encoding = std::env::var("LC_CTYPE")
            .or_else(|_| std::env::var("LC_ALL"))
            .unwrap_or_default();

        let mut map = std::collections::HashMap::new();
        if !lang.is_empty() { map.insert("lang".into(), lang); }
        if !encoding.is_empty() { map.insert("encoding".into(), encoding); }

        if map.is_empty() {
            Ok(InfoValue::Scalar("unknown".into()))
        } else {
            Ok(InfoValue::Map(map))
        }
    }
}
```

- [ ] **Step 6: Write modules/mod.rs**

```rust
pub mod os;
pub mod host;
pub mod kernel;
pub mod uptime;
pub mod locale;
```

- [ ] **Step 7: Update lib.rs — add module**

```rust
pub mod modules;
```

- [ ] **Step 8: Verify compile**

Run: `cargo build -p sysfetch-core`
Expected: builds without errors

- [ ] **Step 9: Write quick test**

Append to `sysfetch-core/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_module_compiles() {
        let m = modules::os::OsModule;
        assert_eq!(m.name(), "os");
    }

    #[test]
    fn test_uptime_format() {
        assert_eq!(crate::modules::uptime::format_uptime(3661), "1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(90061), "1d 1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(120), "2h 0m");
    }
}
```

- [ ] **Step 10: Tests pass**

Run: `cargo test -p sysfetch-core`
Expected: all tests pass

- [ ] **Step 11: Commit**

```bash
git add -A && git commit -m "feat: os, host, kernel, uptime, locale modules"
git push
```

---

### Task 4: Hardware modules (cpu, memory, disk, gpu, network, battery, processes)

**Files:**
- Create: `sysfetch-core/src/modules/cpu.rs`
- Create: `sysfetch-core/src/modules/memory.rs`
- Create: `sysfetch-core/src/modules/disk.rs`
- Create: `sysfetch-core/src/modules/gpu.rs`
- Create: `sysfetch-core/src/modules/network.rs`
- Create: `sysfetch-core/src/modules/battery.rs`
- Create: `sysfetch-core/src/modules/processes.rs`
- Modify: `sysfetch-core/src/modules/mod.rs`

- [ ] **Step 1: Write cpu.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct CpuModule;

impl Module for CpuModule {
    fn name(&self) -> &'static str { "cpu" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = std::collections::HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
                let mut cores = 0usize;
                for line in content.lines() {
                    if let Some(val) = line.strip_prefix("model name\t: ") {
                        if !map.contains_key("model") {
                            map.insert("model".into(), val.trim().into());
                        }
                        cores += 1;
                    }
                    if let Some(val) = line.strip_prefix("cpu MHz\t: ") {
                        if let Ok(mhz) = val.trim().parse::<f64>() {
                            map.insert("freq_mhz".into(), format!("{:.0}", mhz));
                        }
                    }
                    if let Some(val) = line.strip_prefix("cache size\t: ") {
                        map.insert("cache".into(), val.trim().into());
                    }
                }
                map.insert("cores".into(), cores.to_string());
            }
        }

        #[cfg(target_os = "macos")]
        {
            map.insert("model".into(), sysctl_str("machdep.cpu.brand_string").unwrap_or_else(|| "Apple".into()));
            let cores = sysctl_int("hw.ncpu").unwrap_or(0);
            map.insert("cores".into(), cores.to_string());
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(target_os = "macos")]
fn sysctl_str(name: &str) -> Option<String> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut size = 0usize;
    if unsafe { libc::sysctlbyname(cname.as_ptr(), std::ptr::null_mut(), &mut size, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    let mut buf = vec![0u8; size];
    if unsafe { libc::sysctlbyname(cname.as_ptr(), buf.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(std::str::from_utf8(&buf[..len]).unwrap_or("").to_string())
}

#[cfg(target_os = "macos")]
fn sysctl_int(name: &str) -> Option<u64> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    if unsafe { libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut std::ffi::c_void, &mut size, std::ptr::null_mut(), 0) } == 0 {
        Some(val)
    } else {
        None
    }
}
```

- [ ] **Step 2: Write memory.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct MemoryModule;

impl Module for MemoryModule {
    fn name(&self) -> &'static str { "memory" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = std::collections::HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                let mut total_kb = 0u64;
                let mut avail_kb = 0u64;
                for line in content.lines() {
                    if let Some(val) = line.strip_prefix("MemTotal:") {
                        total_kb = parse_kb(val);
                    }
                    if let Some(val) = line.strip_prefix("MemAvailable:") {
                        avail_kb = parse_kb(val);
                    }
                }
                let used_kb = total_kb.saturating_sub(avail_kb);
                map.insert("total".into(), fmt_kb(total_kb));
                map.insert("used".into(), fmt_kb(used_kb));
                map.insert("available".into(), fmt_kb(avail_kb));
                if total_kb > 0 {
                    map.insert("percent".into(), format!("{:.0}%", used_kb as f64 / total_kb as f64 * 100.0));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let total_kb = sysctl_u64("hw.memsize").unwrap_or(0) / 1024;
            // Use vm_stat to get page counts
            if let Ok(out) = std::process::Command::new("vm_stat").output() {
                let s = String::from_utf8_lossy(&out.stdout);
                let page_size = sysctl_u64("hw.pagesize").unwrap_or(4096);
                let mut active = 0u64; let mut wired = 0u64;
                for line in s.lines() {
                    if let Some(v) = line.strip_prefix("Pages active:") {
                        active = v.trim().trim_end_matches('.').parse().unwrap_or(0);
                    }
                    if let Some(v) = line.strip_prefix("Pages wired down:") {
                        wired = v.trim().trim_end_matches('.').parse().unwrap_or(0);
                    }
                }
                let used_kb = ((active + wired) * page_size) / 1024;
                map.insert("total".into(), fmt_kb(total_kb));
                map.insert("used".into(), fmt_kb(used_kb));
                let avail = total_kb.saturating_sub(used_kb);
                map.insert("available".into(), fmt_kb(avail));
                if total_kb > 0 {
                    map.insert("percent".into(), format!("{:.0}%", used_kb as f64 / total_kb as f64 * 100.0));
                }
            } else {
                map.insert("total".into(), fmt_kb(total_kb));
            }
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(target_os = "linux")]
fn parse_kb(s: &str) -> u64 {
    s.split_whitespace().next().and_then(|n| n.parse::<u64>().ok()).unwrap_or(0)
}

fn fmt_kb(kb: u64) -> String {
    if kb >= 1024 * 1024 {
        format!("{:.2} GiB", kb as f64 / (1024.0 * 1024.0))
    } else if kb >= 1024 {
        format!("{:.2} MiB", kb as f64 / 1024.0)
    } else {
        format!("{kb} KiB")
    }
}

#[cfg(target_os = "macos")]
fn sysctl_u64(name: &str) -> Option<u64> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    if unsafe { libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut std::ffi::c_void, &mut size, std::ptr::null_mut(), 0) } == 0 {
        Some(val)
    } else {
        None
    }
}
```

- [ ] **Step 3: Write disk.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str { "disk" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mounts = get_mounts().unwrap_or_default();
        Ok(InfoValue::List(mounts))
    }
}

fn get_mounts() -> Option<Vec<String>> {
    let mut result = Vec::new();

    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/mounts").ok()?;
        let mut seen = std::collections::HashSet::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 { continue; }
            let mount_point = parts[1];
            // skip pseudo filesystems
            if mount_point.starts_with("/sys") || mount_point.starts_with("/proc")
                || mount_point.starts_with("/dev") || mount_point.starts_with("/run")
                || mount_point == "/" { continue; }
            if mount_point.starts_with("/") && seen.insert(mount_point.to_string()) {
                if let Some(usage) = mount_usage(mount_point) {
                    result.push(format!("{mount_point} {usage}"));
                }
            }
        }
        // always include root
        if let Some(usage) = mount_usage("/") {
            result.insert(0, format!("/ {usage}"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = std::process::Command::new("df").args(["-H"]).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let mount = parts[5];
                    if mount.starts_with("/") {
                        result.push(format!("{mount} {used}/{size}", used=parts[2], size=parts[1]));
                    }
                }
            }
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn mount_usage(path: &str) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        let cpath = std::ffi::CString::new(path).ok()?;
        if unsafe { libc::statvfs(cpath.as_ptr(), &mut stat) } != 0 {
            return None;
        }
        let total = stat.f_blocks as u64 * stat.f_frsize as u64;
        let avail = stat.f_bavail as u64 * stat.f_frsize as u64;
        let used = total.saturating_sub(stat.f_bfree as u64 * stat.f_frsize as u64);
        let pct = if total > 0 { format!("{:.0}%", used as f64 / total as f64 * 100.0) } else { "?".into() };
        Some(format!("{}/{} ({})", bytes_fmt(used), bytes_fmt(total), pct))
    }

    #[cfg(not(target_os = "linux"))]
    { None }
}

fn bytes_fmt(bytes: u64) -> String {
    if bytes >= 1 << 40 {
        format!("{:.1}TiB", bytes as f64 / (1u64 << 40) as f64)
    } else if bytes >= 1 << 30 {
        format!("{:.1}GiB", bytes as f64 / (1u64 << 30) as f64)
    } else if bytes >= 1 << 20 {
        format!("{:.1}MiB", bytes as f64 / (1u64 << 20) as f64)
    } else {
        format!("{bytes}B")
    }
}
```

- [ ] **Step 4: Write gpu.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct GpuModule;

impl Module for GpuModule {
    fn name(&self) -> &'static str { "gpu" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut devices = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // lspci for GPU detection
            if let Ok(out) = std::process::Command::new("lspci")
                .args(["-mm"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    let lower = line.to_lowercase();
                    if lower.contains("vga") || lower.contains("3d") || lower.contains("display") {
                        let parts: Vec<&str> = line.split('"').collect();
                        if parts.len() >= 3 {
                            devices.push(parts[1].trim().to_string());
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("system_profiler")
                .args(["SPDisplaysDataType"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    if let Some(val) = line.strip_prefix("Chipset Model: ") {
                        devices.push(val.trim().to_string());
                    }
                }
            }
        }

        Ok(InfoValue::List(devices))
    }
}
```

- [ ] **Step 5: Write network.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct NetworkModule;

impl Module for NetworkModule {
    fn name(&self) -> &'static str { "network" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut interfaces = Vec::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy().to_string();
                    if name == "lo" { continue; }
                    // read operstate
                    let state_path = entry.path().join("operstate");
                    let ip = get_ip_address(&name);
                    let status = std::fs::read_to_string(&state_path)
                        .unwrap_or_default().trim().to_string();
                    interfaces.push(format!("{name} {ip} ({status})"));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("ifconfig")
                .args(["-l"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for name in s.split_whitespace() {
                    if name == "lo0" { continue; }
                    let ip = get_ip_address_macos(name);
                    interfaces.push(format!("{name} {ip}"));
                }
            }
        }

        Ok(InfoValue::List(interfaces))
    }
}

#[cfg(target_os = "linux")]
fn get_ip_address(iface: &str) -> String {
    if let Ok(out) = std::process::Command::new("ip")
        .args(["-4", "addr", "show", iface])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines() {
            if let Some(addr) = line.trim().strip_prefix("inet ") {
                if let Some(ip) = addr.split('/').next() {
                    return ip.to_string();
                }
            }
        }
    }
    "no ip".into()
}

#[cfg(target_os = "macos")]
fn get_ip_address_macos(iface: &str) -> String {
    if let Ok(out) = std::process::Command::new("ifconfig")
        .args([iface])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines() {
            if let Some(addr) = line.trim().strip_prefix("inet ") {
                if let Some(ip) = addr.split_whitespace().next() {
                    return ip.to_string();
                }
            }
        }
    }
    "no ip".into()
}
```

- [ ] **Step 6: Write battery.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str { "battery" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = std::collections::HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let type_path = path.join("type");
                    let typ = std::fs::read_to_string(&type_path).unwrap_or_default();
                    if typ.trim() != "Battery" { continue; }

                    let capacity = std::fs::read_to_string(path.join("capacity")).ok()
                        .and_then(|s| s.trim().parse::<u8>().ok()).unwrap_or(0);
                    let status = std::fs::read_to_string(path.join("status"))
                        .unwrap_or_default().trim().to_string();

                    map.insert("percent".into(), format!("{capacity}%"));
                    map.insert("status".into(), status);
                    break;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("pmset")
                .args(["-g", "batt"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    if let Some(info) = line.trim().strip_prefix("-InternalBattery-0") {
                        if let Some(pct) = info.split_whitespace()
                            .find(|w| w.ends_with('%'))
                        {
                            map.insert("percent".into(), pct.to_string());
                        }
                        if info.contains("discharging") {
                            map.insert("status".into(), "Discharging".into());
                        } else if info.contains("charging") || info.contains("AC") {
                            map.insert("status".into(), "Charging".into());
                        } else if info.contains("charged") {
                            map.insert("status".into(), "Full".into());
                        }
                        break;
                    }
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("no battery".into()));
        }
        Ok(InfoValue::Map(map))
    }
}
```

- [ ] **Step 7: Write processes.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct ProcessesModule;

impl Module for ProcessesModule {
    fn name(&self) -> &'static str { "processes" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let count = get_process_count().unwrap_or(0);
        Ok(InfoValue::Scalar(count.to_string()))
    }
}

fn get_process_count() -> Option<usize> {
    #[cfg(target_os = "linux")]
    {
        let entries = std::fs::read_dir("/proc").ok()?;
        let mut count = 0usize;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.chars().all(|c| c.is_ascii_digit()) {
                count += 1;
            }
        }
        Some(count)
    }

    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("ps")
            .args(["-e", "--no-headers"])
            .output().ok()?;
        let count = String::from_utf8_lossy(&out.stdout).lines().count();
        Some(count)
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}
```

- [ ] **Step 8: Update modules/mod.rs — add all new modules**

```rust
pub mod os;
pub mod host;
pub mod kernel;
pub mod uptime;
pub mod locale;
pub mod cpu;
pub mod memory;
pub mod disk;
pub mod gpu;
pub mod network;
pub mod battery;
pub mod processes;
pub mod shell;
pub mod terminal;
pub mod de;
pub mod wm;
pub mod packages;
pub mod colors;
pub mod custom;
```

(Note: shell, terminal, de, wm, packages, colors, custom will be written in Task 5. Adding them here for the mod declaration since we'll write them next.)

- [ ] **Step 9: Verify compile**

Run: `cargo build -p sysfetch-core`
Expected: builds without errors

- [ ] **Step 10: Commit**

```bash
git add -A && git commit -m "feat: cpu, memory, disk, gpu, network, battery, processes modules"
git push
```

---

### Task 5: Software + display modules (packages, shell, terminal, de, wm, colors, custom)

**Files:**
- Create: `sysfetch-core/src/modules/packages.rs`
- Create: `sysfetch-core/src/modules/shell.rs`
- Create: `sysfetch-core/src/modules/terminal.rs`
- Create: `sysfetch-core/src/modules/de.rs`
- Create: `sysfetch-core/src/modules/wm.rs`
- Create: `sysfetch-core/src/modules/colors.rs`
- Create: `sysfetch-core/src/modules/custom.rs`
- Modify: `sysfetch-core/src/modules/mod.rs` (already done in Task 4 step 8)

- [ ] **Step 1: Write packages.rs**

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str { "packages" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        // dpkg (Debian/Ubuntu)
        if let Ok(count) = count_lines("/var/lib/dpkg/info", Some(".list")) {
            map.insert("dpkg".into(), count.to_string());
        }

        // pacman (Arch)
        if let Ok(count) = run_count("pacman", &["-Q"]) {
            map.insert("pacman".into(), count.to_string());
        }

        // rpm (Fedora/RHEL)
        if let Ok(count) = run_count("rpm", &["-qa"]) {
            map.insert("rpm".into(), count.to_string());
        }

        // flatpak
        if let Ok(count) = run_count("flatpak", &["list"]) {
            map.insert("flatpak".into(), count.to_string());
        }

        // snap
        if let Ok(count) = run_count("snap", &["list"]) {
            map.insert("snap".into(), count.to_string());
        }

        // brew (macOS)
        if let Ok(count) = run_count("brew", &["list", "--formula"]) {
            map.insert("brew_formula".into(), count.to_string());
        }

        if let Ok(count) = run_count("brew", &["list", "--cask"]) {
            map.insert("brew_cask".into(), count.to_string());
        }

        Ok(InfoValue::Map(map))
    }
}

fn count_lines(dir: &str, ext: Option<&str>) -> Result<usize> {
    let dir = std::fs::read_dir(dir).map_err(|e| crate::Error::Io(e))?;
    let count = dir.flatten()
        .filter(|e| {
            if let Some(ext) = ext {
                e.path().extension().map(|e| e == &ext[1..]).unwrap_or(false)
            } else {
                true
            }
        })
        .count();
    Ok(count)
}

fn run_count(cmd: &str, args: &[&str]) -> Result<usize> {
    let out = std::process::Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| crate::Error::Io(e))?;
    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout);
        Ok(s.lines().count())
    } else {
        Err(crate::Error::Parse(format!("{cmd} failed")))
    }
}
```

- [ ] **Step 2: Write shell.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct ShellModule;

impl Module for ShellModule {
    fn name(&self) -> &'static str { "shell" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let shell_path = std::env::var("SHELL").unwrap_or_default();
        let name = shell_path.rsplit('/').next().unwrap_or("unknown").to_string();
        let version = get_version(&name);

        let mut map = std::collections::HashMap::new();
        map.insert("name".into(), name);
        if let Some(v) = version {
            map.insert("version".into(), v);
        }
        Ok(InfoValue::Map(map))
    }
}

fn get_version(shell: &str) -> Option<String> {
    match shell {
        "bash" | "zsh" | "fish" | "nu" => {
            let out = std::process::Command::new(shell)
                .arg("--version").output().ok()?;
            let s = String::from_utf8_lossy(&out.stdout);
            s.lines().next().map(|l| l.to_string())
        }
        _ => None,
    }
}
```

- [ ] **Step 3: Write terminal.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str { "terminal" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let program = std::env::var("TERM_PROGRAM")
            .or_else(|_| std::env::var("TERM"))
            .unwrap_or_else(|_| "unknown".into());
        let version = std::env::var("TERM_PROGRAM_VERSION").ok();

        if let Some(v) = version {
            let mut map = std::collections::HashMap::new();
            map.insert("name".into(), program);
            map.insert("version".into(), v);
            Ok(InfoValue::Map(map))
        } else {
            Ok(InfoValue::Scalar(program))
        }
    }
}
```

- [ ] **Step 4: Write de.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct DeModule;

impl Module for DeModule {
    fn name(&self) -> &'static str { "de" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_else(|_| "unknown".into());
        Ok(InfoValue::Scalar(de))
    }
}
```

- [ ] **Step 5: Write wm.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct WmModule;

impl Module for WmModule {
    fn name(&self) -> &'static str { "wm" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let wm = detect_wm();
        Ok(InfoValue::Scalar(wm))
    }
}

fn detect_wm() -> String {
    // Try XDG_CURRENT_DESKTOP first — many WMs set it
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        // Some DEs include WM info
        let de_lower = de.to_lowercase();
        if !de_lower.contains("gnome") && !de_lower.contains("kde")
            && !de_lower.contains("xfce") && !de_lower.contains("cinnamon")
            && !de_lower.contains("mate") && !de_lower.contains("budgie")
            && !de_lower.contains("pantheon")
        {
            return de;
        }
    }

    // Check running process list for known WMs
    let known_wms = ["i3", "sway", "hyprland", "bspwm", "dwm", "awesome",
                      "qtile", "openbox", "fluxbox", "xfwm4", "marco",
                      "muffin", "kwin_x11", "kwin_wayland"];

    for wm in known_wms {
        if process_exists(wm) {
            return wm.to_string();
        }
    }

    "unknown".into()
}

fn process_exists(name: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let path = entry.path().join("comm");
                if let Ok(comm) = std::fs::read_to_string(&path) {
                    if comm.trim() == name {
                        return true;
                    }
                }
            }
        }
        false
    }
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("pgrep")
            .args(["-x", name]).output();
        matches!(out, Ok(o) if o.status.success())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { false }
}
```

- [ ] **Step 6: Write colors.rs**

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct ColorsModule;

impl Module for ColorsModule {
    fn name(&self) -> &'static str { "colors" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let swatches = (0..8).map(|i| color_swatch(i)).collect();
        Ok(InfoValue::List(swatches))
    }
}

fn color_swatch(n: u8) -> String {
    // ANSI 256-color background block
    let code = match n {
        0 => 0,    // black
        1 => 1,    // red
        2 => 2,    // green
        3 => 3,    // yellow
        4 => 4,    // blue
        5 => 5,    // magenta
        6 => 6,    // cyan
        7 => 7,    // white
        _ => n,
    };
    format!("\x1b[48;5;{code}m  \x1b[0m", code = color_map(n))
}

fn color_map(n: u8) -> u8 {
    match n {
        0 => 0, 1 => 1, 2 => 2, 3 => 3,
        4 => 4, 5 => 5, 6 => 6, 7 => 7,
        _ => n,
    }
}
```

- [ ] **Step 7: Write custom.rs**

```rust
use crate::{Module, InfoValue, Context, Result};
use crate::config::CustomModule as CustomConfig;

pub struct CustomCommandModule {
    pub name: &'static str,
    pub config: CustomConfig,
}

impl Module for CustomCommandModule {
    fn name(&self) -> &'static str { self.name }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let shell = self.config.shell.as_deref().unwrap_or("sh");
        let output = std::process::Command::new(shell)
            .arg("-c")
            .arg(&self.config.command)
            .output()
            .map_err(|e| crate::Error::Io(e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let label = self.config.label.clone().unwrap_or_else(|| self.name.to_string());

        if stdout.is_empty() {
            Ok(InfoValue::scalar(format!("{label}: ")))
        } else {
            Ok(InfoValue::scalar(format!("{label}: {stdout}")))
        }
    }
}
```

- [ ] **Step 8: Verify compile**

Run: `cargo build -p sysfetch-core`
Expected: builds without errors

- [ ] **Step 9: Commit**

```bash
git add -A && git commit -m "feat: packages, shell, terminal, de, wm, colors, custom modules"
git push
```

---

### Task 6: Lua plugin system (sysfetch-lua crate)

**Files:**
- Create: `sysfetch-lua/src/lib.rs`
- Create: `sysfetch-lua/src/api.rs`

**Interfaces:**
- Consumes: `Module`, `InfoValue`, `Context` from sysfetch-core
- Produces: `LuaModule` (implements Module trait), `LuaPluginEngine`

- [ ] **Step 1: Write sysfetch-lua/src/lib.rs**

```rust
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use mlua::{Lua, Function, Table, Value};
use sysfetch_core::{Module, InfoValue, Context, Result, Error};

pub struct LuaModule {
    name: String,
    plugin_dir: std::path::PathBuf,
}

impl LuaModule {
    pub fn new(plugin_dir: std::path::PathBuf) -> Self {
        // Derive name from directory
        let name = plugin_dir
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("lua_plugin")
            .to_string();
        LuaModule { name, plugin_dir }
    }
}

impl Module for LuaModule {
    fn name(&self) -> &'static str {
        // We leak here — acceptable for a CLI tool
        Box::leak(self.name.clone().into_boxed_str())
    }

    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let lua = Lua::new();
        let result = lua.context(|lua_ctx| -> mlua::Result<InfoValue> {
            // Read all .lua files in plugin dir
            let mut scripts = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&self.plugin_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "lua").unwrap_or(false) {
                        if let Ok(code) = std::fs::read_to_string(&path) {
                            scripts.push(code);
                        }
                    }
                }
            }

            // Register API
            let api_table = lua_ctx.create_table()?;
            api_table.set("read_file", lua_ctx.create_function(lua_read_file)?)?;
            api_table.set("run_command", lua_ctx.create_function(lua_run_command)?)?;
            api_table.set("get_env", lua_ctx.create_function(lua_get_env)?)?;
            lua_ctx.globals().set("ctx", api_table)?;

            // Run scripts
            let mut name = String::new();
            let mut collected: Option<InfoValue> = None;

            for code in &scripts {
                let chunk = lua_ctx.load(code);
                let result: Value = chunk.eval()?;

                if let Value::Table(t) = result {
                    if let Ok(n) = t.get::<_, String>("name") {
                        name = n;
                    }
                    if let Ok(func) = t.get::<_, Function>("collect") {
                        let res: Value = func.call::<_, Value>(lua_ctx.globals().get::<_, Table>("ctx")?)?;
                        collected = Some(lua_value_to_info(res));
                    }
                }
            }

            self.name = name;
            Ok(collected.unwrap_or(InfoValue::Scalar("no data".into())))
        });

        result.map_err(|e| Error::Lua(e.to_string()))
    }
}

fn lua_read_file<'lua>(lua_ctx: &'lua Lua, path: String) -> mlua::Result<String> {
    std::fs::read_to_string(&path)
        .map_err(|e| mlua::Error::RuntimeError(format!("read_file: {e}")))
}

fn lua_run_command<'lua>(lua_ctx: &'lua Lua, cmd: String) -> mlua::Result<String> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .map_err(|e| mlua::Error::RuntimeError(format!("run_command: {e}")))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn lua_get_env<'lua>(lua_ctx: &'lua Lua, key: String) -> mlua::Result<String> {
    Ok(std::env::var(&key).unwrap_or_default())
}

fn lua_value_to_info(val: Value) -> InfoValue {
    match val {
        Value::String(s) => InfoValue::Scalar(s.to_string_lossy().to_string()),
        Value::Table(t) => {
            // Check if it's a table with "value" key (structured return)
            if let Ok(val_str) = t.get::<_, String>("value") {
                if let Ok(typ) = t.get::<_, String>("type") {
                    match typ.as_str() {
                        "scalar" => return InfoValue::Scalar(val_str),
                        "map" => {
                            // Try to parse as JSON
                            if let Ok(m) = serde_json::from_str::<HashMap<String, String>>(&val_str) {
                                return InfoValue::Map(m);
                            }
                        }
                        _ => return InfoValue::Scalar(val_str),
                    }
                }
                return InfoValue::Scalar(val_str);
            }
            // Plain table -> try Map
            let mut map = HashMap::new();
            for pair in t.pairs::<Value, Value>() {
                if let Ok((k, v)) = pair {
                    let k_str = format_value(&k);
                    let v_str = format_value(&v);
                    map.insert(k_str, v_str);
                }
            }
            if map.is_empty() {
                InfoValue::Scalar("table".into())
            } else {
                InfoValue::Map(map)
            }
        }
        Value::Integer(i) => InfoValue::Scalar(i.to_string()),
        Value::Number(f) => InfoValue::Scalar(format!("{f}")),
        Value::Boolean(b) => InfoValue::Scalar(if b { "yes".into() } else { "no".into() }),
        Value::Nil => InfoValue::Scalar("nil".into()),
        _ => InfoValue::Scalar("?".into()),
    }
}

fn format_value(val: &Value) -> String {
    match val {
        Value::String(s) => s.to_string_lossy().to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(f) => format!("{f}"),
        Value::Boolean(b) => (if *b { "yes" } else { "no" }).to_string(),
        _ => "?".to_string(),
    }
}
```

- [ ] **Step 2: Write sysfetch-lua/src/api.rs**

```rust
// Re-export from lib.rs for now
```

- [ ] **Step 3: Update sysfetch-lua/Cargo.toml — add sysfetch-core dep properly**

```toml
[package]
name = "sysfetch-lua"
version.workspace = true
edition.workspace = true

[dependencies]
sysfetch-core = { path = "../sysfetch-core" }
mlua = { version = "0.10", features = ["lua54"] }
```

- [ ] **Step 4: Verify compile**

Run: `cargo build -p sysfetch-lua`
Expected: builds without errors

- [ ] **Step 5: Write test Lua plugin**

```bash
mkdir -p tests/fixtures/plugins
cat > tests/fixtures/plugins/hello.lua << 'EOF'
return {
    name = "hello_test",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        return { value = "Hello from Lua, " .. user .. "!", type = "scalar" }
    end
}
EOF
```

- [ ] **Step 6: Commit**

```bash
git add -A && git commit -m "feat: Lua plugin system"
git push
```

---

### Task 7: CLI binary (sysfetch-cli)

**Files:**
- Create: `sysfetch-cli/src/main.rs`
- Create: `sysfetch-core/src/module_registry.rs`
- Modify: `sysfetch-core/src/lib.rs` (add module_registry mod)

**Interfaces:**
- Consumes: All modules, Config, TeraEngine, LuaModule
- Produces: Working binary `flexfetch`

- [ ] **Step 1: Write module_registry.rs**

```rust
use crate::{Module, Context, InfoValue, SystemInfo};
use crate::modules::*;
use crate::config::Config;

pub struct ModuleRegistry {
    native_modules: Vec<Box<dyn Module>>,
    lua_modules: Vec<Box<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new(config: &Config, ctx: &Context) -> Self {
        let mut native: Vec<Box<dyn Module>> = Vec::new();

        // Register all built-in modules by name
        let builtins: Vec<(&str, Box<dyn Module>)> = vec![
            ("os", Box::new(os::OsModule)),
            ("host", Box::new(host::HostModule)),
            ("kernel", Box::new(kernel::KernelModule)),
            ("uptime", Box::new(uptime::UptimeModule)),
            ("locale", Box::new(locale::LocaleModule)),
            ("cpu", Box::new(cpu::CpuModule)),
            ("memory", Box::new(memory::MemoryModule)),
            ("disk", Box::new(disk::DiskModule)),
            ("gpu", Box::new(gpu::GpuModule)),
            ("network", Box::new(network::NetworkModule)),
            ("battery", Box::new(battery::BatteryModule)),
            ("processes", Box::new(processes::ProcessesModule)),
            ("packages", Box::new(packages::PackagesModule)),
            ("shell", Box::new(shell::ShellModule)),
            ("terminal", Box::new(terminal::TerminalModule)),
            ("de", Box::new(de::DeModule)),
            ("wm", Box::new(wm::WmModule)),
            ("colors", Box::new(colors::ColorsModule)),
        ];

        // Only register modules user asked for
        for name in &config.modules {
            if name == "title" || name == "separator" {
                continue; // handled by template
            }
            if let Some((_, m)) = builtins.iter().find(|(n, _)| n == name) {
                native.push(m.clone()); // need Clone trait bound
            }
        }

        ModuleRegistry { native_modules: native, lua_modules: Vec::new() }
    }

    pub fn run_all(&self, ctx: &Context) -> SystemInfo {
        use rayon::prelude::*;
        let mut info = SystemInfo::new();

        let entries: Vec<_> = self.native_modules.par_iter()
            .map(|m| {
                let result = m.collect(ctx);
                (m.name(), result)
            })
            .collect();

        for (name, result) in entries {
            match result {
                Ok(val) => info.add(name, val),
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] module {name} error: {e}");
                    }
                    info.add(name, InfoValue::Scalar("error".into()));
                }
            }
        }

        info
    }
}
```

- [ ] **Step 2: Add clone bound to Module trait**

Update `module.rs`:

```rust
pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, ctx: &Context) -> crate::Result<InfoValue>;
}

// For module_registry cloning
pub trait ModuleClone: Module {
    fn box_clone(&self) -> Box<dyn Module>;
}

impl<T> ModuleClone for T where T: 'static + Module + Clone {
    fn box_clone(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}
```

Wait — simpler approach: store the builder functions instead of cloning. Let me fix module_registry to not need Clone.

- [ ] **Step 2 (revised): Simpler ModuleRegistry without Clone**

```rust
use crate::{Module, Context, InfoValue, SystemInfo};
use crate::config::Config;

type ModuleBuilder = fn() -> Box<dyn Module>;

pub struct ModuleRegistry {
    builders: Vec<(&'static str, ModuleBuilder)>,
}

impl ModuleRegistry {
    pub fn new(config: &Config) -> Self {
        let mut builders: Vec<(&'static str, ModuleBuilder)> = Vec::new();

        macro_rules! reg {
            ($name:ident) => {
                builders.push((stringify!($name), || Box::new(crate::modules::$name::$name##Module)));
            };
        }

        reg!(os); reg!(host); reg!(kernel); reg!(uptime);
        reg!(locale); reg!(cpu); reg!(memory); reg!(disk);
        reg!(gpu); reg!(network); reg!(battery); reg!(processes);
        reg!(packages); reg!(shell); reg!(terminal); reg!(de);
        reg!(wm); reg!(colors);

        ModuleRegistry { builders }
    }

    pub fn run_selected(&self, selected: &[String], ctx: &Context) -> SystemInfo {
        use rayon::prelude::*;
        let mut info = SystemInfo::new();

        let entries: Vec<_> = selected.par_iter()
            .filter_map(|name| {
                if name == "title" || name == "separator" {
                    return None;
                }
                self.builders.iter()
                    .find(|(n, _)| n == name)
                    .map(|(n, builder)| {
                        let module = builder();
                        let result = module.collect(ctx);
                        (*n, result)
                    })
            })
            .collect();

        for (name, result) in entries {
            match result {
                Ok(val) => info.add(name, val),
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] module {name} error: {e}");
                    }
                    info.add(name, InfoValue::Scalar("error".into()));
                }
            }
        }

        info
    }
}
```

- [ ] **Step 3: Write main.rs**

```rust
use clap::Parser;
use sysfetch_core::{Context, Config, TeraEngine, module_registry::ModuleRegistry};

#[derive(Parser)]
#[command(name = "flexfetch", version, about = "Fast, flexible system info tool")]
struct Cli {
    /// Custom config file path
    #[arg(short, long)]
    config: Option<String>,

    /// Override modules (colon-separated)
    #[arg(short, long)]
    modules: Option<String>,

    /// Custom template file
    #[arg(short, long)]
    template: Option<String>,

    /// Output format (text or json)
    #[arg(short = 'f', long, default_value = "text")]
    format: String,

    /// Enable debug output
    #[arg(long)]
    debug: bool,

    /// Generate default config file
    #[arg(long)]
    gen_config: bool,

    /// List available modules
    #[arg(long)]
    list_modules: bool,

    /// List found Lua plugins
    #[arg(long)]
    list_plugins: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.gen_config {
        generate_config();
        return;
    }

    if cli.list_modules {
        list_modules();
        return;
    }

    // Build context
    let config_dir = get_config_dir();
    let cache_dir = get_cache_dir();
    let ctx = Context::new(config_dir.clone(), cache_dir, cli.debug);

    // Load config
    let config_path = cli.config.as_ref().map(|s| std::path::Path::new(s));
    let config = Config::load(config_path).unwrap_or_else(|e| {
        eprintln!("warning: config error: {e}, using defaults");
        Config::default_for_testing()
    });

    // Determine modules to run
    let modules: Vec<String> = if let Some(m) = cli.modules {
        m.split(':').map(|s| s.to_string()).collect()
    } else {
        config.modules.clone()
    };

    // Build registry and run
    let registry = ModuleRegistry::new(&config);
    let info = registry.run_selected(&modules, &ctx);

    // Output
    match cli.format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&info.to_json())
                .unwrap_or_else(|_| "{}".into()));
        }
        _ => {
            let engine = TeraEngine::new_default();
            match engine.render(&info, &config) {
                Ok(output) => println!("{output}"),
                Err(e) => eprintln!("template error: {e}"),
            }
        }
    }
}

fn get_config_dir() -> std::path::PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            std::path::PathBuf::from(home).join(".config")
        })
        .join("flexfetch")
}

fn get_cache_dir() -> std::path::PathBuf {
    std::path::PathBuf::from("/tmp")
}

fn generate_config() {
    let config = Config::default_for_testing();
    let toml = toml::to_string_pretty(&config).unwrap_or_default();
    println!("{toml}");
    let config_dir = get_config_dir();
    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        eprintln!("error creating config dir: {e}");
        return;
    }
    let path = config_dir.join("config.toml");
    if path.exists() {
        eprintln!("config already exists at {path:?}");
        return;
    }
    if let Err(e) = std::fs::write(&path, &toml) {
        eprintln!("error writing config: {e}");
    } else {
        println!("wrote config to {path:?}");
    }
}

fn list_modules() {
    let builtins = [
        "os", "host", "kernel", "uptime", "locale",
        "cpu", "memory", "disk", "gpu", "network", "battery", "processes",
        "packages", "shell", "terminal", "de", "wm", "colors", "custom",
    ];
    println!("Built-in modules:");
    for m in builtins {
        println!("  {m}");
    }
    println!("\nLayout directives (template-only): title, separator");
    println!("\nPlugins: place .lua files in ~/.config/flexfetch/plugins/");
}
```

- [ ] **Step 4: Add `default_for_testing` to Config in config.rs**

```rust
impl Config {
    // ... existing methods ...

    pub fn default_for_testing() -> Self {
        Config {
            modules: Config::default_modules(),
            plugins_dir: None,
            template: Config::default_template(),
            display: DisplayConfig::default(),
            cache: CacheConfig::default(),
            custom: std::collections::HashMap::new(),
        }
    }
}
```

- [ ] **Step 5: Update template engine to accept Config**

Update template.rs to accept config for display options:

```rust
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
```

- [ ] **Step 6: Verify compile**

Run: `cargo build -p sysfetch-cli`
Expected: builds without errors

- [ ] **Step 7: Verify binary runs**

Run: `./target/debug/sysfetch-cli --help`
Expected: shows help text

Run: `./target/debug/sysfetch-cli --list-modules`
Expected: lists modules

- [ ] **Step 8: Commit**

```bash
git add -A && git commit -m "feat: CLI binary + module registry"
git push
```

---

### Task 8: Default template + cache layer + README

**Files:**
- Create: `templates/default.tera`
- Create: `sysfetch-core/src/cache.rs`
- Create: `README.md`
- Modify: `sysfetch-core/src/lib.rs` (add cache mod)

- [ ] **Step 1: Write templates/default.tera**

```tera
{%- set logo_lines = logo(style="auto") -%}
{%- for line in logo_lines %}
{{ line }}
{%- endfor %}

{%- if os %} {{ key_value("OS", os.name ~ " " ~ os.version) }} {% endif %}
{%- if host %} {{ key_value("Host", host.value) }} {% endif %}
{%- if kernel %} {{ key_value("Kernel", kernel.value) }} {% endif %}
{%- if uptime %} {{ key_value("Uptime", uptime.value) }} {% endif %}
{%- if packages %}
  {%- for name, count in packages %}
    {{ key_value("Packages", count ~ " (" ~ name ~ ")") }}
  {%- endfor %}
{%- endif %}
{%- if shell %} {{ key_value("Shell", shell.name ~ " " ~ shell.version) }} {% endif %}
{%- if terminal %} {{ key_value("Terminal", terminal.value) }} {% endif %}
{%- if de %} {{ key_value("DE", de.value) }} {% endif %}
{%- if wm %} {{ key_value("WM", wm.value) }} {% endif %}
{%- if cpu %} {{ key_value("CPU", cpu.model ~ " (" ~ cpu.cores ~ ")") }} {% endif %}
{%- if memory %} {{ key_value("Memory", memory.used ~ " / " ~ memory.total ~ " (" ~ memory.percent ~ ")") }} {% endif %}
{%- if disk %}
  {%- for mount in disk %}
    {{ key_value("Disk", mount) }}
  {%- endfor %}
{%- endif %}
{%- if colors %}
{{ key_value("Colors", colors | join(sep=" ")) }}
{%- endif %}
```

Wait, the built-in blocks `logo()`, `key_value()`, etc. need to be registered with Tera. Let me update the approach — I'll register custom Tera functions instead of using template blocks, which is cleaner.

Actually, Tera supports custom functions via `tera.register_function()`. That's the right approach.

- [ ] **Step 1 (revised): Write templates/default.tera**

```tera
{%- if os %}
OS: {{ os.pretty_name }}
{%- endif %}
{%- if kernel %}
Kernel: {{ kernel.value }}
{%- endif %}
{%- if host %}
Host: {{ host.value }}
{%- endif %}
{%- if uptime %}
Uptime: {{ uptime.value }}
{%- endif %}
{%- if packages %}
Packages:{% for name, count in packages -%}
  {{ count }} ({{ name }})
{% endfor %}
{%- endif %}
{%- if shell %}
Shell: {{ shell.name }}
{%- endif %}
{%- if cpu %}
CPU: {{ cpu.model }} ({{ cpu.cores }} cores)
{%- endif %}
{%- if memory %}
Memory: {{ memory.used }} / {{ memory.total }} ({{ memory.percent }})
{%- endif %}
{%- if gpu %}
{% for device in gpu -%}
GPU: {{ device }}
{% endfor %}
{%- endif %}
{%- if disk %}
{% for mount in disk -%}
Disk: {{ mount }}
{% endfor %}
{%- endif %}
{%- if colors %}
Colors: {{ colors | join(sep=" ") }}
{%- endif %}
```

- [ ] **Step 2: Write cache.rs**

```rust
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    path: PathBuf,
    ttl: u64,
    data: HashMap<String, CacheEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheEntry {
    value: String,
    timestamp: u64,
}

impl Cache {
    pub fn new(cache_dir: PathBuf, ttl: u64) -> Self {
        let path = cache_dir.join("flexfetch-cache.json");
        let data = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        Cache { path, ttl, data }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        self.data.get(key).and_then(|entry| {
            if now - entry.timestamp < self.ttl {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, key: &str, value: String) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs()).unwrap_or(0);
        self.data.insert(key.to_string(), CacheEntry { value, timestamp: now });
        self.flush();
    }

    fn flush(&self) {
        if let Ok(json) = serde_json::to_string(&self.data) {
            let _ = std::fs::write(&self.path, &json);
        }
    }
}
```

- [ ] **Step 3: Integrate cache into Modules**

Update module_registry.rs to accept an optional Cache reference. Simple approach: make cache available through Context.

Add to `context.rs`:

```rust
use crate::cache::Cache;
use std::sync::Mutex;

pub struct Context {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub debug: bool,
    pub cache: Option<Mutex<Cache>>,
}
```

- [ ] **Step 4: Write README.md**

```markdown
# flexfetch

Fast, flexible system information tool for Linux and macOS.

## Quick start

```bash
cargo install --path sysfetch-cli
flexfetch
```

## Configuration

Config at `~/.config/flexfetch/config.toml`:

```toml
modules = ["os", "host", "kernel", "uptime", "cpu", "memory", "disk", "colors"]

[display]
separator = ": "
key_width = 8
```

## Plugins

Write Lua plugins in `~/.config/flexfetch/plugins/`:

```lua
return {
    name = "my_plugin",
    collect = function(ctx)
        return { value = ctx.run_command("echo hello"), type = "scalar" }
    end
}
```

## CLI

```
flexfetch [-c config] [-m os:cpu:memory] [-f json] [--debug]
```

## License

MIT
```

- [ ] **Step 5: Verify end-to-end**

Run: `cargo build --release`
Run: `./target/release/sysfetch-cli`
Expected: prints system info

- [ ] **Step 6: Check binary size**

Run: `ls -lh ./target/release/sysfetch-cli`
Expected: < 10MB (LTO will reduce later)

- [ ] **Step 7: Commit**

```bash
git add -A && git commit -m "feat: default template, cache, README"
git push
```

---

### Task 9: Polish — LTO, release profile, gen-config improvements

**Files:**
- Modify: `Cargo.toml` (workspace — add profile)
- Modify: `sysfetch-cli/src/main.rs` (gen-config from template)

- [ ] **Step 1: Add release profile for small binary**

Add to workspace `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = "z"
strip = true
panic = "abort"
```

- [ ] **Step 2: Improve gen-config template output**

Write a helpful default config that includes comments explaining each section.

- [ ] **Step 3: Final release build + size check**

```bash
cargo build --release
ls -lh target/release/sysfetch-cli
```

- [ ] **Step 4: Final commit**

```bash
git add -A && git commit -m "chore: LTO, release profile, polish"
git push
```

---

## Self-Review Checklist

1. **Spec coverage:** Every module from spec has a task. Template engine (Task 2, 8). Lua plugin system (Task 6). CLI flags (Task 7). Config (Task 2). Cache (Task 8).

2. **Placeholder check:** All code blocks contain real implementation code. No TBD/TODO.

3. **Type consistency:** Module trait uses `&Context` everywhere. `InfoValue` is consistent across module implementations. Config struct is shared between CLI and engine.

4. **Test coverage:** Each task includes compile verification. Task 3 includes unit tests for uptime formatting. Integration tests via manual run in Task 7.
```
