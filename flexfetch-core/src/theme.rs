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

pub fn supports_truecolor() -> bool {
    if let Ok(ct) = std::env::var("COLORTERM") {
        if ct.eq_ignore_ascii_case("truecolor") || ct.contains("24bit") {
            return true;
        }
    }
    let term = std::env::var("TERM").unwrap_or_default().to_lowercase();
    // kitty/wezterm/ghostty are truecolor by definition, even when their TERM
    // value doesn't spell out "truecolor"/"direct" (e.g. TERM=xterm-kitty).
    term.contains("truecolor")
        || term.contains("direct")
        || term.contains("kitty")
        || term.contains("wezterm")
        || term.contains("ghostty")
        || std::env::var("KITTY_WINDOW_ID").is_ok()
}

/// All built-in theme preset names (used by `--list-themes` and `--theme random`).
/// Keep in sync with the `resolve` match arms below.
pub fn preset_names() -> &'static [&'static str] {
    &[
        "catppuccin",
        "dracula",
        "nord",
        "gruvbox",
        "tokyo-night",
        "solarized-dark",
        "solarized-light",
        "rose-pine",
        "rose-pine-dawn",
        "everforest-dark",
        "everforest-light",
        "bamboo",
        "oxocarbon-dark",
        "one-dark",
        "one-light",
        "tokyo-night-storm",
        "catppuccin-mocha",
        "catppuccin-frappe",
        "catppuccin-macchiato",
        "monokai",
        "monokai-pro",
        "ayu-dark",
        "ayu-mirage",
        "palenight",
        "material-ocean",
        "kanagawa",
        "mellow-purple",
    ]
}

/// Pick a pseudo-random preset (Phase 7.8 `--theme random`). Uses a small
/// xorshift-style scramble of the monotonic clock so consecutive runs differ;
/// no external RNG dependency.
pub fn random_preset() -> &'static str {
    let names = preset_names();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as usize)
        .unwrap_or(0);
    let idx = (nanos ^ (nanos >> 16)) % names.len();
    names[idx]
}

/// Truecolor RGB per slot for each preset (Phase 7.1). Only consulted when the
/// terminal supports 24-bit color; the 16-color ANSI consts remain the fallback.
struct ThemeRgb {
    title: [u8; 3],
    keys: [u8; 3],
    values: [u8; 3],
    sep: [u8; 3],
    section: [u8; 3],
}

