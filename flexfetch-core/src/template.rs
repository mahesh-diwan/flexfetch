use std::cmp;
use std::collections::HashMap;
use std::sync::OnceLock;
use tera::{Context as TeraContext, Tera, Value};

use crate::{InfoValue, SystemInfo};

use crate::image_logo::{
    get_distro_logo_path, get_module_logo_path, ImageLogo, ImageProtocol, LogoMode,
};

struct PaletteDisplayFilter;

// ponytail: palette_display filter — swaps block char in ANSI-colored strings
impl tera::Filter for PaletteDisplayFilter {
    fn filter(&self, _value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let style = args
            .get("style")
            .and_then(|v| v.as_str())
            .unwrap_or("blocks");
        let colors = args
            .get("colors")
            .and_then(|v| v.as_array())
            .ok_or_else(|| tera::Error::msg("palette_display requires a colors array"))?;

        let ch = match style {
            "squares" => "\u{2593}", // ▓
            "dots" => "\u{25CF}",    // ●
            _ => "\u{2588}",         // █ (blocks)
        };

        let parts: Vec<String> = colors
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.replace('\u{2588}', ch))
            .collect();

        Ok(Value::String(parts.join(" ")))
    }
}

static CACHED_TERA: OnceLock<Tera> = OnceLock::new();

fn get_tera() -> &'static Tera {
    CACHED_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        tera.add_raw_template("default", include_str!("../../templates/default.tera"))
            .expect("default template is valid");
        tera.register_filter("palette_display", PaletteDisplayFilter);
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
        ctx.insert("display_palette_style", &config.display.palette_style);

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
                if let crate::InfoValue::Scalar(s) = v {
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
