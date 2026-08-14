use clap::Parser;
use flexfetch_cli::Cli;
use flexfetch_core::{
    get_cache_dir, presets, Config, Context, ModuleRegistry, SystemInfo, TeraEngine,
};
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Duration;

mod bench;
mod cli_dispatch;
mod config_load;
mod registry_resolve;
mod render_output;

#[cfg(feature = "live")]
mod live;
#[cfg(feature = "qr")]
mod qr;
mod ssh;
pub(crate) mod tools;
#[cfg(feature = "live")]
mod wizard;
// Phase 8.7 observability: panic hook + --bug-report dump + RUST_LOG gating.
mod telemetry;

fn main() {
    // Phase 8.7: crash dumps go to the cache dir before anything else can panic.
    telemetry::install_panic_hook(&get_cache_dir());

    // Phase 4.1: cold-start clock — measured from process entry (before clap
    // parse + config load) so `--benchmark` reports the true end-to-end time.
    let t_cold_start = std::time::Instant::now();

    // Handle --version before clap to show features
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        // `mut` is only used when lua/live/image-logos/etc are enabled; the
        // minimal build has no pushes so `mut` would be unused there.
        #[allow(unused_mut)]
        let mut features = ["watch"].to_vec();
        #[cfg(feature = "live")]
        features.push("live");
        #[cfg(feature = "image-logos")]
        features.push("image-logos");
        #[cfg(feature = "music")]
        features.push("music");
        #[cfg(feature = "tera")]
        features.push("tera");
        #[cfg(feature = "parallel")]
        features.push("parallel");
        #[cfg(feature = "completions")]
        features.push("completions");
        #[cfg(feature = "qr")]
        features.push("qr");
        #[cfg(feature = "auto-theme")]
        features.push("auto-theme");
        println!(
            "flexfetch {}\nFeatures: {}",
            env!("CARGO_PKG_VERSION"),
            features.join(", ")
        );
        return;
    }

    let cli = Cli::parse();

    // Phase 8.7: RUST_LOG=debug / FLEXFETCH_LOG=debug/trace enables --debug
    // output without an explicit flag (nice for `RUST_LOG=debug flexfetch`).
    let cli = if telemetry::debug_enabled() {
        Cli { debug: true, ..cli }
    } else {
        cli
    };

    // Subcommand: `completions <shell>` (clap_complete). Dispatch happens
    // before config load — it doesn't need one.
    if cli_dispatch::handle_subcommands(&cli) {
        return;
    }

    // Pre-config flags: --gen-config, --list-modules, --list-presets,
    // --list-themes, --hook, --update, --update-db.
    if cli_dispatch::handle_preflags(&cli) {
        return;
    }

    // --import-qr <image>: decode a QR config image and write it to disk.
    // Runs before config load because it doesn't need an existing config.
    if cli.import_qr.is_some() {
        #[cfg(feature = "qr")]
        {
            // Binding lives inside the cfg block so the feature-off build has
            // no unused-variable warning.
            if let Some(image) = &cli.import_qr {
                match qr::import_qr_image(image) {
                    Ok(toml_str) => write_imported_config(&cli, &toml_str),
                    Err(e) => {
                        eprintln!("import-qr error: {e}");
                        std::process::exit(1);
                    }
                }
            }
            return;
        }
        #[cfg(not(feature = "qr"))]
        {
            eprintln!("error: --import-qr requires the `qr` feature (build with --features qr)");
            std::process::exit(1);
        }
    }

    let config_path = cli.config.as_ref().map(std::path::Path::new);
    // `mut`: watch mode rebuilds config on hot-reload. `--flash` skips the
    // config file entirely — baked-in defaults only (no file IO on the fast
    // path is the whole point).
    let loaded = config_load::load(config_path, cli.flash, cli.debug);
    let mut config = loaded.config;
    let mut ctx = loaded.ctx;
    let config_dir = loaded.config_dir;
    let cache_dir = loaded.cache_dir;

    // --doctor: environment diagnostics (terminal, color, config, collectors).
    if cli.doctor {
        tools::run_doctor(&ctx);
        return;
    }

    // Phase 8.7 --bug-report: full environment/version dump for issue reports.
    if cli.bug_report {
        print!("{}", telemetry::generate_bug_report(&ctx, &config));
        return;
    }

    // --qr: render the effective config as a terminal QR code (Phase 4.11).
    // Prefers the raw config file when one exists: it preserves comments and
    // encodes only what the user wrote (smaller, scannable QR). Falls back to
    // the serialized effective config when there is no file on disk.
    if cli.qr {
        #[cfg(feature = "qr")]
        {
            let toml_str = config_file_path(&cli)
                .and_then(|p| std::fs::read_to_string(p).ok())
                .or_else(|| toml::to_string(&config).ok());
            match toml_str {
                Some(toml_str) => match qr::render_config_qr(&toml_str) {
                    Ok(rendered) => {
                        println!("{rendered}");
                        eprintln!(
                            "Scan with a phone (or --import-qr on another machine) to share this config."
                        );
                    }
                    Err(e) => {
                        eprintln!("qr error: {e}");
                        std::process::exit(1);
                    }
                },
                None => {
                    eprintln!("qr error: cannot serialize config");
                    std::process::exit(1);
                }
            }
            return;
        }
        #[cfg(not(feature = "qr"))]
        {
            eprintln!("error: --qr requires the `qr` feature (build with --features qr)");
            std::process::exit(1);
        }
    }

    // Live dashboard: owns the terminal until the user quits
    if cli.live {
        #[cfg(feature = "live")]
        {
            let watch_path = config_file_path(&cli);
            if let Err(e) = live::run(ctx, watch_path) {
                eprintln!("live dashboard error: {e}");
                std::process::exit(1);
            }
            return;
        }
        #[cfg(not(feature = "live"))]
        {
            eprintln!("error: --live requires the `live` feature (build with --features live)");
            std::process::exit(1);
        }
    }

    // Pipe detection. `--demo` always renders full color (it's for screenshots/
    // social previews, where stdout is usually a pipe).
    let is_tty = std::io::stdout().is_terminal();
    let pipe_mode = if cli.demo { false } else { cli.pipe || !is_tty };

    // Module toggle groups and presets
    let mut modules = registry_resolve::resolve(&cli, &config);

    // Pipe mode overrides
    apply_cli_overrides(&cli, &mut config, pipe_mode);

    let registry = registry_resolve::registry();
    let template_content = TeraEngine::default_template_content();

    // --wizard: interactive config wizard (owns the terminal until done)
    if cli.wizard {
        #[cfg(feature = "live")]
        {
            if let Err(e) = wizard::run() {
                eprintln!("wizard error: {e}");
                std::process::exit(1);
            }
            return;
        }
        #[cfg(not(feature = "live"))]
        {
            eprintln!("error: --wizard requires the `live` feature (build with --features live)");
            std::process::exit(1);
        }
    }

    // --ssh: fetch remote info from one or more hosts (parallel)
    if !cli.ssh.is_empty() {
        let results = ssh::fetch_all(&cli.ssh);
        for (host, info) in &results {
            println!("\x1b[1;36m== {host} ==\x1b[0m");
            match info {
                Ok(info) => render_output::render(info, &config, &cli, true),
                Err(e) => eprintln!("  error: {e}"),
            }
            println!();
        }
        return;
    }

    // --diff <target1> <target2>: side-by-side system comparison (Phase 4.9)
    if cli.diff.len() == 2 {
        let a = resolve_diff_target(&cli.diff[0], &modules, &ctx, registry, template_content);
        let b = resolve_diff_target(&cli.diff[1], &modules, &ctx, registry, template_content);
        match (a, b) {
            (Ok(ia), Ok(ib)) => render_output::diff(&ia, &ib, &cli.diff[0], &cli.diff[1]),
            (Err(e), _) | (_, Err(e)) => eprintln!("diff error: {e}"),
        }
        return;
    }

    // --prompt: single-line, ANSI-free prompt string
    if cli.prompt {
        println!("{}", render_output::prompt(&ctx, &modules));
        return;
    }

    // --motd: plain-text banner (no colors, no logo)
    if cli.motd {
        let info = registry.run_selected(&modules, &ctx, template_content);
        match flexfetch_core::export::export_markdown(&info, &config) {
            Ok(md) => print!("{md}"),
            Err(e) => eprintln!("motd error: {e}"),
        }
        return;
    }

    if cli.benchmark.is_some() {
        bench::run(
            &modules,
            &ctx,
            registry,
            template_content,
            &config,
            &cli,
            t_cold_start,
        );
        return;
    }

    let info = registry.run_selected(&modules, &ctx, template_content);

    // Handle --export flag
    if let Some(ref format) = cli.export {
        render_output::export(&info, &config, format, cli.output.as_deref());
        return;
    }

    if cli.watch {
        // Watch mode: refresh every N seconds, hot-reloading the config file
        // when it changes (mtime-based, no extra dependency).
        //
        // Phase 7.11 snapshot reuse: static modules (os/host/kernel/…) are
        // collected once and served from `snapshot` on every tick; only the
        // dynamic ones (cpuusage/memory/disk/network/battery/…) are re-collected.
        // The snapshot is reset on config hot-reload since the module set may
        // have changed.
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let r = running.clone();
        let _ = ctrlc::set_handler(move || {
            r.store(false, std::sync::atomic::Ordering::SeqCst);
        });
        let config_file = config_file_path(&cli);
        let mut last_mtime = config_file.as_deref().and_then(file_mtime);
        let mut snapshot: std::collections::HashMap<String, flexfetch_core::InfoValue> =
            std::collections::HashMap::new();
        while running.load(std::sync::atomic::Ordering::SeqCst) {
            // Config hot-reload: if the file changed, rebuild config/ctx/modules.
            if let Some(path) = &config_file {
                let now = file_mtime(path);
                if now != last_mtime {
                    last_mtime = now;
                    config =
                        Config::load(config_path).unwrap_or_else(|_| Config::default_for_testing());
                    apply_cli_overrides(&cli, &mut config, pipe_mode);
                    ctx = Context::new(
                        config_dir.clone(),
                        cache_dir.clone(),
                        cli.debug,
                        config.custom.clone(),
                    );
                    ctx.set_cache_ttl(config.cache_ttl);
                    modules = registry_resolve::resolve(&cli, &config);
                    snapshot.clear();
                    eprintln!("\n[flexfetch] config reloaded\n");
                }
            }
            print!("\x1b[2J\x1b[H");
            let fresh =
                registry.run_selected_cached(&modules, &ctx, template_content, &mut snapshot);
            render_output::render(&fresh, &config, &cli, false);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            std::thread::sleep(Duration::from_secs(cli.watch_interval));
        }
        println!();
        return;
    }

    render_output::render(&info, &config, &cli, false);
}

