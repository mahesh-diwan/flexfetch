use clap::Parser;
use flexfetch_cli::Cli;
use flexfetch_core::{
    get_cache_dir, presets, Config, Context, InfoValue, ModuleRegistry, SystemInfo, TeraEngine,
};
use std::collections::HashMap;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Duration;

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
            (Ok(ia), Ok(ib)) => render_diff(&ia, &ib, &cli.diff[0], &cli.diff[1]),
            (Err(e), _) | (_, Err(e)) => eprintln!("diff error: {e}"),
        }
        return;
    }

    // --prompt: single-line, ANSI-free prompt string
    if cli.prompt {
        println!("{}", render_prompt(&ctx, &modules));
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
        benchmark(
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
                    modules = resolve_modules(&cli, &config);
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

/// Resolve the module list from CLI flags/presets/config (shared by the main
/// path and watch-mode config hot-reload).
pub(crate) fn resolve_modules(cli: &Cli, config: &Config) -> Vec<String> {
    // Phase 8.8 --demo: every built-in module in a showcase order.
    if cli.demo {
        return vec![
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
            "cpucache".into(),
            "cpuusage".into(),
            "gpu".into(),
            "memory".into(),
            "swap".into(),
            "disk".into(),
            "network".into(),
            "resolution".into(),
            "display".into(),
            "battery".into(),
            "temperature".into(),
            "processes".into(),
            "dns".into(),
            "colors".into(),
            // Deliberately excluded for determinism/speed: publicip (network
            // round-trip), wifi (nmcli), bluetooth (2× bluetoothctl spawn).
        ];
    }
    // --flash: the fast path always runs the lean fixed module set, ignoring
    // config.modules and the --minimal/--full/--preset/--modules switches
    // (everything baked in, nothing user-configurable). --demo above wins.
    if cli.flash {
        return presets::module_group("flash");
    }
    let mut modules: Vec<String> = if cli.minimal {
        presets::module_group("minimal")
    } else if cli.full {
        presets::module_group("full")
    } else if cli.dev {
        presets::module_group("dev")
    } else if let Some(ref preset_name) = cli.preset {
        load_preset(preset_name)
    } else if let Some(ref m) = cli.modules {
        m.split(':').map(|s| s.to_string()).collect()
    } else {
        config.modules.clone()
    };

    // --smart: append $PWD context modules (git, project, container/venv/SSH)
    if cli.smart {
        for name in ["git", "project", "context"] {
            if !modules.iter().any(|m| m == name) {
                modules.push(name.to_string());
            }
        }
    }
    // --health: append the system health module
    if cli.health && !modules.iter().any(|m| m == "health") {
        modules.push("health".to_string());
    }
    modules
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

/// Render a 3-column side-by-side diff table (Phase 4.9). Rows are aligned by
/// module name; differing values are highlighted (red for A, green for B).
fn render_diff(a: &SystemInfo, b: &SystemInfo, name_a: &str, name_b: &str) {
    let a_map: HashMap<&str, &InfoValue> = a.entries.iter().map(|(n, v)| (*n, v)).collect();
    let b_map: HashMap<&str, &InfoValue> = b.entries.iter().map(|(n, v)| (*n, v)).collect();

    // Union of module names, preserving A's order then any B-only modules.
    let mut names: Vec<&str> = a.entries.iter().map(|(n, _)| *n).collect();
    for (n, _) in &b.entries {
        if !names.contains(n) {
            names.push(n);
        }
    }

    let w = 12usize;
    println!("\x1b[1;36m{name_a:<20}\x1b[0m vs \x1b[1;36m{name_b:<20}\x1b[0m");
    println!("{:<w$} | {:<24} | {:<24}", "Property", name_a, name_b);
    println!("{:-<1$}", "", w + 2 + 26 + 26);

    for name in names {
        let va = a_map.get(name).map(|v| v.summary()).unwrap_or_default();
        let vb = b_map.get(name).map(|v| v.summary()).unwrap_or_default();
        let (ca, cb) = if va != vb {
            ("\x1b[31m", "\x1b[32m")
        } else {
            ("", "")
        };
        println!(
            "{:<w$} | {ca}{:<24}\x1b[0m | {cb}{:<24}\x1b[0m",
            name, va, vb
        );
    }
}

fn render_prompt(ctx: &Context, modules: &[String]) -> String {
    let registry = ModuleRegistry::get();
    let mut parts: Vec<String> = Vec::new();

    // OS: distro name/logo-ish hint
    if modules.iter().any(|m| m == "os") {
        if let Some(InfoValue::Map(m)) = registry.run_individual("os", ctx) {
            let name = m
                .get("pretty_name")
                .or_else(|| m.get("name"))
                .cloned()
                .unwrap_or_default();
            if !name.is_empty() {
                parts.push(name.to_lowercase());
            }
        }
    }
    // CPU usage
    if modules.iter().any(|m| m == "cpuusage") {
        if let Some(InfoValue::Scalar(s)) = registry.run_individual("cpuusage", ctx) {
            if s != "unknown" {
                parts.push(format!("CPU {s}"));
            }
        }
    }
    // Memory
    if modules.iter().any(|m| m == "memory") {
        if let Some(InfoValue::Map(m)) = registry.run_individual("memory", ctx) {
            let used = m.get("used").cloned().unwrap_or_default();
            let total = m.get("total").cloned().unwrap_or_default();
            if !used.is_empty() && !total.is_empty() {
                parts.push(format!("RAM {used}/{total}"));
            }
        }
    }

    if parts.is_empty() {
        String::new()
    } else {
        parts.join(" | ")
    }
}

fn benchmark(
    modules: &[String],
    ctx: &Context,
    registry: &'static ModuleRegistry,
    template_content: &str,
    config: &Config,
    cli: &Cli,
    t_cold_start: std::time::Instant,
) {
    let iterations = cli.benchmark.unwrap_or(1).max(1);
    let t0 = std::time::Instant::now();

    // Per-module timing (single iteration, existing behavior)
    let mut timings = Vec::new();
    for name in modules {
        if name == "title" || name == "separator" {
            continue;
        }
        let t = std::time::Instant::now();
        let _ = registry.run_individual(name, ctx);
        timings.push((name.clone(), t.elapsed()));
    }
    timings.sort_by_key(|&(_, dur)| std::cmp::Reverse(dur));

    // Micro-benchmark: run the full selected pipeline N times. Keep the last
    // `info` around so the single-iteration branch can render it directly
    // instead of running `run_selected` a second time.
    let mut run_selected_times = Vec::new();
    let mut render_times = Vec::new();
    let mut last_info = None;
    for _ in 0..iterations {
        let t = std::time::Instant::now();
        let info = registry.run_selected(modules, ctx, template_content);
        run_selected_times.push(t.elapsed());
        let engine = TeraEngine::new_default();
        let t = std::time::Instant::now();
        let _ = engine.render(&info, config);
        render_times.push(t.elapsed());
        last_info = Some(info);
    }

    eprintln!(
        "--- flexfetch benchmark ({iterations} iteration{}) ---",
        if iterations == 1 { "" } else { "s" }
    );
    eprintln!("  cold start:      {:?}", t_cold_start.elapsed());
    eprintln!("  setup:           {:?}", t0.elapsed());
    for (name, dur) in &timings {
        eprintln!("  {name:15} {dur:?}");
    }
    if iterations > 1 {
        let avg = |v: &[std::time::Duration]| -> std::time::Duration {
            let sum: std::time::Duration = v.iter().sum();
            sum / iterations as u32
        };
        let min = |v: &[std::time::Duration]| -> std::time::Duration {
            *v.iter().min().unwrap_or(&std::time::Duration::ZERO)
        };
        eprintln!(
            "  run_selected:    avg {:?} (min {:?})",
            avg(&run_selected_times),
            min(&run_selected_times)
        );
        eprintln!(
            "  template render: avg {:?} (min {:?})",
            avg(&render_times),
            min(&render_times)
        );
        eprintln!("  total:           {:?}", t0.elapsed());
    } else {
        let engine = TeraEngine::new_default();
        let t = std::time::Instant::now();
        if let Some(info) = &last_info {
            let _ = engine.render(info, config);
        }
        eprintln!("  run_selected:    {:?}", run_selected_times[0]);
        eprintln!("  template render: {:?}", t.elapsed());
        eprintln!("  total:           {:?}", t0.elapsed());
    }
    eprintln!("---");

    if let Some(ref format) = cli.export {
        let info =
            last_info.unwrap_or_else(|| registry.run_selected(modules, ctx, template_content));
        handle_export(&info, config, format, cli.output.as_deref());
        return;
    }
    if cli.format == "json" {
        let info =
            last_info.unwrap_or_else(|| registry.run_selected(modules, ctx, template_content));
        println!(
            "{}",
            serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
        );
    }
}

pub(crate) fn render_output(
    info: &flexfetch_core::SystemInfo,
    config: &Config,
    cli: &Cli,
    ssh: bool,
) {
    match cli.format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
            );
        }
        "markdown" | "md" if !ssh => match flexfetch_core::export::export_markdown(info, config) {
            Ok(md) => print!("{md}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "ansible" => match flexfetch_core::export::export_ansible(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "terraform" => match flexfetch_core::export::export_terraform(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "csv" => match flexfetch_core::export::export_csv(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "prometheus" => match flexfetch_core::export::export_prometheus(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "github" => match flexfetch_core::export::export_github(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        _ => {
            // --ssh targets render through the full template engine even on
            // --flash (render_info legacy behavior — no flash fast-path).
            if !ssh && cli.flash && !cli.demo {
                println!("{}", flexfetch_core::template::render_flash(info));
                return;
            }
            let engine = TeraEngine::new_default();
            match engine.render(info, config) {
                Ok(output) => {
                    let out = if config.display.frame != "none" {
                        let theme = flexfetch_core::theme::resolve(config);
                        flexfetch_core::template::frame_wrap(
                            &output,
                            &config.display.frame,
                            &theme.section,
                        )
                    } else {
                        output
                    };
                    println!("{out}");
                }
                Err(e) => eprintln!("template error: {e}"),
            }
        }
    }
}

pub(crate) fn handle_export(
    info: &flexfetch_core::SystemInfo,
    config: &Config,
    format: &str,
    output: Option<&std::path::Path>,
) -> bool {
    let path = output.unwrap_or_else(|| match format {
        "svg" => std::path::Path::new("flexfetch.svg"),
        "html" => std::path::Path::new("flexfetch.html"),
        "png" => std::path::Path::new("flexfetch.png"),
        "markdown" | "md" => std::path::Path::new("flexfetch.md"),
        _ => std::path::Path::new("flexfetch.out"),
    });
    match format {
        "svg" => match flexfetch_core::export::export_svg(info, config) {
            Ok(svg) => {
                if let Err(e) = std::fs::write(path, &svg) {
                    eprintln!("write error: {e}");
                } else {
                    println!("wrote {path:?}");
                }
            }
            Err(e) => eprintln!("export error: {e}"),
        },
        "html" => match flexfetch_core::export::export_html(info, config) {
            Ok(html) => {
                if let Err(e) = std::fs::write(path, &html) {
                    eprintln!("write error: {e}");
                } else {
                    println!("wrote {path:?}");
                }
            }
            Err(e) => eprintln!("export error: {e}"),
        },
        "png" => match flexfetch_core::export::export_png(info, config, path) {
            Ok(()) => println!("wrote {path:?}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "markdown" | "md" => match flexfetch_core::export::export_markdown(info, config) {
            Ok(md) => {
                if let Err(e) = std::fs::write(path, &md) {
                    eprintln!("write error: {e}");
                } else {
                    println!("wrote {path:?}");
                }
            }
            Err(e) => eprintln!("export error: {e}"),
        },
        _ => {
            eprintln!("unknown export format: {format} (use svg, html, png, markdown)");
            return false;
        }
    }
    true
}

fn load_preset(name: &str) -> Vec<String> {
    // Reject path traversal before touching the filesystem: a preset name must
    // be a bare file stem ("neofetch", "minimal"), never a path. A hostile
    // `--preset ../../etc/x` would otherwise read arbitrary TOML files.
    if name.is_empty() || name.contains(['/', '\\']) || name.starts_with('.') || name.contains("..")
    {
        eprintln!("preset '{name}' not found, using default modules");
        return Config::default_modules();
    }

    // Check built-in presets first (via core)
    if presets::builtin_presets().contains_key(name) {
        return presets::load_preset(name);
    }

    // Check user presets (~/.config/flexfetch/presets/<name>.toml)
    let presets_dir = tools::config_dir().join("presets");
    let preset_path = presets_dir.join(format!("{name}.toml"));
    if let Ok(content) = std::fs::read_to_string(&preset_path) {
        if let Ok(doc) = toml::from_str::<toml::Value>(&content) {
            if let Some(arr) = doc.get("modules").and_then(|v| v.as_array()) {
                return arr
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
    }

    eprintln!("preset '{name}' not found, using default modules");
    Config::default_modules()
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

#[cfg(test)]
mod tests {
    use super::load_preset;

    #[test]
    fn preset_traversal_names_are_rejected() {
        // Must fall back to defaults without touching the filesystem.
        for evil in [
            "../etc/shadow",
            "/etc/passwd",
            "../../x",
            ".hidden",
            "a..b",
            "",
            "..\\win",
        ] {
            let m = load_preset(evil);
            assert!(
                !m.is_empty(),
                "preset {evil:?} should fall back to defaults, not read a file"
            );
        }
    }

    #[test]
    fn preset_clean_names_work() {
        // Valid names resolve through the builtin catalog (or warn + default).
        let m = load_preset("neofetch");
        assert!(!m.is_empty());
        let m = load_preset("minimal");
        assert!(!m.is_empty());
    }
}
