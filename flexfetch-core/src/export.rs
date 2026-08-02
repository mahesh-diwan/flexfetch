use crate::{Config, InfoValue, SystemInfo};
use std::path::Path;

// ANSI color table: index → RGB
const ANSI_COLORS: &[[u8; 3]] = &[
    [0, 0, 0],       // 30 black
    [170, 0, 0],     // 31 red
    [0, 170, 0],     // 32 green
    [170, 85, 0],    // 33 yellow
    [0, 0, 170],     // 34 blue
    [170, 0, 170],   // 35 magenta
    [0, 170, 170],   // 36 cyan
    [170, 170, 170], // 37 white
];
const ANSI_BRIGHT_COLORS: &[[u8; 3]] = &[
    [85, 85, 85],    // 90 bright black
    [255, 85, 85],   // 91 bright red
    [85, 255, 85],   // 92 bright green
    [255, 255, 85],  // 93 bright yellow
    [85, 85, 255],   // 94 bright blue
    [255, 85, 255],  // 95 bright magenta
    [85, 255, 255],  // 96 bright cyan
    [255, 255, 255], // 97 bright white
];

fn theme_bg_color(config: &Config) -> [u8; 3] {
    match config.display.theme.as_deref().unwrap_or("") {
        "solarized-light" | "one-light" | "rose-pine-dawn" | "everforest-light" => [253, 246, 227],
        _ => [30, 30, 46], // default dark (#1e1e2e)
    }
}

#[derive(Clone)]
struct Span<'a> {
    text: &'a str,
    color: [u8; 3],
}

/// Parse ANSI-colored text into spans with RGB colors.
fn parse_ansi(text: &str) -> Vec<Span<'_>> {
    let mut spans = Vec::new();
    let mut current_color = [170, 170, 170]; // default gray
    let mut last = 0;

    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Flush text before this escape
            if last < i {
                spans.push(Span {
                    text: &text[last..i],
                    color: current_color,
                });
            }
            // Parse CSI sequence: \x1b[<params>m
            let mut j = i + 2;
            let mut params = Vec::new();
            let mut num = 0u16;
            let mut has_num = false;
            loop {
                if j >= bytes.len() {
                    break;
                }
                match bytes[j] {
                    b'0'..=b'9' => {
                        num = num * 10 + (bytes[j] - b'0') as u16;
                        has_num = true;
                        j += 1;
                    }
                    b';' => {
                        params.push(if has_num { num } else { 0 });
                        num = 0;
                        has_num = false;
                        j += 1;
                    }
                    b'm' => {
                        if has_num {
                            params.push(num);
                        }
                        // Apply SGR parameters
                        for &p in &params {
                            match p {
                                0 => current_color = [170, 170, 170],
                                30..=37 => {
                                    let idx = (p - 30) as usize;
                                    if idx < ANSI_COLORS.len() {
                                        current_color = ANSI_COLORS[idx];
                                    }
                                }
                                90..=97 => {
                                    let idx = (p - 90) as usize;
                                    if idx < ANSI_BRIGHT_COLORS.len() {
                                        current_color = ANSI_BRIGHT_COLORS[idx];
                                    }
                                }
                                38 => {
                                    // 38;2;r;g;b or 38;5;n — consumes rest of params
                                    if params.len() >= 5 && params[1] == 2 {
                                        current_color =
                                            [params[2] as u8, params[3] as u8, params[4] as u8];
                                    } else if params.len() >= 3 && params[1] == 5 {
                                        let c = params[2] as u8;
                                        if c < 16 {
                                            let tbl = if c < 8 {
                                                ANSI_COLORS
                                            } else {
                                                ANSI_BRIGHT_COLORS
                                            };
                                            current_color = tbl[(c % 8) as usize];
                                        }
                                    }
                                    break; // 38 consumes the rest
                                }
                                _ => {}
                            }
                        }
                        // Advance past the 'm' and continue outer loop
                        i = j + 1;
                        last = i;
                        break;
                    }
                    _ => {
                        // Unknown CSI final byte — skip
                        j += 1;
                    }
                }
            }
            // Inner loop broke after processing 'm' — i is already advanced, continue outer loop
            continue;
        }
        i += 1;
    }
    // Flush remaining text
    if last < bytes.len() {
        spans.push(Span {
            text: &text[last..],
            color: current_color,
        });
    }
    spans
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Split spans into per-line groups.
fn spans_per_line<'a>(spans: &[Span<'a>]) -> Vec<Vec<Span<'a>>> {
    let mut lines: Vec<Vec<Span<'a>>> = Vec::new();
    let mut current = Vec::new();
    for span in spans {
        let mut remaining = span.text;
        while let Some(nl_pos) = remaining.find('\n') {
            if nl_pos > 0 {
                current.push(Span {
                    text: &remaining[..nl_pos],
                    color: span.color,
                });
            }
            lines.push(current);
            current = Vec::new();
            remaining = &remaining[nl_pos + 1..];
        }
        if !remaining.is_empty() {
            current.push(Span {
                text: remaining,
                color: span.color,
            });
        }
    }
    if !current.is_empty() || lines.is_empty() {
        lines.push(current);
    }
    lines
}