/// Apply display overrides from CLI flags (+ pipe mode) onto a loaded config.
/// Used by the main path and watch-mode config hot-reload.
fn apply_cli_overrides(cli: &Cli, config: &mut Config, pipe_mode: bool) {
    if let Some(ref theme) = cli.theme {
        config.display.theme = Some(theme.clone());
    }
    if cli.auto_theme {
        // Phase 5.4: `--auto-theme` wins over a config theme, but loses to an
        // explicit `--theme X` on the same invocation.
        if cli.theme.is_none() {
            config.display.theme = Some("auto".into());
        }
    }
    if cli.demo {
        // Phase 8.8: deterministic vibrant theme for screenshots (unless the
        // user overrode it with --theme/--auto-theme).
        if cli.theme.is_none() && !cli.auto_theme {
            config.display.theme = Some("catppuccin-mocha".into());
        }
        // Showcase the boxed frame (supported styles: double, decorative/single).
        if config.display.frame == "none" {
            config.display.frame = "decorative".into();
        }
    }
    if cli.no_gradient {
        config.display.gradient_title = false;
    }
    if cli.no_progress {
        config.display.progress_bars = false;
    }
    if let Some(ref s) = cli.box_style {
        config.display.box_style = s.clone();
    }
    if let Some(ref s) = cli.palette_style {
        config.display.palette_style = s.clone();
    }
    if let Some(ref s) = cli.frame {
        config.display.frame = s.clone();
    }
    if pipe_mode {
        config.display.theme = Some("none".into());
    }
}

