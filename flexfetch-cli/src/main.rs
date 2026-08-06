use clap::Parser;
use flexfetch_core::{
    get_cache_dir, Config, Context, InfoValue, ModuleRegistry, SystemInfo, TeraEngine,
};
use std::collections::HashMap;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Duration;

#[cfg(feature = "notifications")]
mod daemon;
#[cfg(feature = "history")]
mod history;
#[cfg(feature = "live")]
mod live;
#[cfg(feature = "qr")]
mod qr;
// Phase 4.12: plugin auto-loading (Lua + WASM) — only compiled when at least
// one plugin runtime is enabled.
#[cfg(any(feature = "lua", feature = "wasm-plugins"))]
mod plugins;
mod registry;
mod simd;
mod ssh;
mod tools;
#[cfg(feature = "live")]
mod wizard;
// Phase 5.5/5.6 shared health sampling (pure std; only compiled when a
// consumer feature is on so default/minimal builds stay lean).
#[cfg(any(feature = "history", feature = "notifications"))]
mod monitor;
// Phase 8.7 observability: panic hook + --bug-report dump + RUST_LOG gating.
mod telemetry;

#[derive(clap::Subcommand)]
enum Commands {
    /// Generate shell completions for the given shell
    #[cfg(feature = "completions")]
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Plugin registry (Phase 5.7): search/install/list/update Lua plugins
    /// against the hosted registry.toml (checksum + min-version verified).
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
}

/// Subcommands under `flexfetch plugin`.
#[derive(clap::Subcommand)]
enum PluginAction {
    /// Search the hosted registry by name/description
    Search { query: String },
    /// Install a plugin (SHA-256 verified, min-version gated)
    Install { name: String },
    /// List installed plugins + registry status
    List,
    /// Re-install every installed plugin still in the registry
    Update,
}