fn spans_to_html_line(spans: &[Span<'_>]) -> String {
    let mut out = String::new();
    for span in spans {
        if span.text.is_empty() {
            continue;
        }
        let escaped = html_escape(span.text);
        if span.color == [170, 170, 170] {
            out.push_str(&escaped);
        } else {
            out.push_str(&format!(
                "<span style=\"color:#{:02x}{:02x}{:02x}\">{}</span>",
                span.color[0], span.color[1], span.color[2], escaped
            ));
        }
    }
    out
}

fn spans_to_svg_line(spans: &[Span<'_>]) -> String {
    let mut out = String::new();
    for span in spans {
        if span.text.is_empty() {
            continue;
        }
        let escaped = html_escape(span.text);
        if span.color == [170, 170, 170] {
            out.push_str(&escaped);
        } else {
            out.push_str(&format!(
                "<tspan fill=\"#{:02x}{:02x}{:02x}\">{}</tspan>",
                span.color[0], span.color[1], span.color[2], escaped
            ));
        }
    }
    out
}

pub fn export_svg(info: &SystemInfo, config: &Config) -> crate::Result<String> {
    let engine = crate::template::TeraEngine::new_default();
    let text = engine.render(info, config)?;
    let spans = parse_ansi(&text);
    let lines = spans_per_line(&spans);
    let line_count = lines.len();
    let line_height = 20u32;
    let char_width = 9u32;
    let max_chars = text.lines().map(|l| l.chars().count()).max().unwrap_or(40);
    let width = (max_chars as u32) * char_width + 40;
    let height = (line_count as u32) * line_height + 40;

    let bg_rgb = theme_bg_color(config);
    let bg = format!("#{:02x}{:02x}{:02x}", bg_rgb[0], bg_rgb[1], bg_rgb[2]);
    let mut svg = String::with_capacity(1024);
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">"#
    ));
    svg.push_str(&format!(
        r#"<rect width="{width}" height="{height}" fill="{bg}"/>"#
    ));

    for (i, line_spans) in lines.iter().enumerate() {
        let y = 30 + i as u32 * line_height;
        let inner = spans_to_svg_line(line_spans);
        svg.push_str(&format!(
            r#"<text font-family="monospace" font-size="14" x="20" y="{y}">{inner}</text>"#
        ));
    }

    svg.push_str("</svg>");
    Ok(svg)
}

