use std::cmp;

#[cfg(feature = "tera")]
use std::sync::OnceLock;
#[cfg(feature = "tera")]
use tera::{Context as TeraContext, Tera};

use crate::{InfoValue, SystemInfo};

use crate::image_logo::{get_distro_logo_path, ImageProtocol, LogoMode};
#[cfg(feature = "tera")]
use crate::image_logo::{get_module_logo_path, ImageLogo};
use crate::logo::visible_len;

pub fn frame_wrap(text: &str, style: &str, color: &str) -> String {
    let (tl, tr, bl, br, h, v) = match style {
        "double" => ("╔", "╗", "╚", "╝", "═", "║"),
        "decorative" | "single" => ("┌", "┐", "└", "┘", "─", "│"),
        _ => return text.to_string(),
    };
    let max_w = text.lines().map(visible_len).max().unwrap_or(40);
    let mut result = String::new();
    result.push_str(&format!("{color}{tl}{}{tr}\x1b[0m\n", h.repeat(max_w)));
    for line in text.lines() {
        let padding = max_w.saturating_sub(visible_len(line));
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

#[cfg(feature = "tera")]
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
            let s = c.as_str()?;
            let parts: Vec<u8> = s.split(',').filter_map(|p| p.parse().ok()).collect();
            if parts.len() != 3 {
                return None;
            }
            let (r, g, b) = (parts[0], parts[1], parts[2]);
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

#[cfg(feature = "tera")]
fn progress_bar_filter(
    value: &serde_json::Value,
    args: &std::collections::HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let percent = match value {
        serde_json::Value::Number(n) => n.as_u64().unwrap_or(0) as u8,
        serde_json::Value::String(s) => s.parse::<u8>().unwrap_or(0),
        _ => 0,
    };
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

#[cfg(feature = "tera")]
static CACHED_TERA: OnceLock<Tera> = OnceLock::new();

#[cfg(feature = "tera")]
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
    #[cfg(feature = "tera")]
    tera: Tera,
    #[cfg(feature = "tera")]
    template_name: String,
}

impl TeraEngine {
    pub fn new_default() -> Self {
        #[cfg(feature = "tera")]
        {
            TeraEngine {
                tera: get_tera().clone(),
                template_name: "default".to_string(),
            }
        }
        #[cfg(not(feature = "tera"))]
        {
            TeraEngine {}
        }
    }

    /// Template source used for module filtering. Without the `tera` feature
    /// there is no template, so this is empty (and `run_selected`'s filter
    /// passes every module — there is no custom template to honor).
    pub fn default_template_content() -> &'static str {
        #[cfg(feature = "tera")]
        {
            include_str!("../../templates/default.tera")
        }
        #[cfg(not(feature = "tera"))]
        {
            ""
        }
    }

    pub fn render(&self, info: &SystemInfo, config: &crate::Config) -> crate::Result<String> {
        #[cfg(feature = "tera")]
        let raw = self.render_tera(info, config)?;
        #[cfg(not(feature = "tera"))]
        let raw = render_plain(info, config);

        let info_lines: Vec<&str> = raw.lines().collect();
        // Try image logo first (block characters work in any truecolor terminal)
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
        let rendered = if let Some(img_path) = get_distro_logo_path(&os_id) {
            let resolved = crate::image_logo::ImageLogo::resolve_path(&img_path);
            if std::path::Path::new(&resolved).exists() {
                let logo = crate::image_logo::ImageLogo::new(&resolved).with_size(16, 8);
                let ansi = logo.render(ImageProtocol::Block, LogoMode::Image);
                if !ansi.is_empty() {
                    ansi.lines().map(String::from).collect::<Vec<_>>()
                } else {
                    let ascii = crate::logo::detect(&os_id);
                    crate::logo::render(ascii, info_lines.len())
                }
            } else {
                let ascii = crate::logo::detect(&os_id);
                crate::logo::render(ascii, info_lines.len())
            }
        } else {
            let ascii = crate::logo::detect(&os_id);
            crate::logo::render(ascii, info_lines.len())
        };
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

#[cfg(feature = "tera")]
impl TeraEngine {
    /// Tera-backed rendering: build the context (module values, placeholders,
    /// display config, theme, icons, image logos) and render default.tera.
    fn render_tera(&self, info: &SystemInfo, config: &crate::Config) -> crate::Result<String> {
        let mut ctx = TeraContext::new();
        for (name, value) in &info.entries {
            let json_val = serde_json::to_value(value)
                .map_err(|e| crate::Error::Template(format!("serialize {name}: {e}")))?;
            ctx.insert(*name, &json_val);
        }

        // Add structured placeholders for all modules to avoid Tera "missing variable" errors
        // when modules are not selected but referenced in template conditionals.
        // Use empty objects/arrays/strings that are falsy in Tera conditionals.
        let all_modules = [
            ("title", serde_json::Value::String(String::new())),
            ("os", serde_json::Value::Object(serde_json::Map::new())),
            ("host", serde_json::Value::String(String::new())),
            ("kernel", serde_json::Value::String(String::new())),
            ("uptime", serde_json::Value::String(String::new())),
            ("locale", serde_json::Value::Object(serde_json::Map::new())),
            (
                "packages",
                serde_json::Value::Object(serde_json::Map::new()),
            ),
            ("shell", serde_json::Value::String(String::new())),
            (
                "terminal",
                serde_json::Value::Object(serde_json::Map::new()),
            ),
            ("de", serde_json::Value::String(String::new())),
            ("wm", serde_json::Value::Object(serde_json::Map::new())),
            ("cpu", serde_json::Value::Object(serde_json::Map::new())),
            (
                "cpucache",
                serde_json::Value::Object(serde_json::Map::new()),
            ),
            ("cpuusage", serde_json::Value::String(String::new())),
            ("memory", serde_json::Value::Object(serde_json::Map::new())),
            ("gpu", serde_json::Value::Array(vec![])),
            ("disk", serde_json::Value::Array(vec![])),
            ("network", serde_json::Value::Array(vec![])),
            ("battery", serde_json::Value::Object(serde_json::Map::new())),
            ("processes", serde_json::Value::String(String::new())),
            (
                "temperature",
                serde_json::Value::Object(serde_json::Map::new()),
            ),
            ("resolution", serde_json::Value::String(String::new())),
            ("display", serde_json::Value::String(String::new())),
            ("colors", serde_json::Value::Array(vec![])),
            ("custom", serde_json::Value::Array(vec![])),
            ("publicip", serde_json::Value::String(String::new())),
            ("wifi", serde_json::Value::Object(serde_json::Map::new())),
            ("git", serde_json::Value::Object(serde_json::Map::new())),
            ("project", serde_json::Value::Object(serde_json::Map::new())),
            ("context", serde_json::Value::Object(serde_json::Map::new())),
            ("health", serde_json::Value::Object(serde_json::Map::new())),
        ];
        for (module, placeholder) in all_modules {
            if !info.entries.iter().any(|(k, _)| *k == module) {
                ctx.insert(module, &placeholder);
            }
        }

        ctx.insert("display_separator", &config.display.separator);
        ctx.insert("display_key_width", &config.display.key_width);
        ctx.insert("display_palette_style", &config.display.palette_style);

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

        // Add fastfetch-style icons
        ctx.insert("icon_os", &config.display.icon_os);
        ctx.insert("icon_kernel", &config.display.icon_kernel);
        ctx.insert("icon_host", &config.display.icon_host);
        ctx.insert("icon_uptime", &config.display.icon_uptime);
        ctx.insert("icon_locale", &config.display.icon_locale);
        ctx.insert("icon_cpu", &config.display.icon_cpu);
        ctx.insert("icon_gpu", &config.display.icon_gpu);
        ctx.insert("icon_memory", &config.display.icon_memory);
        ctx.insert("icon_swap", &config.display.icon_swap);
        ctx.insert("icon_disk", &config.display.icon_disk);
        ctx.insert("icon_network", &config.display.icon_network);
        ctx.insert("icon_interface", &config.display.icon_interface);
        ctx.insert("icon_resolution", &config.display.icon_resolution);
        ctx.insert("icon_battery", &config.display.icon_battery);
        ctx.insert("icon_processes", &config.display.icon_processes);
        ctx.insert("icon_end", &config.display.icon_end);
        ctx.insert("icon_temp", &config.display.icon_temp);

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
            "cpucache",
            "cpuusage",
            "memory",
            "disk",
            "gpu",
            "network",
            "battery",
            "processes",
            "temperature",
            "resolution",
            "display",
            "colors",
            "custom",
            "publicip",
            "wifi",
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
            .map_err(|e| crate::Error::Template(format!("{:?}", e)))?;
        Ok(raw)
    }
}

// ---------------------------------------------------------------------------
// Plain fallback renderer (used when the `tera` feature is off — the minimal
// build). Produces fastfetch-style `├─ Key: value` lines with the same labels
// the default template uses, so output stays readable without tera.
// ---------------------------------------------------------------------------

#[cfg(not(feature = "tera"))]
fn render_plain(info: &SystemInfo, config: &crate::Config) -> String {
    let theme = crate::theme::resolve(config);
    let mut out = String::new();

    // Title + separator (mirrors default.tera's title block)
    if let Some(InfoValue::Scalar(title)) = info
        .entries
        .iter()
        .find(|(n, _)| *n == "title")
        .map(|(_, v)| v)
    {
        if !title.is_empty() {
            out.push_str(&format!("{}{}{}\n", theme.title, title, theme.reset));
            out.push_str(&format!(
                "{}────────────────────────────{}\n",
                theme.sep, theme.reset
            ));
        }
    }

    // Build label/text rows first so the connector can be chosen up front:
    // every line but the last uses ├─, the final line uses the ╰─ end connector
    // (mirroring the default template). Deciding at build time avoids byte-level
    // surgery on multi-byte UTF-8 box characters.
    let mut rows: Vec<(String, String)> = Vec::new();
    for (name, value) in &info.entries {
        if *name == "title" || *name == "separator" {
            continue;
        }
        let Some(text) = plain_value(name, value) else {
            continue;
        };
        if text.is_empty() {
            continue;
        }
        rows.push((label_for(name), text));
    }
    let last = rows.len().saturating_sub(1);
    for (i, (label, text)) in rows.into_iter().enumerate() {
        let connector = if i == last { "╰─" } else { "├─" };
        out.push_str(&format!(
            "{}{connector} {}{label}: {text}{}",
            theme.keys, theme.values, theme.reset,
        ));
        if i < last {
            out.push('\n');
        }
    }
    out
}

#[cfg(not(feature = "tera"))]
fn label_for(name: &str) -> String {
    match name {
        "os" => "OS".into(),
        "host" => "Host".into(),
        "kernel" => "Kernel".into(),
        "uptime" => "Uptime".into(),
        "locale" => "Locale".into(),
        "project" => "Project".into(),
        "git" => "Git".into(),
        "context" => "Context".into(),
        "cpu" => "CPU".into(),
        "cpucache" => "Cache".into(),
        "cpuusage" => "CPU Usage".into(),
        "gpu" => "GPU".into(),
        "memory" => "Memory".into(),
        "swap" => "Swap".into(),
        "disk" => "Disk".into(),
        "network" => "Network".into(),
        "resolution" => "Resolution".into(),
        "display" => "Display".into(),
        "wifi" => "WiFi".into(),
        "publicip" => "Public IP".into(),
        "battery" => "Battery".into(),
        "temperature" => "Temp".into(),
        "bluetooth" => "Bluetooth".into(),
        "media" => "Media".into(),
        "processes" => "Processes".into(),
        "packages" => "Packages".into(),
        "shell" => "Shell".into(),
        "terminal" => "Terminal".into(),
        "de" => "DE".into(),
        "wm" => "WM".into(),
        "colors" => "Colors".into(),
        "custom" => "Custom".into(),
        "health" => "Health".into(),
        other => other.to_string(),
    }
}

#[cfg(not(feature = "tera"))]
fn plain_value(name: &str, value: &InfoValue) -> Option<String> {
    match value {
        InfoValue::Scalar(s) => {
            if s.is_empty() || s == "unknown" || s == "no media" {
                None
            } else {
                Some(s.clone())
            }
        }
        InfoValue::Map(m) => match name {
            "os" => m
                .get("pretty_name")
                .or_else(|| m.get("name"))
                .filter(|s| !s.is_empty())
                .cloned(),
            "memory" => {
                let used = m.get("used").cloned().unwrap_or_default();
                let total = m.get("total").cloned().unwrap_or_default();
                let pct = m.get("percent").cloned().unwrap_or_default();
                Some(format!("{used} / {total} ({pct})"))
            }
            "swap" => {
                let used = m.get("swap_used").cloned().unwrap_or_default();
                let total = m.get("swap_total").cloned().unwrap_or_default();
                let pct = m.get("swap_percent").cloned().unwrap_or_default();
                if total.is_empty() {
                    None
                } else {
                    Some(format!("{used} / {total} ({pct})"))
                }
            }
            "cpu" => {
                let model = m.get("model").cloned().unwrap_or_default();
                let cores = m.get("cores").cloned().unwrap_or_default();
                let freq = m.get("freq_mhz").cloned().unwrap_or_default();
                if model.is_empty() {
                    None
                } else {
                    Some(format!("{model} ({cores} cores) @ {freq} MHz"))
                }
            }
            "cpucache" => {
                let mut parts = Vec::new();
                for (k, label) in [("l1d", "L1d"), ("l1i", "L1i"), ("l2", "L2"), ("l3", "L3")] {
                    if let Some(v) = m.get(k) {
                        if !v.is_empty() {
                            parts.push(format!("{label} {v}"));
                        }
                    }
                }
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join(" "))
                }
            }
            "battery" => {
                let pct = m.get("percent").cloned().unwrap_or_default();
                let status = m.get("status").cloned().unwrap_or_default();
                if pct.is_empty() {
                    None
                } else if status.is_empty() {
                    Some(pct)
                } else {
                    Some(format!("{pct} ({status})"))
                }
            }
            "git" => {
                let branch = m.get("branch").cloned().unwrap_or_default();
                if branch.is_empty() {
                    return None;
                }
                let mut s = branch;
                if let Some(a) = m.get("ahead") {
                    if !a.is_empty() {
                        s.push_str(&format!(" ↑{a}"));
                    }
                }
                if let Some(b) = m.get("behind") {
                    if !b.is_empty() {
                        s.push_str(&format!(" ↓{b}"));
                    }
                }
                if let Some(d) = m.get("dirty") {
                    if !d.is_empty() {
                        s.push_str(&format!(" [{d} dirty]"));
                    }
                }
                Some(s)
            }
            "project" => {
                let t = m.get("type").cloned().unwrap_or_default();
                let n = m.get("name").cloned().unwrap_or_default();
                if t.is_empty() {
                    None
                } else if n.is_empty() {
                    Some(t)
                } else {
                    Some(format!("{t} — {n}"))
                }
            }
            "health" => {
                let score = m.get("score").cloned().unwrap_or_default();
                if score.is_empty() {
                    None
                } else {
                    let grade = m.get("grade").cloned().unwrap_or_default();
                    let mut s = format!("{score}/100 ({grade})");
                    if let Some(notes) = m.get("notes") {
                        if !notes.is_empty() {
                            s.push_str(&format!(" — {notes}"));
                        }
                    }
                    Some(s)
                }
            }
            "locale" => m.get("lang").filter(|s| !s.is_empty()).cloned(),
            "wm" => m.get("name").filter(|s| !s.is_empty()).cloned(),
            "context" => {
                let mut parts = Vec::new();
                if let Some(c) = m.get("container") {
                    if !c.is_empty() {
                        parts.push(c.clone());
                    }
                }
                if let Some(v) = m.get("venv") {
                    if !v.is_empty() {
                        parts.push(v.clone());
                    }
                }
                if let Some(s) = m.get("ssh") {
                    if !s.is_empty() {
                        parts.push(s.clone());
                    }
                }
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join(", "))
                }
            }
            _ => {
                let vals: Vec<&String> = m.values().filter(|v| !v.is_empty()).collect();
                if vals.is_empty() {
                    None
                } else {
                    Some(
                        vals.iter()
                            .take(2)
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                }
            }
        },
        InfoValue::List(l) => {
            let nonempty: Vec<&String> = l.iter().filter(|s| !s.is_empty()).collect();
            if nonempty.is_empty() {
                None
            } else {
                Some(
                    nonempty
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )
            }
        }
        InfoValue::Table(rows) => {
            let texts: Vec<String> = rows
                .iter()
                .filter_map(|row| {
                    if let Some(label) = row.get("label") {
                        if let Some(val) = row.get("value") {
                            if !val.is_empty() {
                                return Some(format!("{label}: {val}"));
                            }
                        }
                    }
                    row.get("value").filter(|v| !v.is_empty()).cloned()
                })
                .collect();
            if texts.is_empty() {
                None
            } else {
                Some(texts.join(", "))
            }
        }
    }
}
