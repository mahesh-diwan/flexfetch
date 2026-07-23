use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ThemeStrings {
    pub title: String,
    pub keys: String,
    pub values: String,
    pub sep: String,
    pub section: String,
    pub reset: &'static str,
    pub gradient: bool,
    pub gradient_colors: Vec<[u8; 3]>,
}

struct Theme {
    title: &'static str,
    keys: &'static str,
    values: &'static str,
    sep: &'static str,
    section: &'static str,
    reset: &'static str,
}

const RESET: &str = "\x1b[0m";

const NONE: Theme = Theme {
    title: "",
    keys: "",
    values: "",
    sep: "",
    section: "",
    reset: "",
};
const CATPPUCCIN: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[94m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    section: "\x1b[1;94m",
    reset: RESET,
};
const DRACULA: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[95m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    section: "\x1b[1;96m",
    reset: RESET,
};
const NORD: Theme = Theme {
    title: "\x1b[1;94m",
    keys: "\x1b[94m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;92m",
    reset: RESET,
};
const GRUVBOX: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[93m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
};
const TOKYO_NIGHT: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[94m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
};
const SOLARIZED_DARK: Theme = Theme {
    title: "\x1b[1;33m",
    keys: "\x1b[36m",
    values: "\x1b[34m",
    sep: "\x1b[90m",
    section: "\x1b[1;33m",
    reset: RESET,
};
const SOLARIZED_LIGHT: Theme = Theme {
    title: "\x1b[1;31m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;31m",
    reset: RESET,
};
const ROSE_PINE: Theme = Theme {
    title: "\x1b[1;36m",
    keys: "\x1b[36m",
    values: "\x1b[35m",
    sep: "\x1b[90m",
    section: "\x1b[1;36m",
    reset: RESET,
};
const ROSE_PINE_DAWN: Theme = Theme {
    title: "\x1b[1;34m",
    keys: "\x1b[36m",
    values: "\x1b[35m",
    sep: "\x1b[90m",
    section: "\x1b[1;34m",
    reset: RESET,
};
const EVERFOREST_DARK: Theme = Theme {
    title: "\x1b[1;32m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;32m",
    reset: RESET,
};
const EVERFOREST_LIGHT: Theme = Theme {
    title: "\x1b[1;32m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;32m",
    reset: RESET,
};
const BAMBOO: Theme = Theme {
    title: "\x1b[1;31m",
    keys: "\x1b[32m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;31m",
    reset: RESET,
};
const OXOCARBON_DARK: Theme = Theme {
    title: "\x1b[1;36m",
    keys: "\x1b[36m",
    values: "\x1b[35m",
    sep: "\x1b[90m",
    section: "\x1b[1;36m",
    reset: RESET,
};
const ONE_DARK: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[31m",
    values: "\x1b[32m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
};
const ONE_LIGHT: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[31m",
    values: "\x1b[32m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
};
const TOKYO_NIGHT_STORM: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
};
const CATPPUCCIN_MOCHA: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
};
const CATPPUCCIN_FRAPPE: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
};
const CATPPUCCIN_MACCHIATO: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
};
const MONOKAI: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;92m",
    values: "\x1b[91m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
};
const MONOKAI_PRO: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[1;96m",
    values: "\x1b[93m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
};
const AYU_DARK: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
};
const AYU_MIRAGE: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[1;96m",
    values: "\x1b[93m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
};
const PALENIGHT: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
};
const MATERIAL_OCEAN: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;96m",
    values: "\x1b[91m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
};
const KANAGAWA: Theme = Theme {
    title: "\x1b[1;91m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;91m",
    reset: RESET,
};
const MELLOW_PURPLE: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
};

