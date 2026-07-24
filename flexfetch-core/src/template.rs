use std::cmp;
use std::sync::OnceLock;
use tera::{Context as TeraContext, Tera};

use crate::{InfoValue, SystemInfo};

use crate::image_logo::{
    get_distro_logo_path, get_module_logo_path, ImageLogo, ImageProtocol, LogoMode,
};

pub fn frame_wrap(text: &str, style: &str, color: &str) -> String {
    let (tl, tr, bl, br, h, v) = match style {
        "double" => ("╔", "╗", "╚", "╝", "═", "║"),
        "decorative" | "single" => ("┌", "┐", "└", "┘", "─", "│"),
        _ => return text.to_string(),
    };
    let max_w = text.lines().map(|l| l.len()).max().unwrap_or(40);
    let mut result = String::new();
    result.push_str(&format!("{color}{tl}{}{tr}\x1b[0m\n", h.repeat(max_w)));
    for line in text.lines() {
        let padding = max_w.saturating_sub(line.len());
        result.push_str(&format!(
            "{color}{v}\x1b[0m {line}{pad} {color}{v}\x1b[0m\n",
            pad = " ".repeat(padding),
        ));
    }
    result.push_str(&format!("{color}{bl}{}{br}\x1b[0m\n", h.repeat(max_w)));
    result
}

#[derive(Debug, Clone)]
pub struct BoxChars {
    pub header_left: String,
    pub header_line: String,
    pub row: String,
    pub sep: String,
}

pub fn get_box_chars(style: &str) -> BoxChars {
    match style {
        "double" => BoxChars {
            header_left: "╔═ ".into(),
            header_line: "═".into(),
            row: "║".into(),
            sep: "╠".into(),
        },
        "dotted" => BoxChars {
            header_left: "┌─ ".into(),
            header_line: "─".into(),
            row: "│".into(),
            sep: "├".into(),
        },
        "thick" => BoxChars {
            header_left: "┏━ ".into(),
            header_line: "━".into(),
            row: "┃".into(),
            sep: "┣".into(),
        },
        "ascii" => BoxChars {
            header_left: "+- ".into(),
            header_line: "-".into(),
            row: "|".into(),
            sep: "+".into(),
        },
        _ => BoxChars {
            // rounded (default)
            header_left: "╭─ ".into(),
            header_line: "─".into(),
            row: "│".into(),
            sep: "├".into(),
        },
    }
}

fn palette_display_filter(
    value: &serde_json::Value,
    args: &std::collections::HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let style = args
        .get("style")
        .and_then(|v| v.as_str())
        .unwrap_or("blocks");
    let colors = match value {
        serde_json::Value::Array(arr) => arr,
        _ => return Ok(serde_json::Value::String(String::new())),
    };
    let result: String = colors
        .iter()
        .filter_map(|c| {
            let arr = c.as_array()?;
            let r = arr.get(0)?.as_u64()? as u8;
            let g = arr.get(1)?.as_u64()? as u8;
            let b = arr.get(2)?.as_u64()? as u8;
            Some(match style {
                "squares" => format!("\x1b[48;2;{r};{g};{b}m  \x1b[0m"),
                "dots" => format!("\x1b[38;2;{r};{g};{b}m▪\x1b[0m"),
                _ => format!("\x1b[48;2;{r};{g};{b}m██\x1b[0m"),
            })
        })
        .collect::<Vec<_>>()
        .join(" ");
    Ok(serde_json::Value::String(result))
}

fn progress_bar_filter(
    value: &serde_json::Value,
    args: &std::collections::HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let percent = value.as_u64().unwrap_or(0) as u8;
    let width = args.get("width").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let filled = (percent as usize * width) / 100;
    let empty = width - filled;
    let color = if percent < 60 {
        "\x1b[32m"
    } else if percent < 85 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };
    Ok(serde_json::Value::String(format!(
        "{color}[{}{}]\x1b[0m",
        "█".repeat(filled),
        "░".repeat(empty),
    )))
}

static CACHED_TERA: OnceLock<Tera> = OnceLock::new();

fn get_tera() -> &'static Tera {
    CACHED_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        tera.add_raw_template("default", include_str!("../../templates/default.tera"))
            .expect("default template is valid");
        tera.register_filter("palette_display", palette_display_filter);
        tera.register_filter("progress_bar", progress_bar_filter);
        tera
    })
}

pub struct TeraEngine {
    tera: Tera,
    template_name: String,
}

impl TeraEngine {
    pub fn new_default() -> Self {
        TeraEngine {
            tera: get_tera().clone(),
            template_name: "default".to_string(),
        }
    }

