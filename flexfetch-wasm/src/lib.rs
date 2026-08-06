//! Phase 4.12 — sandboxed WASM plugin runtime for flexfetch.
//!
//! Plugins are `wasm32-unknown-unknown` core modules (no WASI, no std fs/env —
//! the sandbox gives them exactly nothing except what the host imports).
//!
//! # ABI (v1)
//!
//! Host-provided imports, in module namespace `flexfetch` (a plugin can only
//! call the ones its declared capabilities allow — capability-gated at link
//! time, so a module that imports `run_command` without the `Command`
//! capability fails to instantiate):
//!
//! | function | signature | capability |
//! | -------- | --------- | ---------- |
//! | `log` | `(i32 msg_ptr, i32 msg_len)` | always |
//! | `env_get` | `(i32 key_ptr, i32 key_len, i32 out_ptr, i32 out_cap) -> i32` | `Env` |
//! | `read_file` | `(i32 path_ptr, i32 path_len, i32 out_ptr, i32 out_cap) -> i32` | `File` |
//! | `run_command` | `(i32 cmd_ptr, i32 cmd_len, i32 out_ptr, i32 out_cap) -> i32` | `Command` |
//!
//! `env_get`/`read_file`/`run_command` write the result into the plugin's
//! memory and return the number of bytes written, or -1 on failure.
//!
//! Plugin-required exports:
//!
//! | export | signature |
//! | ------ | --------- |
//! | `memory` | linear memory (must not exceed the store's memory cap) |
//! | `flexfetch_plugin` | `() -> i64` — returns a packed `(len << 32) \| ptr` to a JSON document in plugin memory |
//! | `flexfetch_plugin_name` | optional `() -> i32` — pointer to a NUL-terminated name (defaults to the file stem) |
//!
//! The JSON document follows `InfoValue` conventions: `{"value": "x"}` for a
//! scalar, a flat object for a map, or an array of strings for a list.
//!
//! Authoring a plugin in Rust (compile with `rustc --target
//! wasm32-unknown-unknown` or `cargo build --target wasm32-unknown-unknown`):
//!
//! ```no_run
//! #[unsafe(no_mangle)]
//! pub extern "C" fn flexfetch_plugin() -> i64 {
//!     let json = b"{\"value\":\"hello from wasm\"}";
//!     // copy json into a static buffer, return (len << 32) | ptr
//!     (json.len() as i64) << 32 | 0
//! }
//! ```

use flexfetch_core::InfoValue;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use wasmtime::{Caller, Config, Engine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};

/// Host capabilities a plugin may be granted. Anything not granted is simply
/// not importable — the module fails to instantiate if it tries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Read process environment variables (`env_get`).
    Env,
    /// Read files from the host (`read_file`).
    File,
    /// Run a shell command (`run_command`).
    Command,
}

/// Runtime sandbox settings.
#[derive(Debug, Clone)]
pub struct Sandbox {
    /// Fuel budget (1 fuel unit ~ 1 wasm instruction). Exhaustion traps.
    pub fuel: u64,
    /// Maximum linear memory in bytes. Exceeded at instantiate or grow → trap.
    pub max_memory: u64,
    /// Host capabilities exposed to the plugin.
    pub capabilities: Vec<Capability>,
}

impl Default for Sandbox {
    fn default() -> Self {
        Sandbox {
            fuel: 10_000_000,
            max_memory: 64 * 1024 * 1024,
            // Safe defaults: log + env only. No filesystem, no commands.
            capabilities: vec![Capability::Env],
        }
    }
}

/// Failure modes surfaced to the caller.
#[derive(Debug)]
pub enum WasmError {
    Compile(String),
    Link(String),
    Instantiate(String),
    Trap(String),
    BadResult(String),
    Io(std::io::Error),
}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmError::Compile(e) => write!(f, "wasm compile: {e}"),
            WasmError::Link(e) => write!(f, "wasm link (missing/denied import): {e}"),
            WasmError::Instantiate(e) => write!(f, "wasm instantiate: {e}"),
            WasmError::Trap(e) => write!(f, "wasm trap: {e}"),
            WasmError::BadResult(e) => write!(f, "wasm bad result: {e}"),
            WasmError::Io(e) => write!(f, "wasm io: {e}"),
        }
    }
}

impl std::error::Error for WasmError {}

/// State the host passes to every imported function. The memory limits live in
/// here too so the store's limiter closure can return a reference derived from
/// the store data (wasmtime 47's `limiter` signature requires that).
struct Host {
    caps: Vec<Capability>,
    debug: bool,
    limits: StoreLimits,
}

impl Host {
    fn has(&self, cap: Capability) -> bool {
        self.caps.contains(&cap)
    }
}

