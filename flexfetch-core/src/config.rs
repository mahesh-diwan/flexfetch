use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "Config::default_modules")]
    pub modules: Vec<String>,

    #[serde(default)]
    pub plugins_dir: Option<PathBuf>,

    #[serde(default = "Config::default_template")]
    pub template: String,

    #[serde(default)]
    pub display: DisplayConfig,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub custom: HashMap<String, CustomModule>,

    #[serde(default)]
    pub modules_config: HashMap<String, ModuleConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModuleConfig {
    pub color_keys: Option<String>,
    pub color_values: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum LogoMode {
    #[default]
    Ascii,
    Block,
    Image,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DisplayConfig {
    #[serde(default = "DisplayConfig::default_separator")]
    pub separator: String,

    #[serde(default = "DisplayConfig::default_key_width")]
    pub key_width: usize,

    #[serde(default)]
    pub theme: Option<String>,

    #[serde(default)]
    pub color_title: Option<String>,

    #[serde(default)]
    pub color_keys: Option<String>,

    #[serde(default)]
    pub color_values: Option<String>,

    #[serde(default)]
    pub color_sep: Option<String>,

    #[serde(default)]
    pub gradient: bool,

    #[serde(default)]
    pub gradient_colors: Option<Vec<String>>,

    #[serde(default)]
    pub logo_mode: LogoMode,

    #[serde(default = "DisplayConfig::default_gradient_title")]
    pub gradient_title: bool,

    #[serde(default = "DisplayConfig::default_progress_bars")]
    pub progress_bars: bool,

    #[serde(default = "DisplayConfig::default_box_style")]
    pub box_style: String,

    #[serde(default = "DisplayConfig::default_pixel_logo")]
    pub pixel_logo: bool,

    #[serde(default = "DisplayConfig::default_palette_style")]
    pub palette_style: String,

    #[serde(default = "DisplayConfig::default_frame")]
    pub frame: String,

    /// Phase 7.6: render the ASCII logo as a per-line brand gradient (interpolates
    /// the theme's gradient stops across the logo's rows) when the terminal
    /// supports truecolor. Default on — fastfetch's signature look.
    #[serde(default = "DisplayConfig::default_logo_gradient")]
    pub logo_gradient: bool,

    // Icons for fastfetch-style output
    #[serde(default = "DisplayConfig::default_icon_os")]
    pub icon_os: String,
    #[serde(default = "DisplayConfig::default_icon_kernel")]
    pub icon_kernel: String,
    #[serde(default = "DisplayConfig::default_icon_host")]
    pub icon_host: String,
    #[serde(default = "DisplayConfig::default_icon_uptime")]
    pub icon_uptime: String,
    #[serde(default = "DisplayConfig::default_icon_locale")]
    pub icon_locale: String,
    #[serde(default = "DisplayConfig::default_icon_cpu")]
    pub icon_cpu: String,
    #[serde(default = "DisplayConfig::default_icon_gpu")]
    pub icon_gpu: String,
    #[serde(default = "DisplayConfig::default_icon_memory")]
    pub icon_memory: String,
    #[serde(default = "DisplayConfig::default_icon_swap")]
    pub icon_swap: String,
    #[serde(default = "DisplayConfig::default_icon_disk")]
    pub icon_disk: String,
    #[serde(default = "DisplayConfig::default_icon_network")]
    pub icon_network: String,
    #[serde(default = "DisplayConfig::default_icon_interface")]
    pub icon_interface: String,
    #[serde(default = "DisplayConfig::default_icon_resolution")]
    pub icon_resolution: String,
    #[serde(default = "DisplayConfig::default_icon_battery")]
    pub icon_battery: String,
    #[serde(default = "DisplayConfig::default_icon_processes")]
    pub icon_processes: String,
    #[serde(default = "DisplayConfig::default_icon_end")]
    pub icon_end: String,
    #[serde(default = "DisplayConfig::default_icon_temp")]
    pub icon_temp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CacheConfig {
    #[serde(default = "CacheConfig::default_ttl")]
    pub ttl: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomModule {
    pub command: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub shell: Option<String>,
}

impl DisplayConfig {
    pub fn default_separator() -> String {
        ": ".to_string()
    }
    pub fn default_key_width() -> usize {
        8
    }
    pub fn default_gradient_title() -> bool {
        true
    }
    pub fn default_progress_bars() -> bool {
        true
    }
    pub fn default_box_style() -> String {
        "rounded".into()
    }
    pub fn default_pixel_logo() -> bool {
        false
    }
    pub fn default_palette_style() -> String {
        "blocks".into()
    }
    pub fn default_frame() -> String {
        "none".into()
    }
    pub fn default_logo_gradient() -> bool {
        true
    }

    // Default Nerd Font icons (fastfetch style)
    pub fn default_icon_os() -> String {
        "󰟀 ".into() // nf-linux-archlinux or similar
    }
    pub fn default_icon_kernel() -> String {
        "󰌽 ".into() // nf-md-kernel
    }
    pub fn default_icon_host() -> String {
        "󰟀 ".into() // nf-linux-archlinux (hostname)
    }
    pub fn default_icon_uptime() -> String {
        "󰅐 ".into() // nf-md-clock-outline
    }
    pub fn default_icon_locale() -> String {
        "󰉋 ".into() // nf-md-translate
    }
    pub fn default_icon_cpu() -> String {
        "󰍛 ".into() // nf-md-cpu-64-bit
    }
    pub fn default_icon_gpu() -> String {
        "󰢮 ".into() // nf-md-expansion-card
    }
    pub fn default_icon_memory() -> String {
        "󰉀 ".into() // nf-md-memory
    }
    pub fn default_icon_swap() -> String {
        "󰯀 ".into() // nf-md-swap-horizontal
    }
    pub fn default_icon_disk() -> String {
        "󰋊 ".into() // nf-md-harddisk
    }
    pub fn default_icon_network() -> String {
        "󰩟 ".into() // nf-md-ip-network
    }
    pub fn default_icon_interface() -> String {
        "󰈀 ".into() // nf-md-lan-connect
    }
    pub fn default_icon_resolution() -> String {
        "󰍹 ".into() // nf-md-monitor
    }
    pub fn default_icon_battery() -> String {
        "󰁹 ".into() // nf-md-battery
    }
    pub fn default_icon_processes() -> String {
        "󰅩 ".into() // nf-md-process
    }
    pub fn default_icon_end() -> String {
        "󰘦 ".into() // nf-md-arrow-right-box
    }
    pub fn default_icon_temp() -> String {
        "󰏗 ".into() // nf-md-thermometer
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            separator: Self::default_separator(),
            key_width: Self::default_key_width(),
            // Phase 6: a default theme gives the output color hierarchy out of
            // the box (fastfetch/neofetch are colored by default too). Users can
            // opt out with `theme = ""` / `theme = "none"`.
            theme: Some("catppuccin".to_string()),
            color_title: None,
            color_keys: None,
            color_values: None,
            color_sep: None,
            gradient: false,
            gradient_colors: None,
            logo_mode: LogoMode::default(),
            gradient_title: Self::default_gradient_title(),
            progress_bars: Self::default_progress_bars(),
            box_style: Self::default_box_style(),
            pixel_logo: Self::default_pixel_logo(),
            palette_style: Self::default_palette_style(),
            frame: Self::default_frame(),
            logo_gradient: Self::default_logo_gradient(),
            // Icons
            icon_os: Self::default_icon_os(),
            icon_kernel: Self::default_icon_kernel(),
            icon_host: Self::default_icon_host(),
            icon_uptime: Self::default_icon_uptime(),
            icon_locale: Self::default_icon_locale(),
            icon_cpu: Self::default_icon_cpu(),
            icon_gpu: Self::default_icon_gpu(),
            icon_memory: Self::default_icon_memory(),
            icon_swap: Self::default_icon_swap(),
            icon_disk: Self::default_icon_disk(),
            icon_network: Self::default_icon_network(),
            icon_interface: Self::default_icon_interface(),
            icon_resolution: Self::default_icon_resolution(),
            icon_battery: Self::default_icon_battery(),
            icon_processes: Self::default_icon_processes(),
            icon_end: Self::default_icon_end(),
            icon_temp: Self::default_icon_temp(),
        }
    }
}

impl CacheConfig {
    pub fn default_ttl() -> u64 {
        60
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            ttl: Self::default_ttl(),
        }
    }
}

impl Config {
    pub fn default_modules() -> Vec<String> {
        vec![
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
            "cpuusage".into(),
            "cpucache".into(),
            "memory".into(),
            "gpu".into(),
            "disk".into(),
            "network".into(),
            "wifi".into(),
            "publicip".into(),
            "display".into(),
            "battery".into(),
            "locale".into(),
            "resolution".into(),
            "colors".into(),
        ]
    }

    pub fn default_template() -> String {
        "default".into()
    }

    pub fn load(path: Option<&std::path::Path>) -> crate::Result<Self> {
        // Start with defaults
        let mut config = Config::default_for_testing();

        // Layer 1: User config ($XDG_CONFIG_HOME/flexfetch/config.toml)
        if let Some(user_config) = find_user_config() {
            if let Ok(content) = std::fs::read_to_string(&user_config) {
                if let Ok(merged) = toml::from_str::<Config>(&content) {
                    config = merge_config(config, merged);
                }
            }
        }

        // Layer 2: Project config (./flexfetch.toml)
        if let Ok(cwd) = std::env::current_dir() {
            let project_config = cwd.join("flexfetch.toml");
            if project_config.exists() {
                if let Ok(content) = std::fs::read_to_string(&project_config) {
                    if let Ok(merged) = toml::from_str::<Config>(&content) {
                        config = merge_config(config, merged);
                    }
                }
            }
        }

        // Layer 3: Explicit path (CLI --config)
        if let Some(explicit) = path {
            if let Ok(content) = std::fs::read_to_string(explicit) {
                if let Ok(merged) = toml::from_str::<Config>(&content) {
                    config = merge_config(config, merged);
                }
            }
        }

        Ok(config)
    }

    pub fn default_for_testing() -> Self {
        Config {
            modules: Config::default_modules(),
            plugins_dir: None,
            template: Config::default_template(),
            display: DisplayConfig::default(),
            cache: CacheConfig::default(),
            custom: HashMap::new(),
            modules_config: HashMap::new(),
        }
    }
}

fn find_user_config() -> Option<PathBuf> {
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".config")
        });

    let p = xdg.join("flexfetch").join("config.toml");
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