    pub fn default_template_content() -> &'static str {
        include_str!("../../templates/default.tera")
    }

    pub fn render(&self, info: &SystemInfo, config: &crate::Config) -> crate::Result<String> {
        let mut ctx = TeraContext::new();
        for (name, value) in &info.entries {
            let json_val = serde_json::to_value(value)
                .map_err(|e| crate::Error::Template(format!("serialize {name}: {e}")))?;
            ctx.insert(*name, &json_val);
        }
        ctx.insert("display_separator", &config.display.separator);
        ctx.insert("display_key_width", &config.display.key_width);

        let box_chars = get_box_chars(&config.display.box_style);
        ctx.insert("box_header_left", &box_chars.header_left);
        ctx.insert("box_header_line", &box_chars.header_line);
        ctx.insert("box_row", &box_chars.row);
        ctx.insert("box_sep", &box_chars.sep);

        let theme = crate::theme::resolve(config);
        ctx.insert("theme_title", &theme.title);
        ctx.insert("theme_keys", &theme.keys);
        ctx.insert("theme_values", &theme.values);
        ctx.insert("theme_sep", &theme.sep);
        ctx.insert("theme_section", &theme.section);
        ctx.insert("theme_reset", &theme.reset);
        ctx.insert("theme_gradient", &theme.gradient);

        // Compute gradient title if enabled
        let title_text = info
            .entries
            .iter()
            .find(|(n, _)| *n == "title")
            .and_then(|(_, v)| {
                if let InfoValue::Scalar(s) = v {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .unwrap_or("");

        if config.display.gradient_title && !theme.gradient_colors.is_empty() {
            let gradient = crate::theme::gradient_text(title_text, &theme.gradient_colors);
            ctx.insert("theme_title_gradient", &gradient);
        } else {
            ctx.insert("theme_title_gradient", &theme.title);
        }

        // Add image logos to context
        let modules = [
            "title",
            "os",
            "host",
            "kernel",
            "uptime",
            "locale",
            "shell",
            "terminal",
            "de",
            "wm",
            "packages",
            "cpu",
            "memory",
            "disk",
            "gpu",
            "network",
            "battery",
            "processes",
            "resolution",
            "colors",
            "custom",
        ];
        let mut image_logos = serde_json::Map::new();
        let protocol = ImageProtocol::detect();
        // Only render inline image logos when terminal supports image protocols
        if matches!(
            protocol,
            ImageProtocol::Kitty
                | ImageProtocol::Iterm2
                | ImageProtocol::Sixel
                | ImageProtocol::Block
        ) {
            for name in modules {
                if info.entries.iter().any(|(n, _)| *n == name) {
                    if let Some(path) = get_module_logo_path(name) {
                        if std::path::Path::new(&path).exists() {
                            let logo = ImageLogo::new(&path).with_size(15, 8);
                            let mode = LogoMode::Auto;
                            let ansi = logo.render(protocol, mode);
                            if !ansi.is_empty() {
                                image_logos
                                    .insert(name.to_string(), serde_json::Value::String(ansi));
                            }
                        }
                    }
                }
            }
        }
        ctx.insert("image_logos", &serde_json::Value::Object(image_logos));

        // Add distro image logo
        let os_id = info
            .entries
            .iter()
            .find(|(n, _)| *n == "os")
            .and_then(|(_, v)| {
                if let InfoValue::Map(m) = v {
                    m.get("id").cloned()
                } else {
                    None
                }
            })
            .unwrap_or_default();

        if matches!(
            protocol,
            ImageProtocol::Kitty
                | ImageProtocol::Iterm2
                | ImageProtocol::Sixel
                | ImageProtocol::Block
        ) {
            if let Some(distro_path) = get_distro_logo_path(&os_id) {
                if std::path::Path::new(&distro_path).exists() {
                    let logo = ImageLogo::new(&distro_path).with_size(15, 30);
                    let mode = LogoMode::Auto;
                    let ansi = logo.render(protocol, mode);
                    if !ansi.is_empty() {
                        ctx.insert("distro_image_logo", &ansi);
                    }
                }
            }
        }

        let raw = self
            .tera
            .render(&self.template_name, &ctx)
            .map_err(|e| crate::Error::Template(e.to_string()))?;

        let os_id = info
            .entries
            .iter()
            .find(|(n, _)| *n == "os")
            .and_then(|(_, v)| {
                if let InfoValue::Map(m) = v {
                    m.get("id").cloned()
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let logo = crate::logo::detect(&os_id);
        let info_lines: Vec<&str> = raw.lines().collect();
        let rendered = crate::logo::render(logo, info_lines.len());
        let logow = crate::logo::logo_width(&rendered) + 3;
        let max = cmp::max(rendered.len(), info_lines.len());
        let mut out = String::with_capacity(raw.len() + rendered.len() * 60);

        for i in 0..max {
            match (i < rendered.len(), i < info_lines.len()) {
                (true, true) => {
                    let vl = crate::logo::visible_len(&rendered[i]);
                    if vl < logow {
                        out.push_str(&rendered[i]);
                        out.push_str(&" ".repeat(logow - vl));
                    } else {
                        out.push_str(&rendered[i]);
                        out.push(' ');
                    }
                    out.push_str(info_lines[i]);
                }
                (true, false) => {
                    let vl = crate::logo::visible_len(&rendered[i]);
                    if vl < logow {
                        out.push_str(&rendered[i]);
                        out.push_str(&" ".repeat(logow - vl));
                    } else {
                        out.push_str(&rendered[i]);
                        out.push(' ');
                    }
                }
                (false, true) => {
                    out.push_str(&" ".repeat(logow));
                    out.push_str(info_lines[i]);
                }
                (false, false) => {}
            }
            out.push('\n');
        }

        Ok(out)
    }
}
