# Task 5: Software + display modules (packages, shell, terminal, de, wm, colors, custom)

**Files:**
- Create: `sysfetch-core/src/modules/packages.rs`
- Create: `sysfetch-core/src/modules/shell.rs`
- Create: `sysfetch-core/src/modules/terminal.rs`
- Create: `sysfetch-core/src/modules/de.rs`
- Create: `sysfetch-core/src/modules/wm.rs`
- Create: `sysfetch-core/src/modules/colors.rs`
- Create: `sysfetch-core/src/modules/custom.rs`

Note: modules/mod.rs already declares these modules. Only create individual files.

## packages.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str { "packages" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        if let Ok(count) = count_lines("/var/lib/dpkg/info", Some(".list")) {
            map.insert("dpkg".into(), count.to_string());
        }
        if let Ok(count) = run_count("pacman", &["-Q"]) {
            map.insert("pacman".into(), count.to_string());
        }
        if let Ok(count) = run_count("rpm", &["-qa"]) {
            map.insert("rpm".into(), count.to_string());
        }
        if let Ok(count) = run_count("flatpak", &["list"]) {
            map.insert("flatpak".into(), count.to_string());
        }
        if let Ok(count) = run_count("snap", &["list"]) {
            map.insert("snap".into(), count.to_string());
        }
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

## shell.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct ShellModule;

impl Module for ShellModule {
    fn name(&self) -> &'static str { "shell" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let shell_path = std::env::var("SHELL").unwrap_or_default();
        let name = shell_path.rsplit('/').next().unwrap_or("unknown").to_string();
        let version = get_version(&name);

        let mut map = HashMap::new();
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

## terminal.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str { "terminal" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let program = std::env::var("TERM_PROGRAM")
            .or_else(|_| std::env::var("TERM"))
            .unwrap_or_else(|_| "unknown".into());
        let version = std::env::var("TERM_PROGRAM_VERSION").ok();

        if let Some(v) = version {
            let mut map = HashMap::new();
            map.insert("name".into(), program);
            map.insert("version".into(), v);
            Ok(InfoValue::Map(map))
        } else {
            Ok(InfoValue::Scalar(program))
        }
    }
}
```

## de.rs

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

## wm.rs

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
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        let de_lower = de.to_lowercase();
        if !de_lower.contains("gnome") && !de_lower.contains("kde")
            && !de_lower.contains("xfce") && !de_lower.contains("cinnamon")
            && !de_lower.contains("mate") && !de_lower.contains("budgie")
            && !de_lower.contains("pantheon")
        {
            return de;
        }
    }

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

## colors.rs

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
    format!("\x1b[48;5;{code}m  \x1b[0m", code = n)
}
```

## custom.rs

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct CustomCommandModule {
    pub name: &'static str,
    pub command: String,
    pub label: Option<String>,
    pub shell: Option<String>,
}

impl Module for CustomCommandModule {
    fn name(&self) -> &'static str { self.name }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let shell = self.shell.as_deref().unwrap_or("sh");
        let output = std::process::Command::new(shell)
            .arg("-c")
            .arg(&self.command)
            .output()
            .map_err(|e| crate::Error::Io(e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let label = self.label.clone().unwrap_or_else(|| self.name.to_string());

        if stdout.is_empty() {
            Ok(InfoValue::scalar(format!("{label}: ")))
        } else {
            Ok(InfoValue::scalar(format!("{label}: {stdout}")))
        }
    }
}
```

## Verify compilation

Run: `cargo build -p sysfetch-core`

## Commit

```bash
git add -A && git commit -m "feat: packages, shell, terminal, de, wm, colors, custom modules"
git push
```