pub fn resolve_ansi(code_or_name: &str) -> String {
    if code_or_name.starts_with('\x1b') || code_or_name.starts_with("\\u001b") {
        return code_or_name.to_string();
    }
    match code_or_name.to_lowercase().as_str() {
        "black" => "\x1b[30m",
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "magenta" => "\x1b[35m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        "bright-black" | "gray" => "\x1b[90m",
        "bright-red" => "\x1b[91m",
        "bright-green" => "\x1b[92m",
        "bright-yellow" => "\x1b[93m",
        "bright-blue" => "\x1b[94m",
        "bright-magenta" | "pink" => "\x1b[95m",
        "bright-cyan" => "\x1b[96m",
        "bright-white" => "\x1b[97m",
        "bold" => "\x1b[1m",
        _ => "",
    }
    .to_string()
}

pub fn resolve(config: &Config) -> ThemeStrings {
    let preset = match config.display.theme.as_deref().unwrap_or("") {
        "catppuccin" => &CATPPUCCIN,
        "dracula" => &DRACULA,
        "nord" => &NORD,
        "gruvbox" => &GRUVBOX,
        "tokyo-night" => &TOKYO_NIGHT,
        "solarized-dark" => &SOLARIZED_DARK,
        "solarized-light" => &SOLARIZED_LIGHT,
        "rose-pine" => &ROSE_PINE,
        "rose-pine-dawn" => &ROSE_PINE_DAWN,
        "everforest-dark" => &EVERFOREST_DARK,
        "everforest-light" => &EVERFOREST_LIGHT,
        "bamboo" => &BAMBOO,
        "oxocarbon-dark" => &OXOCARBON_DARK,
        "one-dark" => &ONE_DARK,
        "one-light" => &ONE_LIGHT,
        "tokyo-night-storm" => &TOKYO_NIGHT_STORM,
        "catppuccin-mocha" => &CATPPUCCIN_MOCHA,
        "catppuccin-frappe" => &CATPPUCCIN_FRAPPE,
        "catppuccin-macchiato" => &CATPPUCCIN_MACCHIATO,
        "monokai" => &MONOKAI,
        "monokai-pro" => &MONOKAI_PRO,
        "ayu-dark" => &AYU_DARK,
        "ayu-mirage" => &AYU_MIRAGE,
        "palenight" => &PALENIGHT,
        "material-ocean" => &MATERIAL_OCEAN,
        "kanagawa" => &KANAGAWA,
        "mellow-purple" => &MELLOW_PURPLE,
        _ => &NONE,
    };

    let gradient_colors = config
        .display
        .gradient_colors
        .as_deref()
        .map(|cs| cs.iter().filter_map(|c| parse_hex_color(c)).collect())
        .unwrap_or_default();

    ThemeStrings {
        title: config
            .display
            .color_title
            .as_deref()
            .map(resolve_ansi)
            .unwrap_or_else(|| preset.title.to_string()),
        keys: config
            .display
            .color_keys
            .as_deref()
            .map(resolve_ansi)
            .unwrap_or_else(|| preset.keys.to_string()),
        values: config
            .display
            .color_values
            .as_deref()
            .map(resolve_ansi)
            .unwrap_or_else(|| preset.values.to_string()),
        sep: config
            .display
            .color_sep
            .as_deref()
            .map(resolve_ansi)
            .unwrap_or_else(|| preset.sep.to_string()),
        section: preset.section.to_string(),
        reset: preset.reset,
        gradient: config.display.gradient,
        gradient_colors,
    }
}

fn parse_hex_color(s: &str) -> Option<[u8; 3]> {
    let hex = s.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some([r, g, b])
}

pub fn gradient_text(text: &str, start: [u8; 3], end: [u8; 3]) -> String {
    let len = text.chars().count();
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let t = if len <= 1 {
                0.0
            } else {
                i as f64 / (len - 1) as f64
            };
            let r = (start[0] as f64 + t * (end[0] as f64 - start[0] as f64)) as u8;
            let g = (start[1] as f64 + t * (end[1] as f64 - start[1] as f64)) as u8;
            let b = (start[2] as f64 + t * (end[2] as f64 - start[2] as f64)) as u8;
            format!("\x1b[38;2;{};{};{}m{}", r, g, b, c)
        })
        .collect::<String>()
        + "\x1b[0m"
}
