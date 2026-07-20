use std::cmp;
use tera::{Context as TeraContext, Tera};

use crate::{InfoValue, SystemInfo};

pub struct TeraEngine {
    tera: Tera,
    template_name: String,
}

impl TeraEngine {
    pub fn new_default() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_template("default", include_str!("../../templates/default.tera"))
            .expect("default template is valid");
        TeraEngine {
            tera,
            template_name: "default".to_string(),
        }
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
        ctx.insert("theme_reset", &theme.reset);

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
        let logow = crate::logo::logo_width(logo) + 3;

        let info_lines: Vec<&str> = raw.lines().collect();
        let max = cmp::max(logo.len(), info_lines.len());
        let mut out = String::with_capacity(raw.len() + logo.len() * 60);

        for i in 0..max {
            match (i < logo.len(), i < info_lines.len()) {
                (true, true) => {
                    let padded = format!("{:width$}", logo[i], width = logow);
                    out.push_str(&theme.keys);
                    out.push_str(&padded);
                    out.push_str(&theme.reset);
                    out.push_str(info_lines[i]);
                }
                (true, false) => {
                    let padded = format!("{:width$}", logo[i], width = logow);
                    out.push_str(&theme.keys);
                    out.push_str(&padded);
                    out.push_str(&theme.reset);
                }
                (false, true) => {
                    let pad: String = std::iter::repeat(' ').take(logow).collect();
                    out.push_str(&pad);
                    out.push_str(info_lines[i]);
                }
                (false, false) => {}
            }
            out.push('\n');
        }

        Ok(out)
    }
}
