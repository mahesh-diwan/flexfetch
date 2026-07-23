use crate::{Config, SystemInfo};
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

    let bg = "#1e1e2e";
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

    let mut body = String::with_capacity(text.len() * 2);
    for (i, line_spans) in lines.iter().enumerate() {
        if i > 0 {
            body.push('\n');
        }
        body.push_str(&spans_to_html_line(line_spans));
    }

    Ok(format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>flexfetch</title></head>
<body style="background:#1e1e2e;color:#cdd6f4;margin:0;padding:20px">
<pre style="font-family:monospace;font-size:14px;line-height:1.5">{body}</pre>
</body>
</html>"#
    ))
}

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

    let mut img =
        image::ImageBuffer::from_pixel(img_w, img_h, image::Rgba([0x1eu8, 0x1e, 0x2e, 0xff]));

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
