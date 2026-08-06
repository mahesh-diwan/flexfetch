//! Phase 4.12 — plugin auto-loading (Lua + WASM).
//!
//! Every run, flexfetch scans the plugins dir (`~/.config/flexfetch/plugins/`),
//! executes each plugin, and flattens the results into one `plugins` table
//! entry (a row per plugin: `{label, value}`) so the default template can
//! render them all with zero user template edits.
//!
//! - `.lua` files run through `flexfetch-lua` (mlua; the `lua` feature,
//!   default on). Unsandboxed by design — Lua is a trusted, user-editable
//!   scripting surface, same trust model as `--module custom` commands.
//! - `.wasm` files run through `flexfetch-wasm` (wasmtime; the `wasm-plugins`
//!   feature, off by default — heavy dep tree). Sandboxed: fuel + memory
//!   limits, and host imports are capability-gated so an untrusted module can
//!   only `log` and read env vars, never touch the filesystem or spawn
//!   commands.
//!
//! A plugin that fails to load/run is skipped with a debug note — one broken
//! plugin never takes down the whole fetch.

use flexfetch_core::{Context, InfoValue};
use std::collections::HashMap;
use std::path::Path;

/// Run every plugin in the plugins dir and flatten the results into a single
/// `InfoValue::Table` (rows `{label, value}`, one per plugin). Returns `None`
/// when there are no runnable plugins (or none succeeded) so the caller can
/// skip adding an empty `plugins` entry entirely.
pub fn collect_plugins(ctx: &Context) -> Option<InfoValue> {
    let dir = crate::registry::plugins_dir();
    let entries = std::fs::read_dir(&dir).ok()?;
    let mut rows: Vec<HashMap<String, String>> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let label = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("plugin")
            .to_string();
        match ext {
            #[cfg(feature = "lua")]
            "lua" => match run_lua(&path, ctx) {
                Ok(v) => push_row(&mut rows, label, v),
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] lua plugin {label} error: {e}");
                    }
                }
            },
            #[cfg(feature = "wasm-plugins")]
            "wasm" => match run_wasm(&path) {
                Ok(v) => push_row(&mut rows, label, v),
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] wasm plugin {label} error: {e}");
                    }
                }
            },
            _ => {}
        }
    }

    if rows.is_empty() {
        None
    } else {
        Some(InfoValue::Table(rows))
    }
}

#[cfg(feature = "lua")]
fn run_lua(path: &Path, ctx: &Context) -> flexfetch_core::Result<InfoValue> {
    let module = flexfetch_lua::LuaModule::new(path.to_path_buf());
    // Fully-qualified: `collect` collides with Iterator::collect on method lookup.
    flexfetch_core::Module::collect(&module, ctx)
}

#[cfg(feature = "wasm-plugins")]
fn run_wasm(path: &Path) -> flexfetch_core::Result<InfoValue> {
    let wasm = std::fs::read(path)?;
    let sandbox = flexfetch_wasm::Sandbox::default();
    flexfetch_wasm::run_plugin(&wasm, &sandbox, false)
        .map_err(|e| flexfetch_core::Error::Config(e.to_string()))
}

/// Add one plugin row: `{label: <stem>, value: <formatted result>}`.
fn push_row(rows: &mut Vec<HashMap<String, String>>, label: String, value: InfoValue) {
    let mut row = HashMap::new();
    row.insert("label".into(), label);
    row.insert("value".into(), fmt_value(&value));
    rows.push(row);
}

