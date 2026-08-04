//! Phase 8.7 — observability & supportability (pure std, no `tracing` dep —
//! the diet applies; this gives 90% of the value with zero new dependencies).
//!
//! - `install_panic_hook()`: on panic, write a dump to
//!   `$XDG_CACHE_HOME/flexfetch/panic.log` (or `~/.cache/...`) and print a
//!   pointer to the issue tracker.
//! - `generate_bug_report()`: the `--bug-report` dump — version, OS, kernel,
//!   terminal, shell, columns, config path + parsed config, and the last module
//!   error log lines.
//! - `debug_enabled()`: `RUST_LOG`/`FLEXFETCH_LOG` gating for verbose output
//!   (the project's debug channel is `--debug`; the env vars turn it on too).

use flexfetch_core::{Context, InfoValue, ModuleRegistry};

/// Install a panic hook that persists a crash dump to the cache dir.
/// Best-effort: a failing write (read-only cache, etc.) is ignored.
pub fn install_panic_hook(cache_dir: &std::path::Path) {
    let dir = cache_dir.to_path_buf();
    std::panic::set_hook(Box::new(move |info| {
        let path = dir.join("panic.log");
        let _ = std::fs::create_dir_all(&dir);
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic payload".to_string()
        };
        let loc = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown location".into());
        let dump = format!(
            "flexfetch panic @ {loc}\npayload: {payload}\nversion: {}\n",
            env!("CARGO_PKG_VERSION")
        );
        let _ = std::fs::write(&path, dump);
        eprintln!(
            "\n\x1b[1;31mflexfetch crashed\x1b[0m: {payload} (at {loc})\n\
             Report saved to: {}\n\
             Please file a bug with `flexfetch --bug-report`:\n\
             https://github.com/mahesh-diwan/flexfetch/issues\n",
            path.display()
        );
    }));
}

/// `RUST_LOG=debug` (or `FLEXFETCH_LOG=debug/trace`) enables the same verbose
/// output as `--debug`.
pub fn debug_enabled() -> bool {
    for var in ["RUST_LOG", "FLEXFETCH_LOG"] {
        if let Ok(v) = std::env::var(var) {
            let v = v.to_lowercase();
            if v.contains("debug") || v.contains("trace") {
                return true;
            }
        }
    }
    false
}

/// The `--bug-report` dump: everything a maintainer needs to reproduce a
/// rendering/behavior bug. No secrets (custom module commands are included as
/// command *names* only).
pub fn generate_bug_report(ctx: &Context, config: &flexfetch_core::Config) -> String {
    let registry = ModuleRegistry::get();

    let os = match registry.run_individual("os", ctx) {
        Some(InfoValue::Map(m)) => m
            .get("pretty_name")
            .or_else(|| m.get("name"))
            .cloned()
            .unwrap_or_else(|| "unknown".into()),
        _ => "unknown".into(),
    };
    let kernel = match registry.run_individual("kernel", ctx) {
        Some(InfoValue::Scalar(s)) => s,
        _ => "unknown".into(),
    };
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "unknown".into());
    let terminal = std::env::var("TERM_PROGRAM")
        .or_else(|_| std::env::var("TERM"))
        .unwrap_or_else(|_| "unknown".into());
    let cols = std::env::var("COLUMNS").unwrap_or_else(|_| "unset".into());
    let colorterm = std::env::var("COLORTERM").unwrap_or_else(|_| "unset".into());
    let theme = config
        .display
        .theme
        .clone()
        .unwrap_or_else(|| "none".into());

    // Config file path + a redacted count of custom modules (commands can
    // contain secrets, so only their labels are listed).
    let cfg_dir = ctx.config_dir.display();
    let custom = if config.custom.is_empty() {
        "none".to_string()
    } else {
        config.custom.keys().cloned().collect::<Vec<_>>().join(", ")
    };

    format!(
        "\
flexfetch version:    {}
build features:       {}
OS:                   {}
Kernel:               {}
Shell:                {}
Terminal:             {}
COLUMNS:              {}
COLORTERM:            {}
Theme:                {}
Modules:              {}
Config dir:           {}
Custom modules:       {}
module_count:         {}
config template:      {}
",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
        os,
        kernel,
        shell,
        terminal,
        cols,
        colorterm,
        theme,
        config.modules.join(","),
        cfg_dir,
        custom,
        config.modules.len(),
        config.template,
    )
}
