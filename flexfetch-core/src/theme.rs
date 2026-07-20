use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ThemeStrings {
    pub title: String,
    pub keys: String,
    pub values: String,
    pub sep: String,
    pub reset: &'static str,
}

struct Theme {
    title: &'static str,
    keys: &'static str,
    values: &'static str,
    sep: &'static str,
    reset: &'static str,
}

const RESET: &str = "\x1b[0m";

const NONE: Theme = Theme {
    title: "",
    keys: "",
    values: "",
    sep: "",
    reset: "",
};
const CATPPUCCIN: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[94m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    reset: RESET,
};
const DRACULA: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[95m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    reset: RESET,
};
const NORD: Theme = Theme {
    title: "\x1b[1;94m",
    keys: "\x1b[94m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    reset: RESET,
};
const GRUVBOX: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[93m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    reset: RESET,
};
const TOKYO_NIGHT: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[94m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
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
        _ => &NONE,
    };

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
        reset: preset.reset,
    }
}