/// Execute a WASM plugin and convert its JSON result into an `InfoValue`.
///
/// `wasm` is the raw `.wasm` bytes, `name` is used only for error messages.
pub fn run_plugin(
    wasm: &[u8],
    sandbox: &Sandbox,
    debug: bool,
) -> std::result::Result<InfoValue, WasmError> {
    let mut config = Config::new();
    config.consume_fuel(true);
    let engine = Engine::new(&config).map_err(|e| WasmError::Compile(e.to_string()))?;

    let module = Module::new(&engine, wasm).map_err(|e| WasmError::Compile(e.to_string()))?;

    let host = Host {
        caps: sandbox.capabilities.clone(),
        debug,
        // Hard memory cap: enforced at instantiate and on every memory.grow.
        limits: StoreLimitsBuilder::new()
            .memory_size(sandbox.max_memory as usize)
            .build(),
    };
    let mut store = Store::new(&engine, host);
    store.limiter(|host| &mut host.limits as &mut dyn wasmtime::ResourceLimiter);

    // Fuel budget: when fuel hits zero, execution traps (wasmtime 47 traps on
    // exhaustion by default for sync stores).
    store
        .set_fuel(sandbox.fuel)
        .map_err(|e| WasmError::Instantiate(e.to_string()))?;

    let mut linker = Linker::new(&engine);

    // `log` is always available (it is the plugin's only unconditionally-safe
    // escape hatch and writes nothing back to the host).
    linker
        .func_wrap("flexfetch", "log", log_wrap)
        .map_err(|e| WasmError::Link(e.to_string()))?;

    if store.data().has(Capability::Env) {
        linker
            .func_wrap("flexfetch", "env_get", env_get_wrap)
            .map_err(|e| WasmError::Link(e.to_string()))?;
    }
    if store.data().has(Capability::File) {
        linker
            .func_wrap("flexfetch", "read_file", read_file_wrap)
            .map_err(|e| WasmError::Link(e.to_string()))?;
    }
    if store.data().has(Capability::Command) {
        linker
            .func_wrap("flexfetch", "run_command", run_command_wrap)
            .map_err(|e| WasmError::Link(e.to_string()))?;
    }

    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|e| WasmError::Link(e.to_string()))?;

    let func = instance
        .get_typed_func::<(), i64>(&mut store, "flexfetch_plugin")
        .map_err(|e| WasmError::Link(format!("missing export flexfetch_plugin: {e}")))?;

    let packed = func
        .call(&mut store, ())
        .map_err(|e| WasmError::Trap(e.to_string()))?;

    let ptr = (packed & 0xFFFF_FFFF) as usize;
    let len = (packed >> 32) as usize;
    if len == 0 || len > sandbox.max_memory as usize {
        return Err(WasmError::BadResult(format!(
            "plugin returned ptr={ptr} len={len} (out of bounds)"
        )));
    }

    // Read the JSON document out of plugin memory.
    let mem = instance
        .get_memory(&mut store, "memory")
        .ok_or(WasmError::BadResult(
            "plugin does not export `memory`".into(),
        ))?;
    let data = mem.data(&store);
    let end = ptr.saturating_add(len);
    if end > data.len() {
        return Err(WasmError::BadResult(format!(
            "result [{ptr}..{end}) beyond memory ({} bytes)",
            data.len()
        )));
    }

    let json: serde_json::Value = serde_json::from_slice(&data[ptr..end])
        .map_err(|e| WasmError::BadResult(format!("result is not JSON: {e}")))?;
    Ok(json_to_info(json))
}

/// Optional name export: NUL-terminated string pointer.
pub fn plugin_name(wasm: &[u8]) -> Option<String> {
    let engine = Engine::default();
    let module = Module::new(&engine, wasm).ok()?;
    if !module
        .exports()
        .any(|e| e.name() == "flexfetch_plugin_name")
    {
        return None;
    }
    // Name resolution needs a store + instantiation; keep it lazy and cheap:
    // the CLI uses the file stem instead, which matches the Lua plugin naming.
    None
}