/// The config file flexfetch actually reads: `--config PATH` if given, else the
/// default user config (`~/.config/flexfetch/config.toml`) if it exists.
fn config_file_path(cli: &Cli) -> Option<PathBuf> {
    if let Some(p) = &cli.config {
        return Some(PathBuf::from(p));
    }
    let default = tools::config_dir().join("config.toml");
    if default.exists() {
        Some(default)
    } else {
        None
    }
}

fn file_mtime(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

/// Resolve a `--diff` target into a `SystemInfo`: `local` collects locally,
/// `host@remote` (or any host without `/`) fetches via SSH, anything else is
/// treated as a path to a flexfetch JSON export file.
fn resolve_diff_target(
    target: &str,
    modules: &[String],
    ctx: &Context,
    registry: &'static ModuleRegistry,
    template_content: &str,
) -> Result<SystemInfo, String> {
    if target == "local" {
        return Ok(registry.run_selected(modules, ctx, template_content));
    }
    if target.contains('/') || target.ends_with(".json") {
        let content = std::fs::read_to_string(target).map_err(|e| format!("read {target}: {e}"))?;
        let json: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| format!("parse {target}: {e}"))?;
        return SystemInfo::from_json(&json).map_err(|e| format!("parse {target}: {e}"));
    }
    // Remote host (reuses the --ssh fetch machinery, including the scp fallback).
    let mut results = ssh::fetch_all(&[target.to_string()]);
    results
        .pop()
        .map(|(_, r)| r)
        .unwrap_or_else(|| Err(format!("no result from {target}")))
}

