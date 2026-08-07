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

/// Truecolor RGB per slot for each preset. Only consulted when the terminal
/// supports 24-bit color; the 16-color ANSI strings remain the fallback.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ThemeRgb {
    pub title: [u8; 3],
    pub keys: [u8; 3],
    pub values: [u8; 3],
    pub sep: [u8; 3],
    pub section: [u8; 3],
}

struct ThemeEntry {
    name: &'static str,
    title: &'static str,
    keys: &'static str,
    values: &'static str,
    sep: &'static str,
    section: &'static str,
    reset: &'static str,
    gradient_colors: &'static [[u8; 3]],
    rgb: ThemeRgb,
}

const RESET: &str = "\x1b[0m";

/// Single source of truth for every built-in theme: preset name, the 16-color
/// ANSI strings per slot, the gradient stops, and the truecolor RGB per slot.
/// `preset_names`, `preset_rgb`, and `resolve` are all derived from this table.
const THEMES: &[ThemeEntry] = &[
    ThemeEntry {
        name: "catppuccin",
        title: "\x1b[1;95m",
        keys: "\x1b[94m",
        values: "\x1b[96m",
        sep: "\x1b[90m",
        section: "\x1b[1;94m",
        reset: RESET,
        gradient_colors: &[[203, 166, 247], [245, 224, 220], [137, 180, 250]],
        rgb: ThemeRgb {
            title: [203, 166, 247],
            keys: [137, 180, 250],
            values: [148, 226, 213],
            sep: [108, 112, 134],
            section: [137, 180, 250],
        },
    },
    ThemeEntry {
        name: "dracula",
        title: "\x1b[1;95m",
        keys: "\x1b[95m",
        values: "\x1b[96m",
        sep: "\x1b[90m",
        section: "\x1b[1;96m",
        reset: RESET,
        gradient_colors: &[[189, 147, 249], [255, 121, 198], [139, 233, 253]],
        rgb: ThemeRgb {
            title: [255, 121, 198],
            keys: [189, 147, 249],
            values: [139, 233, 253],
            sep: [98, 114, 164],
            section: [189, 147, 249],
        },
    },
    ThemeEntry {
        name: "nord",
        title: "\x1b[1;94m",
        keys: "\x1b[94m",
        values: "\x1b[92m",
        sep: "\x1b[90m",
        section: "\x1b[1;92m",
        reset: RESET,
        gradient_colors: &[[143, 188, 187], [136, 192, 208], [163, 190, 140]],
        rgb: ThemeRgb {
            title: [136, 192, 208],
            keys: [129, 161, 193],
            values: [163, 190, 140],
            sep: [76, 86, 106],
            section: [129, 161, 193],
        },
    },
    ThemeEntry {
        name: "gruvbox",
        title: "\x1b[1;93m",
        keys: "\x1b[93m",
        values: "\x1b[92m",
        sep: "\x1b[90m",
        section: "\x1b[1;93m",
        reset: RESET,
        gradient_colors: &[[250, 189, 47], [184, 184, 184], [131, 165, 152]],
        rgb: ThemeRgb {
            title: [250, 189, 47],
            keys: [184, 187, 38],
            values: [131, 165, 152],
            sep: [146, 131, 116],
            section: [184, 187, 38],
        },
    },
    ThemeEntry {
        name: "tokyo-night",
        title: "\x1b[1;95m",
        keys: "\x1b[94m",
        values: "\x1b[96m",
        sep: "\x1b[90m",
        section: "\x1b[1;95m",
        reset: RESET,
        gradient_colors: &[[187, 154, 247], [122, 162, 247], [125, 207, 255]],
        rgb: ThemeRgb {
            title: [187, 154, 247],
            keys: [122, 162, 247],
            values: [125, 207, 255],
            sep: [86, 95, 137],
            section: [122, 162, 247],
        },
    },
    ThemeEntry {
        name: "solarized-dark",
        title: "\x1b[1;33m",
        keys: "\x1b[36m",
        values: "\x1b[34m",
        sep: "\x1b[90m",
        section: "\x1b[1;33m",
        reset: RESET,
        gradient_colors: &[[181, 137, 0], [42, 161, 152], [38, 139, 210]],
        rgb: ThemeRgb {
            title: [181, 137, 0],
            keys: [38, 139, 210],
            values: [42, 161, 152],
            sep: [147, 161, 161],
            section: [38, 139, 210],
        },
    },
    ThemeEntry {
        name: "solarized-light",
        title: "\x1b[1;31m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;31m",
        reset: RESET,
        gradient_colors: &[[203, 75, 22], [38, 139, 210], [42, 161, 152]],
        rgb: ThemeRgb {
            title: [203, 75, 22],
            keys: [38, 139, 210],
            values: [42, 161, 152],
            sep: [131, 148, 150],
            section: [38, 139, 210],
        },
    },
    ThemeEntry {
        name: "rose-pine",
        title: "\x1b[1;36m",
        keys: "\x1b[36m",
        values: "\x1b[35m",
        sep: "\x1b[90m",
        section: "\x1b[1;36m",
        reset: RESET,
        gradient_colors: &[[235, 111, 146], [246, 193, 119], [156, 207, 216]],
        rgb: ThemeRgb {
            title: [235, 111, 146],
            keys: [156, 207, 216],
            values: [196, 167, 231],
            sep: [110, 106, 134],
            section: [156, 207, 216],
        },
    },
    ThemeEntry {
        name: "rose-pine-dawn",
        title: "\x1b[1;34m",
        keys: "\x1b[36m",
        values: "\x1b[35m",
        sep: "\x1b[90m",
        section: "\x1b[1;34m",
        reset: RESET,
        gradient_colors: &[[184, 90, 120], [204, 159, 95], [121, 164, 171]],
        rgb: ThemeRgb {
            title: [215, 130, 126],
            keys: [86, 148, 159],
            values: [144, 122, 169],
            sep: [152, 147, 165],
            section: [86, 148, 159],
        },
    },
    ThemeEntry {
        name: "everforest-dark",
        title: "\x1b[1;32m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;32m",
        reset: RESET,
        gradient_colors: &[[163, 190, 140], [127, 187, 164], [211, 198, 170]],
        rgb: ThemeRgb {
            title: [167, 192, 128],
            keys: [127, 187, 179],
            values: [131, 192, 146],
            sep: [122, 132, 120],
            section: [127, 187, 179],
        },
    },
    ThemeEntry {
        name: "everforest-light",
        title: "\x1b[1;32m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;32m",
        reset: RESET,
        gradient_colors: &[[133, 160, 112], [96, 158, 139], [178, 162, 138]],
        rgb: ThemeRgb {
            title: [141, 161, 1],
            keys: [58, 148, 197],
            values: [53, 167, 124],
            sep: [166, 176, 160],
            section: [58, 148, 197],
        },
    },
    ThemeEntry {
        name: "bamboo",
        title: "\x1b[1;31m",
        keys: "\x1b[32m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;31m",
        reset: RESET,
        gradient_colors: &[[220, 90, 90], [120, 190, 120], [100, 180, 210]],
        rgb: ThemeRgb {
            title: [224, 104, 72],
            keys: [104, 168, 112],
            values: [88, 152, 168],
            sep: [136, 138, 144],
            section: [104, 168, 112],
        },
    },
    ThemeEntry {
        name: "oxocarbon-dark",
        title: "\x1b[1;36m",
        keys: "\x1b[36m",
        values: "\x1b[35m",
        sep: "\x1b[90m",
        section: "\x1b[1;36m",
        reset: RESET,
        gradient_colors: &[[35, 165, 189], [169, 123, 255], [235, 188, 55]],
        rgb: ThemeRgb {
            title: [8, 189, 186],
            keys: [51, 177, 255],
            values: [255, 126, 182],
            sep: [82, 82, 82],
            section: [51, 177, 255],
        },
    },
    ThemeEntry {
        name: "one-dark",
        title: "\x1b[1;35m",
        keys: "\x1b[31m",
        values: "\x1b[32m",
        sep: "\x1b[90m",
        section: "\x1b[1;35m",
        reset: RESET,
        gradient_colors: &[[198, 120, 221], [224, 108, 117], [152, 195, 121]],
        rgb: ThemeRgb {
            title: [198, 120, 221],
            keys: [97, 175, 239],
            values: [152, 195, 121],
            sep: [92, 99, 112],
            section: [97, 175, 239],
        },
    },
    ThemeEntry {
        name: "one-light",
        title: "\x1b[1;35m",
        keys: "\x1b[31m",
        values: "\x1b[32m",
        sep: "\x1b[90m",
        section: "\x1b[1;35m",
        reset: RESET,
        gradient_colors: &[[165, 93, 194], [209, 83, 97], [120, 169, 96]],
        rgb: ThemeRgb {
            title: [166, 38, 164],
            keys: [228, 86, 73],
            values: [80, 161, 79],
            sep: [160, 161, 167],
            section: [228, 86, 73],
        },
    },
    ThemeEntry {
        name: "tokyo-night-storm",
        title: "\x1b[1;35m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;35m",
        reset: RESET,
        gradient_colors: &[[187, 154, 247], [125, 207, 255], [187, 154, 247]],
        rgb: ThemeRgb {
            title: [187, 154, 247],
            keys: [122, 162, 247],
            values: [125, 207, 255],
            sep: [86, 95, 137],
            section: [122, 162, 247],
        },
    },
    ThemeEntry {
        name: "catppuccin-mocha",
        title: "\x1b[1;35m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;35m",
        reset: RESET,
        gradient_colors: &[[203, 166, 247], [245, 224, 220], [137, 180, 250]],
        rgb: ThemeRgb {
            title: [203, 166, 247],
            keys: [137, 180, 250],
            values: [148, 226, 213],
            sep: [108, 112, 134],
            section: [137, 180, 250],
        },
    },
    ThemeEntry {
        name: "catppuccin-frappe",
        title: "\x1b[1;35m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;35m",
        reset: RESET,
        gradient_colors: &[[202, 158, 230], [242, 213, 207], [140, 170, 238]],
        rgb: ThemeRgb {
            title: [202, 158, 230],
            keys: [140, 170, 238],
            values: [129, 200, 190],
            sep: [115, 121, 148],
            section: [140, 170, 238],
        },
    },
    ThemeEntry {
        name: "catppuccin-macchiato",
        title: "\x1b[1;35m",
        keys: "\x1b[34m",
        values: "\x1b[36m",
        sep: "\x1b[90m",
        section: "\x1b[1;35m",
        reset: RESET,
        gradient_colors: &[[198, 160, 246], [238, 212, 209], [138, 173, 244]],
        rgb: ThemeRgb {
            title: [198, 160, 246],
            keys: [138, 173, 244],
            values: [139, 213, 202],
            sep: [110, 115, 141],
            section: [138, 173, 244],
        },
    },
    ThemeEntry {
        name: "monokai",
        title: "\x1b[1;93m",
        keys: "\x1b[1;92m",
        values: "\x1b[91m",
        sep: "\x1b[90m",
        section: "\x1b[1;93m",
        reset: RESET,
        gradient_colors: &[[229, 192, 123], [166, 226, 118], [249, 38, 114]],
        rgb: ThemeRgb {
            title: [230, 219, 116],
            keys: [166, 226, 46],
            values: [249, 38, 114],
            sep: [117, 113, 94],
            section: [166, 226, 46],
        },
    },
    ThemeEntry {
        name: "monokai-pro",
        title: "\x1b[1;95m",
        keys: "\x1b[1;96m",
        values: "\x1b[93m",
        sep: "\x1b[90m",
        section: "\x1b[1;95m",
        reset: RESET,
        gradient_colors: &[[171, 123, 224], [120, 204, 220], [252, 183, 88]],
        rgb: ThemeRgb {
            title: [255, 216, 102],
            keys: [169, 220, 118],
            values: [120, 220, 232],
            sep: [114, 112, 114],
            section: [169, 220, 118],
        },
    },
    ThemeEntry {
        name: "ayu-dark",
        title: "\x1b[1;93m",
        keys: "\x1b[1;96m",
        values: "\x1b[92m",
        sep: "\x1b[90m",
        section: "\x1b[1;93m",
        reset: RESET,
        gradient_colors: &[[230, 193, 70], [100, 210, 200], [171, 233, 124]],
        rgb: ThemeRgb {
            title: [255, 180, 84],
            keys: [57, 186, 230],
            values: [170, 217, 76],
            sep: [92, 103, 115],
            section: [57, 186, 230],
        },
    },
    ThemeEntry {
        name: "ayu-mirage",
        title: "\x1b[1;95m",
        keys: "\x1b[1;96m",
        values: "\x1b[93m",
        sep: "\x1b[90m",
        section: "\x1b[1;95m",
        reset: RESET,
        gradient_colors: &[[202, 150, 220], [100, 210, 200], [255, 204, 102]],
        rgb: ThemeRgb {
            title: [255, 204, 102],
            keys: [115, 208, 255],
            values: [212, 191, 255],
            sep: [92, 103, 115],
            section: [115, 208, 255],
        },
    },
    ThemeEntry {
        name: "palenight",
        title: "\x1b[1;93m",
        keys: "\x1b[1;96m",
        values: "\x1b[92m",
        sep: "\x1b[90m",
        section: "\x1b[1;93m",
        reset: RESET,
        gradient_colors: &[[199, 146, 234], [85, 180, 222], [171, 233, 124]],
        rgb: ThemeRgb {
            title: [255, 203, 107],
            keys: [130, 170, 255],
            values: [199, 146, 234],
            sep: [103, 110, 149],
            section: [130, 170, 255],
        },
    },
    ThemeEntry {
        name: "material-ocean",
        title: "\x1b[1;93m",
        keys: "\x1b[1;96m",
        values: "\x1b[91m",
        sep: "\x1b[90m",
        section: "\x1b[1;93m",
        reset: RESET,
        gradient_colors: &[[255, 183, 77], [0, 230, 230], [255, 82, 82]],
        rgb: ThemeRgb {
            title: [255, 203, 107],
            keys: [130, 170, 255],
            values: [137, 221, 255],
            sep: [82, 89, 117],
            section: [130, 170, 255],
        },
    },
    ThemeEntry {
        name: "kanagawa",
        title: "\x1b[1;91m",
        keys: "\x1b[1;96m",
        values: "\x1b[92m",
        sep: "\x1b[90m",
        section: "\x1b[1;91m",
        reset: RESET,
        gradient_colors: &[[232, 63, 86], [114, 191, 201], [166, 209, 137]],
        rgb: ThemeRgb {
            title: [195, 64, 67],
            keys: [126, 156, 216],
            values: [106, 149, 137],
            sep: [114, 113, 105],
            section: [126, 156, 216],
        },
    },
    ThemeEntry {
        name: "mellow-purple",
        title: "\x1b[1;95m",
        keys: "\x1b[1;96m",
        values: "\x1b[92m",
        sep: "\x1b[90m",
        section: "\x1b[1;95m",
        reset: RESET,
        gradient_colors: &[[178, 102, 255], [0, 210, 210], [166, 226, 118]],
        rgb: ThemeRgb {
            title: [210, 166, 255],
            keys: [179, 161, 230],
            values: [156, 230, 212],
            sep: [138, 136, 184],
            section: [179, 161, 230],
        },
    },
];