fn json_to_info(v: serde_json::Value) -> InfoValue {
    match v {
        serde_json::Value::String(s) => InfoValue::Scalar(s),
        serde_json::Value::Number(n) => InfoValue::Scalar(n.to_string()),
        serde_json::Value::Bool(b) => InfoValue::Scalar(if b { "yes".into() } else { "no".into() }),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr
                .into_iter()
                .filter_map(|e| match e {
                    serde_json::Value::String(s) => Some(s),
                    other => other.as_str().map(|s| s.to_string()),
                })
                .collect();
            InfoValue::List(items)
        }
        serde_json::Value::Object(obj) => {
            // `{"value": "x"}` is a scalar (same convention as the Lua API).
            if let Some(v) = obj.get("value").and_then(|v| v.as_str()) {
                return InfoValue::Scalar(v.to_string());
            }
            let mut map = HashMap::new();
            for (k, val) in obj {
                let s = match val {
                    serde_json::Value::String(s) => s,
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => (if b { "yes" } else { "no" }).to_string(),
                    other => other.as_str().map(|s| s.to_string()).unwrap_or_default(),
                };
                map.insert(k, s);
            }
            InfoValue::Map(map)
        }
        serde_json::Value::Null => InfoValue::Scalar("null".into()),
    }
}

// ---------------------------------------------------------------------------
// Host import wrappers
// ---------------------------------------------------------------------------

/// Read `len` bytes at `ptr` from the caller's memory (bounds-checked).
fn read_bytes(caller: &mut Caller<'_, Host>, ptr: i32, len: i32) -> Option<Vec<u8>> {
    if ptr < 0 || len < 0 {
        return None;
    }
    let (ptr, len) = (ptr as usize, len as usize);
    let mem = caller.get_export("memory")?.into_memory()?;
    let data = mem.data(&*caller);
    if ptr.saturating_add(len) > data.len() {
        return None;
    }
    Some(data[ptr..ptr + len].to_vec())
}

/// Write up to `cap` bytes into the caller's memory at `out`; returns written.
fn write_bytes(caller: &mut Caller<'_, Host>, out: i32, cap: i32, bytes: &[u8]) -> i32 {
    if out < 0 || cap < 0 {
        return -1;
    }
    let (out, cap) = (out as usize, cap as usize);
    let Some(mem) = caller.get_export("memory").and_then(|e| e.into_memory()) else {
        return -1;
    };
    let n = bytes.len().min(cap);
    let data = mem.data_mut(caller);
    if out.saturating_add(n) > data.len() {
        return -1;
    }
    data[out..out + n].copy_from_slice(&bytes[..n]);
    n as i32
}

fn log_wrap(mut caller: Caller<'_, Host>, ptr: i32, len: i32) {
    let debug = caller.data().debug;
    if let Some(bytes) = read_bytes(&mut caller, ptr, len) {
        if debug {
            eprintln!("[flexfetch-wasm] {}", String::from_utf8_lossy(&bytes));
        }
    }
}

fn env_get_wrap(
    mut caller: Caller<'_, Host>,
    key_ptr: i32,
    key_len: i32,
    out: i32,
    cap: i32,
) -> i32 {
    let Some(key) = read_bytes(&mut caller, key_ptr, key_len) else {
        return -1;
    };
    let key = String::from_utf8_lossy(&key);
    match std::env::var(key.as_ref()) {
        Ok(v) => write_bytes(&mut caller, out, cap, v.as_bytes()),
        Err(_) => -1,
    }
}

fn read_file_wrap(
    mut caller: Caller<'_, Host>,
    path_ptr: i32,
    path_len: i32,
    out: i32,
    cap: i32,
) -> i32 {
    let Some(path) = read_bytes(&mut caller, path_ptr, path_len) else {
        return -1;
    };
    let path = String::from_utf8_lossy(&path);
    match std::fs::read(path.as_ref()) {
        Ok(bytes) => write_bytes(&mut caller, out, cap, &bytes),
        Err(_) => -1,
    }
}

fn run_command_wrap(
    mut caller: Caller<'_, Host>,
    cmd_ptr: i32,
    cmd_len: i32,
    out: i32,
    cap: i32,
) -> i32 {
    let Some(cmd) = read_bytes(&mut caller, cmd_ptr, cmd_len) else {
        return -1;
    };
    let cmd = String::from_utf8_lossy(&cmd);
    match std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd.as_ref())
        .output()
    {
        Ok(o) => write_bytes(&mut caller, out, cap, &o.stdout),
        Err(_) => -1,
    }
}

