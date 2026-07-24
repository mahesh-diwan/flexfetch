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
    gradient_colors: &'static [[u8; 3]],
}

const RESET: &str = "\x1b[0m";

const NONE: Theme = Theme {
    title: "",
    keys: "",
    values: "",
    sep: "",
    section: "",
    reset: "",
    gradient_colors: &[],
};
const CATPPUCCIN: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[94m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    section: "\x1b[1;94m",
    reset: RESET,
    gradient_colors: &[[203, 166, 247], [245, 224, 220], [137, 180, 250]],
};
const DRACULA: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[95m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    section: "\x1b[1;96m",
    reset: RESET,
    gradient_colors: &[[189, 147, 249], [255, 121, 198], [139, 233, 253]],
};
const NORD: Theme = Theme {
    title: "\x1b[1;94m",
    keys: "\x1b[94m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;92m",
    reset: RESET,
    gradient_colors: &[[143, 188, 187], [136, 192, 208], [163, 190, 140]],
};
const GRUVBOX: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[93m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
    gradient_colors: &[[250, 189, 47], [184, 184, 184], [131, 165, 152]],
};
const TOKYO_NIGHT: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[94m",
    values: "\x1b[96m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
    gradient_colors: &[[187, 154, 247], [122, 162, 247], [125, 207, 255]],
};
const SOLARIZED_DARK: Theme = Theme {
    title: "\x1b[1;33m",
    keys: "\x1b[36m",
    values: "\x1b[34m",
    sep: "\x1b[90m",
    section: "\x1b[1;33m",
    reset: RESET,
    gradient_colors: &[[181, 137, 0], [42, 161, 152], [38, 139, 210]],
};
const SOLARIZED_LIGHT: Theme = Theme {
    title: "\x1b[1;31m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;31m",
    reset: RESET,
    gradient_colors: &[[203, 75, 22], [38, 139, 210], [42, 161, 152]],
};
const ROSE_PINE: Theme = Theme {
    title: "\x1b[1;36m",
    keys: "\x1b[36m",
    values: "\x1b[35m",
    sep: "\x1b[90m",
    section: "\x1b[1;36m",
    reset: RESET,
    gradient_colors: &[[235, 111, 146], [246, 193, 119], [156, 207, 216]],
};
const ROSE_PINE_DAWN: Theme = Theme {
    title: "\x1b[1;34m",
    keys: "\x1b[36m",
    values: "\x1b[35m",
    sep: "\x1b[90m",
    section: "\x1b[1;34m",
    reset: RESET,
    gradient_colors: &[[184, 90, 120], [204, 159, 95], [121, 164, 171]],
};
const EVERFOREST_DARK: Theme = Theme {
    title: "\x1b[1;32m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;32m",
    reset: RESET,
    gradient_colors: &[[163, 190, 140], [127, 187, 164], [211, 198, 170]],
};
const EVERFOREST_LIGHT: Theme = Theme {
    title: "\x1b[1;32m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;32m",
    reset: RESET,
    gradient_colors: &[[133, 160, 112], [96, 158, 139], [178, 162, 138]],
};
const BAMBOO: Theme = Theme {
    title: "\x1b[1;31m",
    keys: "\x1b[32m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;31m",
    reset: RESET,
    gradient_colors: &[[220, 90, 90], [120, 190, 120], [100, 180, 210]],
};
const OXOCARBON_DARK: Theme = Theme {
    title: "\x1b[1;36m",
    keys: "\x1b[36m",
    values: "\x1b[35m",
    sep: "\x1b[90m",
    section: "\x1b[1;36m",
    reset: RESET,
    gradient_colors: &[[35, 165, 189], [169, 123, 255], [235, 188, 55]],
};
const ONE_DARK: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[31m",
    values: "\x1b[32m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
    gradient_colors: &[[198, 120, 221], [224, 108, 117], [152, 195, 121]],
};
const ONE_LIGHT: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[31m",
    values: "\x1b[32m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
    gradient_colors: &[[165, 93, 194], [209, 83, 97], [120, 169, 96]],
};
const TOKYO_NIGHT_STORM: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
    gradient_colors: &[[187, 154, 247], [125, 207, 255], [187, 154, 247]],
};
const CATPPUCCIN_MOCHA: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
    gradient_colors: &[[203, 166, 247], [245, 224, 220], [137, 180, 250]],
};
const CATPPUCCIN_FRAPPE: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
    gradient_colors: &[[202, 158, 230], [242, 213, 207], [140, 170, 238]],
};
const CATPPUCCIN_MACCHIATO: Theme = Theme {
    title: "\x1b[1;35m",
    keys: "\x1b[34m",
    values: "\x1b[36m",
    sep: "\x1b[90m",
    section: "\x1b[1;35m",
    reset: RESET,
    gradient_colors: &[[198, 160, 246], [238, 212, 209], [138, 173, 244]],
};
const MONOKAI: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;92m",
    values: "\x1b[91m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
    gradient_colors: &[[229, 192, 123], [166, 226, 118], [249, 38, 114]],
};
const MONOKAI_PRO: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[1;96m",
    values: "\x1b[93m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
    gradient_colors: &[[171, 123, 224], [120, 204, 220], [252, 183, 88]],
};
const AYU_DARK: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
    gradient_colors: &[[230, 193, 70], [100, 210, 200], [171, 233, 124]],
};
const AYU_MIRAGE: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[1;96m",
    values: "\x1b[93m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
    gradient_colors: &[[202, 150, 220], [100, 210, 200], [255, 204, 102]],
};
const PALENIGHT: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
    gradient_colors: &[[199, 146, 234], [85, 180, 222], [171, 233, 124]],
};
const MATERIAL_OCEAN: Theme = Theme {
    title: "\x1b[1;93m",
    keys: "\x1b[1;96m",
    values: "\x1b[91m",
    sep: "\x1b[90m",
    section: "\x1b[1;93m",
    reset: RESET,
    gradient_colors: &[[255, 183, 77], [0, 230, 230], [255, 82, 82]],
};
const KANAGAWA: Theme = Theme {
    title: "\x1b[1;91m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;91m",
    reset: RESET,
    gradient_colors: &[[232, 63, 86], [114, 191, 201], [166, 209, 137]],
};
const MELLOW_PURPLE: Theme = Theme {
    title: "\x1b[1;95m",
    keys: "\x1b[1;96m",
    values: "\x1b[92m",
    sep: "\x1b[90m",
    section: "\x1b[1;95m",
    reset: RESET,
    gradient_colors: &[[178, 102, 255], [0, 210, 210], [166, 226, 118]],
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
        .unwrap_or_else(|| preset.gradient_colors.to_vec());

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

pub fn gradient_text(text: &str, colors: &[[u8; 3]]) -> String {
    if colors.is_empty() || text.is_empty() {
        return text.to_string();
    }
    let mut result = String::with_capacity(text.len() * 20);
    for (i, ch) in text.char_indices() {
        let color = colors[i % colors.len()];
        result.push_str(&format!(
            "\x1b[38;2;{};{};{}m{}",
            color[0], color[1], color[2], ch
        ));
    }
    result.push_str(RESET);
    result
}