const NONE: ThemeEntry = ThemeEntry {
    name: "",
    title: "",
    keys: "",
    values: "",
    sep: "",
    section: "",
    reset: "",
    gradient_colors: &[],
    rgb: ThemeRgb {
        title: [0, 0, 0],
        keys: [0, 0, 0],
        values: [0, 0, 0],
        sep: [0, 0, 0],
        section: [0, 0, 0],
    },
};

fn find_preset(name: &str) -> Option<&'static ThemeEntry> {
    THEMES.iter().find(|t| t.name == name)
}

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
/// Derived from the `THEMES` table — the single source of truth.
pub fn preset_names() -> &'static [&'static str] {
    static NAMES: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();
    NAMES.get_or_init(|| THEMES.iter().map(|t| t.name).collect())
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

/// Truecolor RGB per slot for a preset (Phase 7.1). Only consulted when the
/// terminal supports 24-bit color; the 16-color ANSI strings remain the
/// fallback. Unknown names (including "random"/"auto", handled by callers)
/// return None.
fn preset_rgb(name: &str) -> Option<ThemeRgb> {
    find_preset(name).map(|t| t.rgb)
}

/// ANSI truecolor code for one color slot (bold flag sets the bright prefix).
pub(crate) fn truecolor(rgb: [u8; 3], bold: bool) -> String {
    let pre = if bold { "\x1b[1;38;2;" } else { "\x1b[38;2;" };
    format!("{pre}{};{};{}m", rgb[0], rgb[1], rgb[2])
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
    let preset = find_preset(resolved_name).unwrap_or(&NONE);

    let truecolor_support = supports_truecolor();
    let rgb = preset_rgb(resolved_name);
    // Pick a color code for one slot: explicit config override wins; else
    // truecolor RGB when the terminal supports it; else the 16-color ANSI.
    let slot =
        |override_cfg: &Option<String>, ansi: &str, rgbc: Option<[u8; 3]>, bold: bool| -> String {
            if let Some(c) = override_cfg {
                return resolve_ansi(c);
            }
            if let Some(rgb) = rgbc {
                if truecolor_support {
                    return truecolor(rgb, bold);
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
