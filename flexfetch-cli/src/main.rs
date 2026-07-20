use clap::Parser;
use flexfetch_core::{Config, Context, ModuleRegistry, TeraEngine};

#[derive(Parser)]
#[command(name = "flexfetch", version, about = "Fast, flexible system info tool")]
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

    let config_dir = get_config_dir();
    let cache_dir = get_cache_dir();
    let ctx = Context::new(config_dir.clone(), cache_dir, cli.debug);

    let config_path = cli.config.as_ref().map(|s| std::path::Path::new(s));
    let mut config = Config::load(config_path).unwrap_or_else(|_| Config::default_for_testing());

    if let Some(theme) = cli.theme {
        config.display.theme = Some(theme);
    }

    let modules: Vec<String> = if let Some(m) = cli.modules {
        m.split(':').map(|s| s.to_string()).collect()
    } else {
        config.modules.clone()
    };

    let registry = ModuleRegistry::new(&config);
    let info = registry.run_selected(&modules, &ctx);

    match cli.format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
            );
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
        "os",
        "host",
        "kernel",
        "uptime",
        "locale",
        "cpu",
        "memory",
        "disk",
        "gpu",
        "network",
        "battery",
        "processes",
        "packages",
        "shell",
        "terminal",
        "de",
        "wm",
        "colors",
        "custom",
    ];
    println!("Built-in modules:");
    for m in builtins {
        println!("  {m}");
    }
    println!("\nLayout directives (template-only): title, separator");
    println!("\nPlugins: place .lua files in ~/.config/flexfetch/plugins/");
}