pub fn export_html(info: &SystemInfo, config: &Config) -> crate::Result<String> {
    let engine = crate::template::TeraEngine::new_default();
    let text = engine.render(info, config)?;
    let spans = parse_ansi(&text);
    let lines = spans_per_line(&spans);

    let bg_rgb = theme_bg_color(config);
    let bg_hex = format!("#{:02x}{:02x}{:02x}", bg_rgb[0], bg_rgb[1], bg_rgb[2]);
    let brightness = (bg_rgb[0] as u32 + bg_rgb[1] as u32 + bg_rgb[2] as u32) / 3;
    let fg_hex = if brightness > 128 {
        "#1e1e2e"
    } else {
        "#cdd6f4"
    };

    let mut body = String::with_capacity(text.len() * 2);
    for (i, line_spans) in lines.iter().enumerate() {
        if i > 0 {
            body.push('\n');
        }
        body.push_str(&spans_to_html_line(line_spans));
    }

    Ok(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>flexfetch</title>
<style>
  *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{
    background: {bg_hex};
    color: {fg_hex};
    font-family: "SF Mono", "Cascadia Code", "Fira Code", "JetBrains Mono", Menlo, Monaco, "Courier New", monospace;
    font-size: 14px;
    line-height: 1.6;
    padding: 24px;
    min-height: 100vh;
  }}
  pre {{
    max-width: 90ch;
    margin: 0 auto;
  }}
</style>
</head>
<body>
<pre>{body}</pre>
</body>
</html>"#
    ))
}

#[cfg(feature = "image-logos")]
pub fn export_png(info: &SystemInfo, config: &Config, path: &Path) -> crate::Result<()> {
    let engine = crate::template::TeraEngine::new_default();
    let text = engine.render(info, config)?;
    let spans = parse_ansi(&text);
    let lines = spans_per_line(&spans);
    let line_count = lines.len();
    let max_chars = text.lines().map(|l| l.chars().count()).max().unwrap_or(40);

    let char_w = 9u32;
    let char_h = 18u32;
    let pad = 20u32;
    let img_w = (max_chars as u32) * char_w + pad * 2;
    let img_h = (line_count as u32) * char_h + pad * 2;

    let bg_rgb = theme_bg_color(config);
    let mut img = image::ImageBuffer::from_pixel(
        img_w,
        img_h,
        image::Rgba([bg_rgb[0], bg_rgb[1], bg_rgb[2], 0xff]),
    );

    let mut cy = pad;
    for line_spans in &lines {
        let mut cx = pad;
        for span in line_spans {
            for ch in span.text.chars() {
                if ch != ' ' {
                    let color = image::Rgba([span.color[0], span.color[1], span.color[2], 255]);
                    for dy in 4..char_h - 2 {
                        for dx in 1..char_w - 1 {
                            let px = cx + dx;
                            let py = cy + dy;
                            if px < img_w && py < img_h {
                                img.put_pixel(px, py, color);
                            }
                        }
                    }
                }
                cx += char_w;
            }
        }
        cy += char_h;
    }

    img.save(path)
        .map_err(|e| crate::Error::Template(format!("png save: {e}")))
}

// PNG export needs the `image` crate; the minimal `--no-default-features` build
// drops it, so export_png degrades to a clear error instead of a missing symbol.
#[cfg(not(feature = "image-logos"))]
pub fn export_png(_info: &SystemInfo, _config: &Config, _path: &Path) -> crate::Result<()> {
    Err(crate::Error::Template(
        "PNG export requires the `image-logos` feature (build with --features image-logos)"
            .to_string(),
    ))
}

// ---------------------------------------------------------------------------
// Phase 4.10 — infrastructure exports (ansible / terraform / csv / prometheus)
// ---------------------------------------------------------------------------

/// Ansible facts JSON: `{ "ansible_flexfetch": { "module": ... } }` — each
/// InfoValue is normalized to plain scalars (Maps become nested objects).
/// Lists/Table become arrays, so the output is valid JSON throughout.
pub fn export_ansible(info: &SystemInfo) -> crate::Result<String> {
    let mut facts = serde_json::Map::new();
    for (name, value) in &info.entries {
        facts.insert(name.to_string(), info_value_to_json(value));
    }
    let mut root = serde_json::Map::new();
    root.insert("ansible_flexfetch".into(), serde_json::Value::Object(facts));
    serde_json::to_string_pretty(&serde_json::Value::Object(root))
        .map_err(|e| crate::Error::Template(format!("ansible export: {e}")))
}