fn merge_config(base: Config, override_config: Config) -> Config {
    Config {
        modules: if override_config.modules != Config::default_modules() {
            override_config.modules
        } else {
            base.modules
        },
        template: if override_config.template != "default" {
            override_config.template
        } else {
            base.template
        },
        plugins_dir: override_config.plugins_dir.or(base.plugins_dir),
        display: DisplayConfig {
            separator: if override_config.display.separator != ": " {
                override_config.display.separator
            } else {
                base.display.separator
            },
            key_width: if override_config.display.key_width != 8 {
                override_config.display.key_width
            } else {
                base.display.key_width
            },
            theme: override_config.display.theme.or(base.display.theme),
            color_title: override_config
                .display
                .color_title
                .or(base.display.color_title),
            color_keys: override_config
                .display
                .color_keys
                .or(base.display.color_keys),
            color_values: override_config
                .display
                .color_values
                .or(base.display.color_values),
            color_sep: override_config.display.color_sep.or(base.display.color_sep),
            gradient: override_config.display.gradient || base.display.gradient,
            gradient_colors: override_config
                .display
                .gradient_colors
                .or(base.display.gradient_colors),
            logo_mode: override_config.display.logo_mode,
            gradient_title: override_config.display.gradient_title,
            progress_bars: override_config.display.progress_bars,
            box_style: override_config.display.box_style,
            pixel_logo: override_config.display.pixel_logo,
            palette_style: override_config.display.palette_style,
            frame: override_config.display.frame,
            logo_gradient: override_config.display.logo_gradient,
            // Icons
            icon_os: override_config.display.icon_os,
            icon_kernel: override_config.display.icon_kernel,
            icon_host: override_config.display.icon_host,
            icon_uptime: override_config.display.icon_uptime,
            icon_locale: override_config.display.icon_locale,
            icon_cpu: override_config.display.icon_cpu,
            icon_gpu: override_config.display.icon_gpu,
            icon_memory: override_config.display.icon_memory,
            icon_swap: override_config.display.icon_swap,
            icon_disk: override_config.display.icon_disk,
            icon_network: override_config.display.icon_network,
            icon_interface: override_config.display.icon_interface,
            icon_resolution: override_config.display.icon_resolution,
            icon_battery: override_config.display.icon_battery,
            icon_processes: override_config.display.icon_processes,
            icon_end: override_config.display.icon_end,
            icon_temp: override_config.display.icon_temp,
        },
        cache: override_config.cache,
        custom: if !override_config.custom.is_empty() {
            override_config.custom
        } else {
            base.custom
        },
        modules_config: if !override_config.modules_config.is_empty() {
            override_config.modules_config
        } else {
            base.modules_config
        },
    }
}
