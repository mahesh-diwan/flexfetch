use clap::Parser;
use flexfetch_core::{
    get_cache_dir, Config, Context, InfoValue, ModuleRegistry, SystemInfo, TeraEngine,
};
use std::collections::HashMap;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::Duration;

#[cfg(feature = "live")]
mod live;
mod ssh;
#[cfg(feature = "live")]
mod wizard;

#[cfg(feature = "completions")]
#[derive(clap::Subcommand)]
enum Commands {
    /// Generate shell completions for the given shell
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
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

    /// Interactive config wizard (writes ~/.config/flexfetch/config.toml)
    #[arg(long)]
    wizard: bool,

    #[cfg(feature = "completions")]
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
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
        println!(
            "flexfetch {}\nFeatures: {}",
            env!("CARGO_PKG_VERSION"),
            features.join(", ")
        );
        return;
    }

    let cli = Cli::parse();

    // `completions <shell>` subcommand (clap_complete)
    #[cfg(feature = "completions")]
    if let Some(Commands::Completions { shell }) = &cli.command {
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        clap_complete::generate(*shell, &mut cmd, "flexfetch", &mut std::io::stdout());
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

    // Pipe detection
    let is_tty = std::io::stdout().is_terminal();
    let pipe_mode = cli.pipe || !is_tty;

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
        benchmark(&modules, &ctx, registry, template_content, &config, &cli);
        return;
    }

    let info = registry.run_selected(&modules, &ctx, template_content);

    // Handle --export flag
    if let Some(ref format) = cli.export {
        handle_export(&info, &config, format, cli.output.as_deref());
        return;
    }

    if cli.watch {
        // Watch mode: refresh every N seconds, hot-reloading the config file
        // when it changes (mtime-based, no extra dependency).
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let r = running.clone();
        let _ = ctrlc::set_handler(move || {
            r.store(false, std::sync::atomic::Ordering::SeqCst);
        });
        let config_file = config_file_path(&cli);
        let mut last_mtime = config_file.as_deref().and_then(file_mtime);
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
                    eprintln!("\n[flexfetch] config reloaded\n");
                }
            }
            print!("\x1b[2J\x1b[H");
            let fresh = registry.run_selected(&modules, &ctx, template_content);
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

fn render_info(info: &SystemInfo, config: &Config, cli: &Cli) {
    match cli.format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
            );
        }
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
    ];
    println!("Built-in modules:");
    for m in builtins {
        println!("  {m}");
    }
    println!("\nLayout directives (template-only): title, separator");
    println!("\nPlugins: place .lua files in ~/.config/flexfetch/plugins/");
}