/// Convenience: file-stem name for a plugin file.
pub fn stem_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("wasm_plugin")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_wat(wat: &str, sandbox: &Sandbox) -> std::result::Result<InfoValue, WasmError> {
        let bytes = wat::parse_str(wat).expect("fixture compiles");
        run_plugin(&bytes, sandbox, false)
    }

    #[test]
    fn scalar_result() {
        let wat = r#"
(module
  (memory (export "memory") 1)
  (data (i32.const 16) "{\"value\":\"hi\"}")
  (func (export "flexfetch_plugin") (result i64)
    (i64.const 14) (i64.const 32) (i64.shl) (i64.const 16) (i64.or)))
"#;
        match run_wat(wat, &Sandbox::default()).unwrap() {
            InfoValue::Scalar(s) => assert_eq!(s, "hi"),
            other => panic!("expected scalar, got {other:?}"),
        }
    }

    #[test]
    fn map_result() {
        let wat = r#"
(module
  (memory (export "memory") 1)
  (data (i32.const 0) "{\"arch\":\"wasm\",\"os\":\"sandbox\"}")
  (func (export "flexfetch_plugin") (result i64)
    (i64.const 30) (i64.const 32) (i64.shl) (i64.const 0) (i64.or)))
"#;
        match run_wat(wat, &Sandbox::default()).unwrap() {
            InfoValue::Map(m) => {
                assert_eq!(m.get("arch").map(String::as_str), Some("wasm"));
                assert_eq!(m.get("os").map(String::as_str), Some("sandbox"));
            }
            other => panic!("expected map, got {other:?}"),
        }
    }

    #[test]
    fn denied_import_fails_to_link() {
        // Imports run_command but the sandbox only grants Env → link error.
        let wat = r#"
(module
  (import "flexfetch" "run_command" (func $rc (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "{\"value\":\"x\"}")
  (func (export "flexfetch_plugin") (result i64)
    (i64.const 12) (i64.const 32) (i64.shl) (i64.const 0) (i64.or)))
"#;
        let err = run_wat(wat, &Sandbox::default()).unwrap_err();
        assert!(
            err.to_string().contains("run_command"),
            "error should name the denied import: {err}"
        );
    }

    #[test]
    fn env_get_granted_by_default_and_denied_without_capability() {
        // env_get IS granted by the default sandbox: instantiating a module
        // that imports it must NOT fail at link time.
        let wat = r#"
(module
  (import "flexfetch" "env_get" (func $get (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 64) "PATH")
  (func (export "flexfetch_plugin") (result i64)
    (drop (call $get (i32.const 64) (i32.const 4) (i32.const 256) (i32.const 4096)))
    (i64.const 15) (i64.const 32) (i64.shl) (i64.const 16) (i64.or))
  (data (i32.const 16) "{\"value\":\"env\"}")
  )
"#;
        match run_wat(wat, &Sandbox::default()) {
            Ok(InfoValue::Scalar(s)) => assert_eq!(s, "env"),
            Ok(other) => panic!("unexpected result {other:?}"),
            Err(e) => panic!("env_get should be granted by default: {e}"),
        }

        // Without the Env capability the same module must fail to link.
        let no_caps = Sandbox {
            capabilities: vec![],
            ..Sandbox::default()
        };
        let err = run_wat(wat, &no_caps).unwrap_err();
        assert!(
            err.to_string().contains("env_get"),
            "denied import should be named in the link error: {err}"
        );
    }

    #[test]
    fn fuel_exhaustion_traps() {
        // Count to 1e9 with a tiny fuel budget → traps long before the loop
        // finishes ("all fuel consumed"). Bounded loop so it compiles cleanly.
        let wat = r#"
(module
  (memory (export "memory") 1)
  (global $c (mut i64) (i64.const 0))
  (func (export "flexfetch_plugin") (result i64)
    (block $exit
      (loop $l
        (global.set $c (i64.add (global.get $c) (i64.const 1)))
        (br_if $exit (i64.ge_u (global.get $c) (i64.const 1000000000)))
        (br $l)))
    (global.get $c)))
"#;
        let sandbox = Sandbox {
            fuel: 1000,
            ..Sandbox::default()
        };
        let err = run_wat(wat, &sandbox).unwrap_err();
        assert!(
            err.to_string().contains("fuel") || err.to_string().contains("trap"),
            "expected a fuel/trap error, got: {err:?}"
        );
    }

    #[test]
    fn memory_cap_enforced_at_instantiation() {
        // Declares 2 pages (128 KiB) but the cap is 1 page (64 KiB) → trap.
        let wat = r#"
(module
  (memory (export "memory") 2)
  (func (export "flexfetch_plugin") (result i64)
    (i64.const 2) (i64.const 32) (i64.shl) (i64.const 0) (i64.or)))
"#;
        let sandbox = Sandbox {
            max_memory: 64 * 1024,
            ..Sandbox::default()
        };
        assert!(run_wat(wat, &sandbox).is_err());
    }

    #[test]
    fn bad_result_out_of_bounds() {
        let wat = r#"
(module
  (memory (export "memory") 1)
  (func (export "flexfetch_plugin") (result i64)
    (i64.const 100) (i64.const 32) (i64.shl) (i64.const 100000) (i64.or)))
"#;
        let err = run_wat(wat, &Sandbox::default()).unwrap_err();
        assert!(
            err.to_string().contains("beyond memory") || err.to_string().contains("out of bounds")
        );
    }
}