#[derive(Parser)]
#[command(name = "flexfetch", about = "Fast, flexible system info tool")]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    modules: Option<String>,

    #[arg(short, long)]
    template: Option<String>,

    #[arg(short = 'f', long, default_value = "text")]
    format: String,

    #[arg(long)]
    theme: Option<String>,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    gen_config: bool,

    #[arg(long)]
    list_modules: bool,

    #[arg(long)]
    list_presets: bool,

    /// List all built-in theme presets (Phase 7.8 — pairs with `--theme random`).
    #[arg(long)]
    list_themes: bool,

    /// Micro-benchmark: `--benchmark` (per-module timing) or `--benchmark N`
    /// (run each module N times, report min/avg/total).
    #[arg(long, num_args = 0..=1, default_missing_value = "1")]
    benchmark: Option<u64>,

    #[arg(long)]
    pipe: bool,

    #[arg(long)]
    minimal: bool,

    #[arg(long)]
    full: bool,

    #[arg(long)]
    dev: bool,

    #[arg(long)]
    preset: Option<String>,

    #[arg(long)]
    export: Option<String>,

    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    #[arg(long)]
    no_gradient: bool,

    #[arg(long)]
    no_progress: bool,

    #[arg(long)]
    box_style: Option<String>,

    #[arg(long)]
    pixel_logo: bool,

    #[arg(long)]
    palette_style: Option<String>,

    #[arg(long)]
    frame: Option<String>,

    #[arg(long)]
    watch: bool,

    #[arg(long, default_value_t = 2)]
    watch_interval: u64,

    /// Live dashboard: real-time CPU/memory gauges, top processes, network rates
    #[arg(long)]
    live: bool,

    /// Phase 5.10: record the --live dashboard to an asciinema v2 cast file
    /// (e.g. --live --record flexfetch.cast; replay with `asciinema play`).
    #[cfg(feature = "live")]
    #[arg(long)]
    record: Option<PathBuf>,

    /// SIMD CPU micro-benchmark (Phase 4.3): vectorized integer benchmark with
    /// runtime AVX2/SSE4/NEON detection, scalar fallback.
    #[arg(long)]
    bench_cpu: bool,

    /// SIMD memory-bandwidth micro-benchmark (Phase 4.3).
    #[arg(long)]
    bench_memory: bool,

    /// Smart fetch: add $PWD context (git branch/status, project type, container/venv/SSH)
    #[arg(long)]
    smart: bool,

    /// Add the system health module (score 0-100: disk/swap/load/battery)
    #[arg(long)]
    health: bool,

    /// Single-line prompt string (e.g. `🐧 arch | CPU 12% | RAM 3.2G`)
    #[arg(long)]
    prompt: bool,

    /// Plain-text banner (ANSI colors stripped) for MOTD/startup
    #[arg(long)]
    motd: bool,

    /// Fetch remote system info via SSH (repeatable, parallel)
    #[arg(long)]
    ssh: Vec<String>,

    /// Diff mode (Phase 4.9): compare two systems side-by-side. Each target is
    /// `local`, `host@remote`, or a path to a flexfetch JSON export file.
    #[arg(long, num_args = 2)]
    diff: Vec<String>,

    /// Interactive config wizard (writes ~/.config/flexfetch/config.toml)
    #[arg(long)]
    wizard: bool,

    /// Render the effective config as a terminal QR code (base64+zstd payload,
    /// unicode blocks). Scan it with a phone to import on another machine.
    #[arg(long)]
    qr: bool,

    /// Import a config from a QR-code image (PNG/etc; decoded via rqrr) and
    /// write it to the config path (existing file is backed up).
    #[arg(long)]
    import_qr: Option<PathBuf>,

    /// Self-update: check the latest GitHub release and re-run the install
    /// script if a newer version exists (requires curl).
    #[arg(long)]
    update: bool,

    /// Environment doctor: validate terminal, color, config, and collectors.
    #[arg(long)]
    doctor: bool,

    /// Print a shell hook (bash|zsh|fish) for cd-into-git-repo context fetches.
    #[arg(long)]
    hook: Option<String>,

    /// Print a tmux.conf snippet that auto-runs the fetch in new idle panes
    /// (Phase 5.3 — pair with the bundled `flexfetch-tmux` helper binary).
    #[arg(long)]
    tmux_config: bool,

    /// Refresh the crowdsourced hardware database (Phase 5.8): downloads the
    /// latest PCI/USB name map to the cache dir; falls back to the bundled
    /// seed when offline.
    #[arg(long)]
    update_db: bool,

    /// Phase 5.4: derive the theme from the wallpaper's dominant colors
    /// (requires the `auto-theme` feature; falls back to catppuccin otherwise).
    #[arg(long)]
    auto_theme: bool,

    /// Phase 5.5: record cpu/mem/disk/temp snapshots to history.db every
    /// --history-interval seconds until Ctrl+C (requires the `history` feature).
    #[arg(long)]
    history: bool,

    /// Phase 5.5: record interval for --history / --daemon (seconds).
    #[arg(long, default_value_t = 60)]
    history_interval: u64,

    /// Phase 5.5: print an ASCII sparkline of the recorded metric over the
    /// last --hours (cpu|memory|disk|temp; requires the `history` feature).
    #[arg(long)]
    history_graph: Option<String>,

    /// Phase 5.5: window for --history-graph, in hours.
    #[arg(long, default_value_t = 24)]
    hours: u64,

    /// Phase 5.5: export the snapshots table to a CSV file.
    #[arg(long)]
    history_export: Option<PathBuf>,

    /// Phase 5.6: critical health notifications daemon — poll every
    /// --history-interval seconds, notify on threshold breach (requires the
    /// `notifications` feature).
    #[arg(long)]
    daemon: bool,

    /// Phase 5.6: threshold overrides, e.g. --threshold cpu=95,mem=88,temp=80.
    #[arg(long)]
    threshold: Vec<String>,

    /// Phase 8.8: showcase mode — every module + every visual feature, for
    /// screenshots / social previews / `install.sh` first-run demos.
    #[arg(long)]
    demo: bool,

    /// Phase 8.7: print a full environment/version dump for bug reports
    /// (version, OS, kernel, terminal, shell, config, module errors).
    #[arg(long)]
    bug_report: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// Parse the `--history-graph` metric argument (cpu|memory|disk|temp).