pub(crate) fn list_presets() {
    let builtins = presets::builtin_presets();
    println!("Built-in presets:");
    for (name, modules) in &builtins {
        let list: Vec<&str> = modules.iter().map(|s| s.as_str()).collect();
        println!("  {name:12} {}", list.join(", "));
    }

    // Check user presets directory
    let presets_dir = tools::config_dir().join("presets");
    if presets_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&presets_dir) {
            let user_presets: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "toml")
                        .unwrap_or(false)
                })
                .collect();
            if !user_presets.is_empty() {
                println!("\nUser presets ({}):", presets_dir.display());
                for entry in user_presets {
                    let name = entry
                        .path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("?")
                        .to_string();
                    println!("  {name}");
                }
            }
        }
    }
}

pub(crate) fn generate_config() {
    let config = Config::default_for_testing();
    let toml = toml::to_string_pretty(&config).unwrap_or_default();
    println!("{toml}");
    let config_dir = tools::config_dir();
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

/// Write an imported config to the target path — `--config PATH` if given, else
/// the default `~/.config/flexfetch/config.toml` — backing up any existing file
/// first (timestamped, so repeated imports never clobber an older backup).
#[cfg(feature = "qr")]
fn write_imported_config(cli: &Cli, toml_str: &str) {
    let path = cli
        .config
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| tools::config_dir().join("config.toml"));
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("error creating config dir: {e}");
            std::process::exit(1);
        }
    }
    if path.exists() {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let bak = path.with_extension(format!("toml.bak.{ts}"));
        if let Err(e) = std::fs::rename(&path, &bak) {
            eprintln!("error backing up existing config: {e}");
            std::process::exit(1);
        }
        eprintln!("backed up existing config to {bak:?}");
    }
    if let Err(e) = std::fs::write(&path, toml_str) {
        eprintln!("error writing config: {e}");
        std::process::exit(1);
    }
    println!("wrote imported config to {path:?}");
}

pub(crate) fn list_modules() {
    println!("Built-in modules:");
    for m in flexfetch_core::MODULE_CATALOG {
        if m.name != "title" {
            println!("  {}", m.name);
        }
    }
    println!("\nLayout directives (template-only): title, separator");
}
