use clap::Parser;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Generate shell completions for the given shell
    #[cfg(feature = "completions")]
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Parser)]
#[command(name = "flexfetch", about = "Fast, flexible system info tool")]
pub struct Cli {
    #[arg(short, long)]
    pub config: Option<String>,

    #[arg(short, long)]
    pub modules: Option<String>,

    #[arg(short, long)]
    pub template: Option<String>,

    #[arg(short = 'f', long, default_value = "text")]
    pub format: String,

    #[arg(long)]
    pub theme: Option<String>,

    #[arg(long)]
    pub debug: bool,

    #[arg(long)]
    pub gen_config: bool,

    #[arg(long)]
    pub list_modules: bool,

    #[arg(long)]
    pub list_presets: bool,

    /// List all built-in theme presets (Phase 7.8 — pairs with `--theme random`).
    #[arg(long)]
    pub list_themes: bool,

    /// Micro-benchmark: `--benchmark` (per-module timing) or `--benchmark N`
    /// (run each module N times, report min/avg/total).
    #[arg(long, num_args = 0..=1, default_missing_value = "1")]
    pub benchmark: Option<u64>,

    #[arg(long)]
    pub pipe: bool,

    #[arg(long)]
    pub minimal: bool,

    /// Fast path (like fastfetch's `flashfetch`): baked-in defaults, minimal
    /// module set, no config file read, no template engine — the fastest
    /// possible one-shot fetch. Only affects the plain terminal render; other
    /// modes (--export, --watch, --live, --wizard, --ssh, --diff, --prompt,
    /// --motd, --benchmark, --demo) keep their existing behavior.
    #[arg(long)]
    pub flash: bool,

    #[arg(long)]
    pub full: bool,

    #[arg(long)]
    pub dev: bool,

    #[arg(long)]
    pub preset: Option<String>,

    #[arg(long)]
    pub export: Option<String>,

    #[arg(short = 'o', long)]
    pub output: Option<std::path::PathBuf>,

    #[arg(long)]
    pub no_gradient: bool,

    #[arg(long)]
    pub no_progress: bool,

    #[arg(long)]
    pub box_style: Option<String>,

    #[arg(long)]
    pub palette_style: Option<String>,

    #[arg(long)]
    pub frame: Option<String>,

    #[arg(long)]
    pub watch: bool,

    #[arg(long, default_value_t = 2)]
    pub watch_interval: u64,

    /// Live dashboard: real-time CPU/memory gauges, top processes, network rates
    #[arg(long)]
    pub live: bool,

    /// Smart fetch: add $PWD context (git branch/status, project type, container/venv/SSH)
    #[arg(long)]
    pub smart: bool,

    /// Add the system health module (score 0-100: disk/swap/load/battery)
    #[arg(long)]
    pub health: bool,

    /// Single-line prompt string (e.g. `🐧 arch | CPU 12% | RAM 3.2G`)
    #[arg(long)]
    pub prompt: bool,

    /// Plain-text banner (ANSI colors stripped) for MOTD/startup
    #[arg(long)]
    pub motd: bool,

    /// Fetch remote system info via SSH (repeatable, parallel)
    #[arg(long)]
    pub ssh: Vec<String>,

    /// Diff mode (Phase 4.9): compare two systems side-by-side. Each target is
    /// `local`, `host@remote`, or a path to a flexfetch JSON export file.
    #[arg(long, num_args = 2)]
    pub diff: Vec<String>,

    /// Interactive config wizard (writes ~/.config/flexfetch/config.toml)
    #[arg(long)]
    pub wizard: bool,

    /// Render the effective config as a terminal QR code (base64+zstd payload,
    /// unicode blocks). Scan it with a phone to import on another machine.
    #[arg(long)]
    pub qr: bool,

    /// Import a config from a QR-code image (PNG/etc; decoded via rqrr) and
    /// write it to the config path (existing file is backed up).
    #[arg(long)]
    pub import_qr: Option<std::path::PathBuf>,

    /// Self-update: check the latest GitHub release and re-run the install
    /// script if a newer version exists (requires curl).
    #[arg(long)]
    pub update: bool,

    /// Environment doctor: validate terminal, color, config, and collectors.
    #[arg(long)]
    pub doctor: bool,

    /// Print a shell hook (bash|zsh|fish) for cd-into-git-repo context fetches.
    #[arg(long)]
    pub hook: Option<String>,

    /// Refresh the crowdsourced hardware database (Phase 5.8): downloads the
    /// latest PCI/USB name map to the cache dir; falls back to the bundled
    /// seed when offline.
    #[arg(long)]
    pub update_db: bool,

    /// Phase 5.4: derive the theme from the wallpaper's dominant colors
    /// (requires the `auto-theme` feature; falls back to catppuccin otherwise).
    #[arg(long)]
    pub auto_theme: bool,

    /// Phase 8.8: showcase mode — every module + every visual feature, for
    /// screenshots / social previews / `install.sh` first-run demos.
    #[arg(long)]
    pub demo: bool,

    /// Phase 8.7: print a full environment/version dump for bug reports
    /// (version, OS, kernel, terminal, shell, config, module errors).
    #[arg(long)]
    pub bug_report: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