#[cfg(feature = "history")]
fn parse_history_metric(s: &str) -> Option<history::Metric> {
    match s.trim().to_lowercase().as_str() {
        "cpu" => Some(history::Metric::Cpu),
        "memory" | "mem" => Some(history::Metric::Memory),
        "disk" => Some(history::Metric::Disk),
        "temp" | "temperature" => Some(history::Metric::Temp),
        _ => None,
    }
}

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
        #[cfg(feature = "lua")]
        features.push("lua");
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
        #[cfg(feature = "history")]
        features.push("history");
        #[cfg(feature = "notifications")]
        features.push("notifications");
        #[cfg(feature = "wasm-plugins")]
        features.push("wasm-plugins");
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

    // Subcommands: `completions <shell>` (clap_complete) and `plugin …`
    // (registry). Dispatch happens before config load — neither needs one.
    if let Some(command) = &cli.command {
        match command {
            #[cfg(feature = "completions")]
            Commands::Completions { shell } => {
                use clap::CommandFactory;
                let mut cmd = Cli::command();
                clap_complete::generate(*shell, &mut cmd, "flexfetch", &mut std::io::stdout());
            }
            Commands::Plugin { action } => registry::run(action),
        }
        return;
    }

    if cli.gen_config {
        generate_config();
        return;
    }

    if cli.list_modules {
        list_modules();
        return;
    }

    if cli.list_presets {
        list_presets();
        return;
    }

    // --list-themes: show every built-in theme preset name (Phase 7.8).
    if cli.list_themes {
        for name in flexfetch_core::theme::preset_names() {
            println!("{name}");
        }
        return;
    }

    // --hook <shell>: print a cd-into-git-repo hook for the given shell.
    if let Some(ref shell) = cli.hook {
        tools::print_hook(shell);
        return;
    }

    // --update: re-run the install script if a newer release exists.
    if cli.update {
        tools::self_update();
        return;
    }

    // --tmux-config: print the tmux.conf snippet (pure output, no deps).
    if cli.tmux_config {
        tools::print_tmux_config();
        return;
    }

    // --update-db: refresh the crowdsourced hardware database (needs curl).
    if cli.update_db {
        match flexfetch_core::hardware_db::refresh() {
            Ok(msg) => println!("{msg}"),
            Err(e) => {
                eprintln!("update-db: {e}");
                std::process::exit(1);
            }
        }
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

    let config_dir = get_config_dir();
    let cache_dir = get_cache_dir();

    let config_path = cli.config.as_ref().map(std::path::Path::new);
    let mut config = Config::load(config_path).unwrap_or_else(|_| Config::default_for_testing());

    // `mut`: watch mode rebuilds ctx on config hot-reload.
    let mut ctx = Context::new(
        config_dir.clone(),
        cache_dir.clone(),
        cli.debug,
        config.custom.clone(),
    );

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
            if let Err(e) = live::run(ctx, watch_path, cli.record.clone()) {
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

    // Phase 5.5 --history-graph: ASCII sparkline of a recorded metric.
    if cli.history_graph.is_some() {
        #[cfg(feature = "history")]
        {
            let metric_arg = cli.history_graph.as_deref().unwrap_or("");
            let metric = match parse_history_metric(metric_arg) {
                Some(m) => m,
                None => {
                    eprintln!(
                        "error: --history-graph expects cpu|memory|disk|temp (got {metric_arg:?})"
                    );
                    std::process::exit(1);
                }
            };
            match history::open(&ctx.cache_dir) {
                Ok(conn) => {
                    if let Err(e) = history::graph(&conn, metric, cli.hours as i64) {
                        eprintln!("history error: {e}");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("history error: {e}");
                    std::process::exit(1);
                }
            }
            return;
        }
        #[cfg(not(feature = "history"))]
        {
            eprintln!(
                "error: --history-graph requires the `history` feature (build with --features history)"
            );
            std::process::exit(1);
        }
    }

    // Phase 5.5 --history-export: dump the snapshots table to CSV.
    if cli.history_export.is_some() {
        #[cfg(feature = "history")]
        {
            let path = cli.history_export.as_deref().unwrap_or_else(|| {
                eprintln!("error: --history-export requires a file path");
                std::process::exit(1);
            });
            match history::open(&ctx.cache_dir) {
                Ok(conn) => {
                    if let Err(e) = history::export_csv(&conn, path) {
                        eprintln!("history error: {e}");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("history error: {e}");
                    std::process::exit(1);
                }
            }
            return;
        }
        #[cfg(not(feature = "history"))]
        {
            eprintln!(
                "error: --history-export requires the `history` feature (build with --features history)"
            );
            std::process::exit(1);
        }
    }

    // Phase 5.6 --daemon: critical health notifications until Ctrl+C.
    if cli.daemon {
        #[cfg(feature = "notifications")]
        {
            let thresholds = daemon::Thresholds::new(&cli.threshold);
            daemon::run(ctx, cli.history_interval, thresholds);
            return;
        }
        #[cfg(not(feature = "notifications"))]
        {
            eprintln!(
                "error: --daemon requires the `notifications` feature (build with --features notifications)"
            );
            std::process::exit(1);
        }
    }

    // Phase 5.5 --history: record snapshots to history.db until Ctrl+C.
    if cli.history {
        #[cfg(feature = "history")]
        {
            if let Err(e) = history::record_loop(ctx, cli.history_interval) {
                eprintln!("history error: {e}");
                std::process::exit(1);
            }
            return;
        }
        #[cfg(not(feature = "history"))]
        {
            eprintln!(
                "error: --history requires the `history` feature (build with --features history)"
            );
            std::process::exit(1);
        }
    }

    // Pipe detection. `--demo` always renders full color (it's for screenshots/
    // social previews, where stdout is usually a pipe).
    let is_tty = std::io::stdout().is_terminal();
    let pipe_mode = if cli.demo { false } else { cli.pipe || !is_tty };

    // Module toggle groups and presets
    let mut modules = resolve_modules(&cli, &config);

    // Pipe mode overrides
    apply_cli_overrides(&cli, &mut config, pipe_mode);

    let registry = ModuleRegistry::get();
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
                Ok(info) => render_info(info, &config, &cli),
                Err(e) => eprintln!("  error: {e}"),
            }
            println!();
        }
        return;
    }

    // --bench-cpu / --bench-memory: SIMD micro-benchmarks (Phase 4.3)
    if cli.bench_cpu || cli.bench_memory {
        let iterations = cli.benchmark.unwrap_or(5).max(1);
        if cli.bench_cpu {
            let (level, best) = simd::bench_cpu(iterations);
            println!("CPU SIMD benchmark ({iterations} iterations):");
            println!("  path:  {level:?}");
            println!("  best:  {:?}", best);
        }
        if cli.bench_memory {
            let (gb_s, best) = simd::bench_memory(iterations);
            println!("Memory write bandwidth ({iterations} iterations, 64 MiB buffer):");
            println!("  best:  {gb_s:.2} GiB/s ({:?})", best);
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

    // `mut`: only the plugin merge (lua/wasm-plugins features) mutates `info`;
    // the minimal build has neither, so silence the lint there.
    #[allow(unused_mut)]
    let mut info = registry.run_selected(&modules, &ctx, template_content);

    // Phase 4.12: auto-load Lua/WASM plugins from the plugins dir and merge
    // them in as a `plugins` table (rendered by the template's Plugins block).
    #[cfg(any(feature = "lua", feature = "wasm-plugins"))]
    if let Some(plugins) = plugins::collect_plugins(&ctx) {
        info.add("plugins", plugins);
    }

    // Handle --export flag
    if let Some(ref format) = cli.export {
        handle_export(&info, &config, format, cli.output.as_deref());
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
            // `mut`: only the plugin merge (lua/wasm-plugins features) mutates
            // `fresh`; the minimal build has neither, so silence the lint.
            #[allow(unused_mut)]
            let mut fresh =
                registry.run_selected_cached(&modules, &ctx, template_content, &mut snapshot);
            // Phase 4.12: plugins are static per session (like the cached
            // modules) — run once, serve from the snapshot on later ticks.
            #[cfg(any(feature = "lua", feature = "wasm-plugins"))]
            {
                if let Some(cached) = snapshot.get("plugins") {
                    if !cached.is_empty() {
                        fresh.add("plugins", cached.clone());
                    }
                } else if let Some(plugins) = plugins::collect_plugins(&ctx) {
                    snapshot.insert("plugins".into(), plugins.clone());
                    fresh.add("plugins", plugins);
                }
            }
            render_output(&fresh, &config, &cli);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            std::thread::sleep(Duration::from_secs(cli.watch_interval));
        }
        println!();
        return;
    }

    render_output(&info, &config, &cli);
}

/// Resolve the module list from CLI flags/presets/config (shared by the main
/// path and watch-mode config hot-reload).
fn resolve_modules(cli: &Cli, config: &Config) -> Vec<String> {
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
    let mut modules: Vec<String> = if cli.minimal {
        module_group("minimal")
    } else if cli.full {
        module_group("full")
    } else if cli.dev {
        module_group("dev")
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
    if cli.pixel_logo {
        config.display.pixel_logo = true;
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
    let default = get_config_dir().join("config.toml");
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
        let va = a_map
            .get(name)
            .map(|v| info_value_summary(v))
            .unwrap_or_default();
        let vb = b_map
            .get(name)
            .map(|v| info_value_summary(v))
            .unwrap_or_default();
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

/// Compact one-line summary of an InfoValue for the diff table.
fn info_value_summary(v: &InfoValue) -> String {
    match v {
        InfoValue::Scalar(s) => s.clone(),
        InfoValue::Map(m) => {
            let mut parts: Vec<String> = m.iter().map(|(k, val)| format!("{k}={val}")).collect();
            parts.sort();
            parts.join(", ")
        }
        InfoValue::List(l) => l.join(", "),
        InfoValue::Table(t) => format!("{} rows", t.len()),
    }
}

fn render_info(info: &SystemInfo, config: &Config, cli: &Cli) {
    match cli.format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
            );
        }
        "ansible" => match flexfetch_core::export::export_ansible(info) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "terraform" => match flexfetch_core::export::export_terraform(info) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "csv" => match flexfetch_core::export::export_csv(info) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "prometheus" => match flexfetch_core::export::export_prometheus(info) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "github" => match flexfetch_core::export::export_github(info) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        _ => {
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

fn render_output(info: &flexfetch_core::SystemInfo, config: &Config, cli: &Cli) {
    match cli.format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
            );
        }
        "markdown" | "md" => match flexfetch_core::export::export_markdown(info, config) {
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

fn handle_export(
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

fn module_group(name: &str) -> Vec<String> {
    match name {
        "minimal" => vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
        ],
        "full" => Config::default_modules(),
        "dev" => vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "shell".into(),
            "terminal".into(),
        ],
        _ => Config::default_modules(),
    }
}

fn builtin_presets() -> HashMap<String, Vec<String>> {
    let mut presets = HashMap::new();
    presets.insert("default".into(), Config::default_modules());
    presets.insert("minimal".into(), module_group("minimal"));
    presets.insert("full".into(), module_group("full"));
    presets.insert("dev".into(), module_group("dev"));
    presets.insert(
        "server".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "network".into(),
        ],
    );
    presets.insert(
        "laptop".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "battery".into(),
            "shell".into(),
        ],
    );
    presets.insert(
        "ci".into(),
        vec![
            "os".into(),
            "kernel".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "network".into(),
        ],
    );
    presets.insert(
        "neofetch".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "host".into(),
            "kernel".into(),
            "uptime".into(),
            "packages".into(),
            "shell".into(),
            "de".into(),
            "wm".into(),
            "terminal".into(),
            "cpu".into(),
            "gpu".into(),
            "memory".into(),
            "disk".into(),
            "battery".into(),
            "colors".into(),
        ],
    );
    presets
}

