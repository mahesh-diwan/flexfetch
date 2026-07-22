use std::cmp;
use std::sync::OnceLock;
use tera::{Context as TeraContext, Tera};

use crate::{InfoValue, SystemInfo};

static CACHED_TERA: OnceLock<Tera> = OnceLock::new();

fn get_tera() -> &'static Tera {
    CACHED_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        tera.add_raw_template("default", include_str!("../../templates/default.tera"))
            .expect("default template is valid");
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

        let theme = crate::theme::resolve(config);
        ctx.insert("theme_title", &theme.title);
        ctx.insert("theme_keys", &theme.keys);
        ctx.insert("theme_values", &theme.values);
        ctx.insert("theme_sep", &theme.sep);
        ctx.insert("theme_section", &theme.section);
        ctx.insert("theme_reset", &theme.reset);
        ctx.insert("theme_gradient", &theme.gradient);
        if let (Some(&s), Some(&e)) = (theme.gradient_colors.first(), theme.gradient_colors.get(1))
        {
            ctx.insert("theme_gradient_start", &s);
            ctx.insert("theme_gradient_end", &e);
        } else {
            ctx.insert("theme_gradient_start", &[0u8; 3]);
            ctx.insert("theme_gradient_end", &[255u8; 3]);
        }

        let raw = self
            .tera
            .render(&self.template_name, &ctx)
            .map_err(|e| crate::Error::Template(e.to_string()))?;

        // Apply gradient to first line if enabled
        let raw = if theme.gradient && theme.gradient_colors.len() >= 2 {
            let start = theme.gradient_colors[0];
            let end = theme.gradient_colors[1];
            let mut out = String::new();
            for (i, line) in raw.lines().enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                if i == 0 {
                    out.push_str(&crate::theme::gradient_text(line, start, end));
                } else {
                    out.push_str(line);
                }
            }
            out
        } else {
            raw
        };

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