/// Flatten any InfoValue into a one-line display string, honoring the same
/// `{"value": "x"}` scalar convention the Lua and WASM ABIs share.
fn fmt_value(v: &InfoValue) -> String {
    match v {
        InfoValue::Scalar(s) => s.clone(),
        InfoValue::Map(m) => {
            if let Some(v) = m.get("value") {
                return v.clone();
            }
            let mut parts: Vec<String> = m.iter().map(|(k, val)| format!("{k}: {val}")).collect();
            parts.sort();
            parts.join(", ")
        }
        InfoValue::List(l) => l.join(", "),
        InfoValue::Table(t) => t
            .iter()
            .filter_map(|row| row.get("value").filter(|v| !v.is_empty()).cloned())
            .collect::<Vec<_>>()
            .join(", "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `with_plugins_dir` mutates the process-global XDG_CONFIG_HOME, so any
    // tests that touch the plugins dir must run serialized or they stomp each
    // other's env var (rust's harness runs #[test]s on parallel threads).
    // Only the wasm-gated tests use these helpers; without the feature they'd
    // be dead code in the default build.
    #[cfg(feature = "wasm-plugins")]
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Point the plugins dir at a temp dir for the duration of the test.
    /// Caller must hold `ENV_LOCK` (see `_guard` in each test).
    #[cfg(feature = "wasm-plugins")]
    fn with_plugins_dir() -> tempdir::TempDir {
        let dir = tempdir::TempDir::new("flexfetch-plugins").expect("temp dir");
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
        dir
    }

    #[cfg(feature = "wasm-plugins")]
    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner())
    }

    #[cfg(feature = "wasm-plugins")]
    #[test]
    fn wasm_plugins_render_rows() {
        let _guard = env_guard();
        let dir = with_plugins_dir();
        let plugins = dir.path().join("flexfetch").join("plugins");
        std::fs::create_dir_all(&plugins).unwrap();

        // A WAT plugin that returns {"value":"hello from wasm"}.
        let wat = r#"
(module
  (memory (export "memory") 1)
  (data (i32.const 0) "{\"value\":\"hello from wasm\"}")
  (func (export "flexfetch_plugin") (result i64)
    (i64.const 27) (i64.const 32) (i64.shl) (i64.const 0) (i64.or)))
"#;
        let wasm = wat::parse_str(wat).expect("wat compiles");
        std::fs::write(plugins.join("hello.wasm"), &wasm).unwrap();

        let ctx = Context::new(
            dir.path().to_path_buf(),
            dir.path().join("cache"),
            false,
            Default::default(),
        );
        let out = collect_plugins(&ctx).expect("plugins collected");
        let InfoValue::Table(rows) = out else {
            panic!("expected a table, got {out:?}");
        };
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get("label").map(String::as_str), Some("hello"));
        assert_eq!(
            rows[0].get("value").map(String::as_str),
            Some("hello from wasm")
        );
    }

    #[cfg(feature = "wasm-plugins")]
    #[test]
    fn broken_wasm_plugin_is_skipped() {
        let _guard = env_guard();
        let dir = with_plugins_dir();
        let plugins = dir.path().join("flexfetch").join("plugins");
        std::fs::create_dir_all(&plugins).unwrap();
        // Garbage bytes: fails to compile, must be skipped (not crash).
        std::fs::write(plugins.join("broken.wasm"), b"not a wasm module").unwrap();

        let ctx = Context::new(
            dir.path().to_path_buf(),
            dir.path().join("cache"),
            false,
            Default::default(),
        );
        assert!(collect_plugins(&ctx).is_none(), "broken plugin is skipped");
    }

    #[cfg(feature = "wasm-plugins")]
    #[test]
    fn capability_denied_plugin_is_skipped() {
        let _guard = env_guard();
        let dir = with_plugins_dir();
        let plugins = dir.path().join("flexfetch").join("plugins");
        std::fs::create_dir_all(&plugins).unwrap();

        // Imports run_command (denied under the default Env-only sandbox) and
        // never reaches a result — instantiation must fail, plugin skipped.
        let wat = r#"
(module
  (import "flexfetch" "run_command" (func $rc (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "flexfetch_plugin") (result i64) (i64.const 0)))
"#;
        let wasm = wat::parse_str(wat).expect("wat compiles");
        std::fs::write(plugins.join("evil.wasm"), &wasm).unwrap();

        let ctx = Context::new(
            dir.path().to_path_buf(),
            dir.path().join("cache"),
            false,
            Default::default(),
        );
        assert!(
            collect_plugins(&ctx).is_none(),
            "capability-denied plugin is skipped"
        );
    }

    #[test]
    fn fmt_value_shapes() {
        assert_eq!(fmt_value(&InfoValue::Scalar("x".into())), "x");
        let mut m = HashMap::new();
        m.insert("value".into(), "v".into());
        assert_eq!(fmt_value(&InfoValue::Map(m)), "v");
        let mut m2 = HashMap::new();
        m2.insert("b".into(), "2".into());
        m2.insert("a".into(), "1".into());
        assert_eq!(fmt_value(&InfoValue::Map(m2)), "a: 1, b: 2");
        assert_eq!(
            fmt_value(&InfoValue::List(vec!["a".into(), "b".into()])),
            "a, b"
        );
    }

    // Tiny temp-dir helper to avoid adding a dev-dep for two tests. Lives inside
    // the test module so the shipped binary never compiles (or dead-code-warns)
    // it.
    #[cfg(feature = "wasm-plugins")]
    mod tempdir {
        pub struct TempDir(std::path::PathBuf);

        impl TempDir {
            pub fn new(prefix: &str) -> std::io::Result<TempDir> {
                let base = std::env::temp_dir();
                let path = base.join(format!(
                    "{prefix}-{}-{}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_nanos())
                        .unwrap_or(0)
                ));
                std::fs::create_dir_all(&path)?;
                Ok(TempDir(path))
            }
            pub fn path(&self) -> &std::path::Path {
                &self.0
            }
        }

        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }
}