fn load_preset(name: &str) -> Vec<String> {
    // Check built-in presets first
    if let Some(modules) = builtin_presets().get(name) {
        return modules.clone();
    }

    // Check user presets (~/.config/flexfetch/presets/<name>.toml)
    let presets_dir = get_config_dir().join("presets");
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

fn list_presets() {
    let builtins = builtin_presets();
    println!("Built-in presets:");
    for (name, modules) in &builtins {
        let list: Vec<&str> = modules.iter().map(|s| s.as_str()).collect();
        println!("  {name:12} {}", list.join(", "));
    }

    // Check user presets directory
    let presets_dir = get_config_dir().join("presets");
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

fn get_config_dir() -> std::path::PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            std::path::PathBuf::from(home).join(".config")
        })
        .join("flexfetch")
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

/// Write an imported config to the target path — `--config PATH` if given, else
/// the default `~/.config/flexfetch/config.toml` — backing up any existing file
/// first (timestamped, so repeated imports never clobber an older backup).
#[cfg(feature = "qr")]
fn write_imported_config(cli: &Cli, toml_str: &str) {
    let path = cli
        .config
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| get_config_dir().join("config.toml"));
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

fn list_modules() {
    let builtins = [
        "os",
        "host",
        "kernel",
        "uptime",
        "locale",
        "cpu",
        "cpucache",
        "cpuusage",
        "memory",
        "swap",
        "disk",
        "gpu",
        "network",
        "dns",
        "display",
        "bluetooth",
        "media",
        "battery",
        "temperature",
        "processes",
        "packages",
        "shell",
        "terminal",
        "de",
        "wm",
        "colors",
        "custom",
        "publicip",
        "wifi",
        "git",
        "project",
        "context",
        "health",
        "weather",
        "container",
        "wallpaper",
        "fsdeep",
    ];
    println!("Built-in modules:");
    for m in builtins {
        println!("  {m}");
    }
    println!("\nLayout directives (template-only): title, separator");
    println!("\nPlugins: place .lua files in ~/.config/flexfetch/plugins/");
    #[cfg(feature = "wasm-plugins")]
    println!("         place .wasm files in ~/.config/flexfetch/plugins/ (build with --features wasm-plugins)");
}