/// Terraform-style HCL variable declarations: `variable "os_name" { default = "..." }`.
/// Keys are normalized (dots/spaces → `_`); Maps are flattened to `module_key`.
pub fn export_terraform(info: &SystemInfo) -> crate::Result<String> {
    let mut out = String::new();
    for (name, value) in &info.entries {
        match value {
            InfoValue::Scalar(s) => out.push_str(&format!(
                "variable \"{name}\" {{ default = \"{}\" }}\n",
                s.replace('"', "\\\"")
            )),
            InfoValue::Map(m) => {
                let mut keys: Vec<_> = m.keys().collect();
                keys.sort();
                for k in keys {
                    out.push_str(&format!(
                        "variable \"{name}_{}\" {{ default = \"{}\" }}\n",
                        normalize_key(k),
                        m[k].replace('"', "\\\"")
                    ));
                }
            }
            InfoValue::List(l) => {
                let joined = l
                    .iter()
                    .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("variable \"{name}\" {{ default = [{joined}] }}\n"));
            }
            InfoValue::Table(t) => {
                out.push_str(&format!(
                    "variable \"{name}_count\" {{ default = {} }}\n",
                    t.len()
                ));
            }
        }
    }
    Ok(out)
}

/// Flat `key,value` CSV (one row per leaf value; Maps flattened to `module_key`).
pub fn export_csv(info: &SystemInfo) -> crate::Result<String> {
    let mut out = String::from("key,value\n");
    for (name, value) in &info.entries {
        match value {
            InfoValue::Scalar(s) => out.push_str(&format!("{},{}\n", name, csv_escape(s))),
            InfoValue::Map(m) => {
                let mut keys: Vec<_> = m.keys().collect();
                keys.sort();
                for k in keys {
                    out.push_str(&format!(
                        "{}_{},{}\n",
                        name,
                        normalize_key(k),
                        csv_escape(&m[k])
                    ));
                }
            }
            InfoValue::List(l) => {
                out.push_str(&format!("{},{}\n", name, csv_escape(&l.join(" | "))));
            }
            InfoValue::Table(t) => {
                out.push_str(&format!("{}_count,{}\n", name, t.len()));
            }
        }
    }
    Ok(out)
}

/// OpenMetrics exposition format (Prometheus text protocol v0.0.4): every
/// scalar becomes a gauge named `flexfetch_<module>`, Maps become labelled
/// gauges, numeric-looking values keep their number.
pub fn export_prometheus(info: &SystemInfo) -> crate::Result<String> {
    let mut out = String::new();
    for (name, value) in &info.entries {
        let metric = format!("flexfetch_{}", normalize_key(name));
        match value {
            InfoValue::Scalar(s) => {
                out.push_str(&format!("# HELP {metric} {name}\n# TYPE {metric} gauge\n"));
                out.push_str(&format!("{metric} {}\n", prom_number(s)));
            }
            InfoValue::Map(m) => {
                out.push_str(&format!("# HELP {metric} {name}\n# TYPE {metric} gauge\n"));
                let mut keys: Vec<_> = m.keys().collect();
                keys.sort();
                for k in keys {
                    out.push_str(&format!(
                        "{metric}{{{}=\"{}\"}} {}\n",
                        normalize_key(k),
                        m[k],
                        prom_number(&m[k])
                    ));
                }
            }
            InfoValue::List(_) | InfoValue::Table(_) => {
                // Not a gauge metric; skip structured values in Prometheus format.
            }
        }
    }
    Ok(out)
}

fn info_value_to_json(value: &InfoValue) -> serde_json::Value {
    match value {
        InfoValue::Scalar(s) => serde_json::Value::String(s.clone()),
        InfoValue::Map(m) => serde_json::Value::Object(
            m.iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect(),
        ),
        InfoValue::List(l) => serde_json::Value::Array(
            l.iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect(),
        ),
        InfoValue::Table(t) => serde_json::Value::Array(
            t.iter()
                .map(|row| {
                    serde_json::Value::Object(
                        row.iter()
                            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                            .collect(),
                    )
                })
                .collect(),
        ),
    }
}

