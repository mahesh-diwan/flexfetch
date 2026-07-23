use clap::Parser;
use flexfetch_core::{Config, Context, ModuleRegistry, TeraEngine};
use std::collections::HashMap;
use std::io::IsTerminal;

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

    #[arg(long)]
    benchmark: bool,

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
    list_presets: bool,
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

    if cli.list_presets {
        list_presets();
        return;
    }

    let config_dir = get_config_dir();
    let cache_dir = get_cache_dir();

    let config_path = cli.config.as_ref().map(|s| std::path::Path::new(s));
    let mut config = Config::load(config_path).unwrap_or_else(|_| Config::default_for_testing());

    let ctx = Context::new(
        config_dir.clone(),
        cache_dir,
        cli.debug,
        config.custom.clone(),
    );

    if let Some(theme) = cli.theme {
        config.display.theme = Some(theme);
    }

    // Pipe detection
    let is_tty = std::io::stdout().is_terminal();
    let pipe_mode = cli.pipe || !is_tty;

    // Module toggle groups and presets
    let modules: Vec<String> = if cli.minimal {
        module_group("minimal")
    } else if cli.full {
        module_group("full")
    } else if cli.dev {
        module_group("dev")
    } else if let Some(ref preset_name) = cli.preset {
        load_preset(preset_name)
    } else if let Some(m) = cli.modules {
        m.split(':').map(|s| s.to_string()).collect()
    } else {
        config.modules.clone()
    };

    // Pipe mode overrides
    if pipe_mode {
        config.display.theme = Some("none".into());
    }

    let registry = ModuleRegistry::get();
    let template_content = TeraEngine::default_template_content();

    if cli.benchmark {
        let t0 = std::time::Instant::now();
        let t1 = std::time::Instant::now();
        let mut timings = Vec::new();
        for name in &modules {
            if name == "title" || name == "separator" {
                continue;
            }
            let t = std::time::Instant::now();
            let _ = registry.run_individual(name, &ctx);
            timings.push((name.clone(), t.elapsed()));
        }
        timings.sort_by(|a, b| b.1.cmp(&a.1));
        let t2 = std::time::Instant::now();
        let info = registry.run_selected(&modules, &ctx, template_content);
        let run_selected_dur = t2.elapsed();
        let engine = TeraEngine::new_default();
        let t3 = std::time::Instant::now();
        let rendered = engine.render(&info, &config);
        let render_dur = t3.elapsed();
        eprintln!("--- flexfetch benchmark ---");
        eprintln!("  registry init:   {:?}", t1.elapsed());
        for (name, dur) in &timings {
            eprintln!("  {name:15} {dur:?}");
        }
        eprintln!("  run_selected:    {run_selected_dur:?}");
        eprintln!("  template render: {render_dur:?}");
        eprintln!("  total:           {:?}", t0.elapsed());
        eprintln!("---");
        match cli.format.as_str() {
            "json" => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
                );
            }
            _ => match rendered {
                Ok(output) => println!("{output}"),
                Err(e) => eprintln!("template error: {e}"),
            },
        }
        return;
    }

    let info = registry.run_selected(&modules, &ctx, template_content);

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