fn preset_rgb(name: &str) -> Option<ThemeRgb> {
    let hex = |s: &str| -> [u8; 3] {
        let h = s.trim_start_matches('#');
        [
            u8::from_str_radix(&h[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&h[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&h[4..6], 16).unwrap_or(0),
        ]
    };
    Some(match name {
        "catppuccin" => ThemeRgb {
            title: hex("#cba6f7"),
            keys: hex("#89b4fa"),
            values: hex("#94e2d5"),
            sep: hex("#6c7086"),
            section: hex("#89b4fa"),
        },
        "dracula" => ThemeRgb {
            title: hex("#ff79c6"),
            keys: hex("#bd93f9"),
            values: hex("#8be9fd"),
            sep: hex("#6272a4"),
            section: hex("#bd93f9"),
        },
        "nord" => ThemeRgb {
            title: hex("#88c0d0"),
            keys: hex("#81a1c1"),
            values: hex("#a3be8c"),
            sep: hex("#4c566a"),
            section: hex("#81a1c1"),
        },
        "gruvbox" => ThemeRgb {
            title: hex("#fabd2f"),
            keys: hex("#b8bb26"),
            values: hex("#83a598"),
            sep: hex("#928374"),
            section: hex("#b8bb26"),
        },
        "tokyo-night" | "tokyo-night-storm" => ThemeRgb {
            title: hex("#bb9af7"),
            keys: hex("#7aa2f7"),
            values: hex("#7dcfff"),
            sep: hex("#565f89"),
            section: hex("#7aa2f7"),
        },
        "solarized-dark" => ThemeRgb {
            title: hex("#b58900"),
            keys: hex("#268bd2"),
            values: hex("#2aa198"),
            sep: hex("#93a1a1"),
            section: hex("#268bd2"),
        },
        "solarized-light" => ThemeRgb {
            title: hex("#cb4b16"),
            keys: hex("#268bd2"),
            values: hex("#2aa198"),
            sep: hex("#839496"),
            section: hex("#268bd2"),
        },
        "rose-pine" => ThemeRgb {
            title: hex("#eb6f92"),
            keys: hex("#9ccfd8"),
            values: hex("#c4a7e7"),
            sep: hex("#6e6a86"),
            section: hex("#9ccfd8"),
        },
        "rose-pine-dawn" => ThemeRgb {
            title: hex("#d7827e"),
            keys: hex("#56949f"),
            values: hex("#907aa9"),
            sep: hex("#9893a5"),
            section: hex("#56949f"),
        },
        "everforest-dark" => ThemeRgb {
            title: hex("#a7c080"),
            keys: hex("#7fbbb3"),
            values: hex("#83c092"),
            sep: hex("#7a8478"),
            section: hex("#7fbbb3"),
        },
        "everforest-light" => ThemeRgb {
            title: hex("#8da101"),
            keys: hex("#3a94c5"),
            values: hex("#35a77c"),
            sep: hex("#a6b0a0"),
            section: hex("#3a94c5"),
        },
        "bamboo" => ThemeRgb {
            title: hex("#e06848"),
            keys: hex("#68a870"),
            values: hex("#5898a8"),
            sep: hex("#888a90"),
            section: hex("#68a870"),
        },
        "oxocarbon-dark" => ThemeRgb {
            title: hex("#08bdba"),
            keys: hex("#33b1ff"),
            values: hex("#ff7eb6"),
            sep: hex("#525252"),
            section: hex("#33b1ff"),
        },
        "one-dark" => ThemeRgb {
            title: hex("#c678dd"),
            keys: hex("#61afef"),
            values: hex("#98c379"),
            sep: hex("#5c6370"),
            section: hex("#61afef"),
        },
        "one-light" => ThemeRgb {
            title: hex("#a626a4"),
            keys: hex("#e45649"),
            values: hex("#50a14f"),
            sep: hex("#a0a1a7"),
            section: hex("#e45649"),
        },
        "catppuccin-mocha" => ThemeRgb {
            title: hex("#cba6f7"),
            keys: hex("#89b4fa"),
            values: hex("#94e2d5"),
            sep: hex("#6c7086"),
            section: hex("#89b4fa"),
        },
        "catppuccin-frappe" => ThemeRgb {
            title: hex("#ca9ee6"),
            keys: hex("#8caaee"),
            values: hex("#81c8be"),
            sep: hex("#737994"),
            section: hex("#8caaee"),
        },
        "catppuccin-macchiato" => ThemeRgb {
            title: hex("#c6a0f6"),
            keys: hex("#8aadf4"),
            values: hex("#8bd5ca"),
            sep: hex("#6e738d"),
            section: hex("#8aadf4"),
        },
        "monokai" => ThemeRgb {
            title: hex("#e6db74"),
            keys: hex("#a6e22e"),
            values: hex("#f92672"),
            sep: hex("#75715e"),
            section: hex("#a6e22e"),
        },
        "monokai-pro" => ThemeRgb {
            title: hex("#ffd866"),
            keys: hex("#a9dc76"),
            values: hex("#78dce8"),
            sep: hex("#727072"),
            section: hex("#a9dc76"),
        },
        "ayu-dark" => ThemeRgb {
            title: hex("#ffb454"),
            keys: hex("#39bae6"),
            values: hex("#aad94c"),
            sep: hex("#5c6773"),
            section: hex("#39bae6"),
        },
        "ayu-mirage" => ThemeRgb {
            title: hex("#ffcc66"),
            keys: hex("#73d0ff"),
            values: hex("#d4bfff"),
            sep: hex("#5c6773"),
            section: hex("#73d0ff"),
        },
        "palenight" => ThemeRgb {
            title: hex("#ffcb6b"),
            keys: hex("#82aaff"),
            values: hex("#c792ea"),
            sep: hex("#676e95"),
            section: hex("#82aaff"),
        },
        "material-ocean" => ThemeRgb {
            title: hex("#ffcb6b"),
            keys: hex("#82aaff"),
            values: hex("#89ddff"),
            sep: hex("#525975"),
            section: hex("#82aaff"),
        },
        "kanagawa" => ThemeRgb {
            title: hex("#c34043"),
            keys: hex("#7e9cd8"),
            values: hex("#6a9589"),
            sep: hex("#727169"),
            section: hex("#7e9cd8"),
        },
        "mellow-purple" => ThemeRgb {
            title: hex("#d2a6ff"),
            keys: hex("#b3a1e6"),
            values: hex("#9ce6d4"),
            sep: hex("#8a88b8"),
            section: hex("#b3a1e6"),
        },
        _ => return None,
    })
}

pub fn resolve(config: &Config) -> ThemeStrings {
    let theme_arg = config.display.theme.as_deref().unwrap_or("");
    // Phase 7.8: `--theme random` (and `theme = "random"` in config) resolves
    // to a random preset each run.
    // Phase 5.4: `--auto-theme` (and `theme = "auto"` in config) derives the
    // theme from the wallpaper's dominant colors. Falls back to catppuccin when
    // the feature is off or no usable palette can be extracted.
    if theme_arg == "auto" {
        #[cfg(feature = "auto-theme")]
        if let Some(auto) = crate::autotheme::auto_theme() {
            return auto;
        }
        // Feature off / extraction failed → behave like the default preset.
        let fallback = Config::default_for_testing();
        return resolve(&fallback);
    }

    let resolved_name = if theme_arg == "random" {
        random_preset()
    } else {
        theme_arg
    };
    let preset = match resolved_name {
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

    let truecolor = supports_truecolor();
    let rgb = preset_rgb(resolved_name);
    // Pick a color code for one slot: explicit config override wins; else
    // truecolor RGB when the terminal supports it; else the 16-color ANSI.
    let slot =
        |override_cfg: &Option<String>, ansi: &str, rgbc: Option<[u8; 3]>, bold: bool| -> String {
            if let Some(c) = override_cfg {
                return resolve_ansi(c);
            }
            if let Some(rgb) = rgbc {
                if truecolor {
                    let pre = if bold { "\x1b[1;38;2;" } else { "\x1b[38;2;" };
                    return format!("{pre}{};{};{}m", rgb[0], rgb[1], rgb[2]);
                }
            }
            ansi.to_string()
        };

    let gradient_colors = config
        .display
        .gradient_colors
        .as_deref()
        .map(|cs| cs.iter().filter_map(|c| parse_hex_color(c)).collect())
        .unwrap_or_else(|| preset.gradient_colors.to_vec());

    ThemeStrings {
        title: slot(
            &config.display.color_title,
            preset.title,
            rgb.as_ref().map(|r| r.title),
            true,
        ),
        keys: slot(
            &config.display.color_keys,
            preset.keys,
            rgb.as_ref().map(|r| r.keys),
            false,
        ),
        values: slot(
            &config.display.color_values,
            preset.values,
            rgb.as_ref().map(|r| r.values),
            false,
        ),
        sep: slot(
            &config.display.color_sep,
            preset.sep,
            rgb.as_ref().map(|r| r.sep),
            false,
        ),
        section: slot(&None, preset.section, rgb.as_ref().map(|r| r.section), true),
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