fn normalize_key(k: &str) -> String {
    k.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

/// Keep numeric-looking values as bare numbers (Prometheus requires numbers);
/// anything else becomes 1 with the value on the label (best effort).
fn prom_number(s: &str) -> String {
    let t = s.trim();
    if t.parse::<f64>().is_ok() {
        t.to_string()
    } else if let Some(pct) = t.strip_suffix('%') {
        pct.trim().to_string()
    } else if let Some(gi) = t.strip_suffix("GiB") {
        gi.trim().to_string()
    } else if let Some(mi) = t.strip_suffix("MiB") {
        mi.trim().to_string()
    } else {
        "1".to_string()
    }
}

pub fn export_markdown(info: &SystemInfo, config: &Config) -> crate::Result<String> {
    let engine = crate::template::TeraEngine::new_default();
    let text = engine.render(info, config)?;

    // Strip all ANSI escape sequences for plain text
    let mut result = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Skip CSI sequence: \x1b[...<letter>
            i += 2;
            while i < bytes.len() {
                match bytes[i] {
                    b'A'..=b'Z' | b'a'..=b'z' => {
                        i += 1;
                        break;
                    }
                    _ => i += 1,
                }
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn parse_basic_ansi() {
        let input = "\x1b[31mred\x1b[0m normal";
        let spans = parse_ansi(input);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].text, "red");
        assert_eq!(spans[0].color, [170, 0, 0]);
        assert_eq!(spans[1].text, " normal");
    }

    #[test]
    fn parse_truecolor() {
        let input = "\x1b[38;2;255;128;0morange\x1b[0m";
        let spans = parse_ansi(input);
        assert_eq!(spans[0].color, [255, 128, 0]);
    }

    #[test]
    fn parse_bright_color() {
        let input = "\x1b[91mbr\x1b[0m";
        let spans = parse_ansi(input);
        assert_eq!(spans[0].color, [255, 85, 85]);
    }

    #[test]
    fn spans_per_line_splits() {
        let spans = vec![Span {
            text: "hello\nworld",
            color: [255, 0, 0],
        }];
        let lines = spans_per_line(&spans);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].len(), 1);
        assert_eq!(lines[0][0].text, "hello");
        assert_eq!(lines[1][0].text, "world");
    }

    fn sample_info() -> SystemInfo {
        let mut info = SystemInfo::new();
        info.add(
            "os",
            InfoValue::Map(HashMap::from([("pretty_name".into(), "Arch Linux".into())])),
        );
        info.add("kernel", InfoValue::Scalar("6.10.2-arch1-1".into()));
        info.add(
            "cpu",
            InfoValue::Map(HashMap::from([("cores".into(), "12".into())])),
        );
        info
    }

    #[test]
    fn ansible_export_is_valid_json() {
        let out = export_ansible(&sample_info()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert!(v.get("ansible_flexfetch").is_some());
        assert_eq!(v["ansible_flexfetch"]["os"]["pretty_name"], "Arch Linux");
    }

    #[test]
    fn terraform_export_normalizes_keys() {
        let out = export_terraform(&sample_info()).unwrap();
        assert!(out.contains("variable \"os_pretty_name\""));
        assert!(out.contains("variable \"kernel\""));
    }

    #[test]
    fn csv_export_escapes_commas() {
        let out = export_csv(&sample_info()).unwrap();
        assert!(out.starts_with("key,value\n"));
        assert!(out.contains("os_pretty_name,Arch Linux"));
    }

    #[test]
    fn prometheus_export_numbers() {
        let out = export_prometheus(&sample_info()).unwrap();
        assert!(out.contains("# TYPE flexfetch_kernel gauge"));
        // kernel "6.10.2-arch1-1" is not numeric → exported as 1
        assert!(out.contains("flexfetch_kernel 1"));
        // cpu.cores "12" IS numeric → stays numeric
        assert!(out.contains("flexfetch_cpu{cores=\"12\"} 12"));
    }
}
