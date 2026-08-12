pub struct Logo {
    pub lines: &'static [&'static str],
    pub colors: &'static [&'static str],
}

pub fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s.chars().peekable();
    #[allow(clippy::while_let_on_iterator)]
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip an escape sequence without counting its bytes as cells.
            // CSI (ESC [ ... m) ends at the first alphabetic byte; OSC
            // hyperlinks (ESC ] 8 ; ; url ESC \, Phase 7.7) must be skipped
            // wholesale or the URL would be counted as visible columns and
            // break `--frame` width math.
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    while let Some(next) = chars.next() {
                        if next.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some(']') => {
                    // OSC: consume until the ST terminator (ESC \) or BEL.
                    while let Some(next) = chars.next() {
                        if next == '\x1b' {
                            if chars.peek() == Some(&'\\') {
                                chars.next();
                            }
                            break;
                        }
                        if next == '\x07' {
                            break;
                        }
                    }
                }
                _ => {
                    // Lone ESC or unsupported sequence: drop the ESC itself.
                }
            }
        } else {
            // Phase 7.2: count display columns, not chars — Nerd Font icons and
            // CJK glyphs are double-width, so naive +1 misaligns icon'd rows.
            len += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        }
    }
    len
}

pub fn render(logo: &Logo, target_height: usize) -> Vec<String> {
    let mut out = Vec::with_capacity(target_height);
    for line in logo.lines {
        let mut s = line.to_string();
        // ponytail: handle both ${N} (our logos) and $N (fastfetch logos)
        for (i, color) in logo.colors.iter().enumerate() {
            let n = i + 1;
            let braced = format!("${{{}}}", n);
            let plain = format!("${}", n);
            s = s.replace(&braced, color);
            s = s.replace(&plain, color);
        }
        out.push(s);
    }
    while out.len() < target_height {
        out.push(String::new());
    }
    out
}

pub fn logo_width(rendered: &[String]) -> usize {
    rendered.iter().map(|l| visible_len(l)).max().unwrap_or(0)
}

/// Interpolate between `stops` at position `t` in [0, 1] (Phase 7.6 logo
/// gradients). Linear between consecutive stops; clamps at the ends.
fn interpolate_stops(stops: &[[u8; 3]], t: f64) -> [u8; 3] {
    if stops.is_empty() {
        return [255, 255, 255];
    }
    if stops.len() == 1 {
        return stops[0];
    }
    let scaled = t.clamp(0.0, 1.0) * (stops.len() - 1) as f64;
    let i = (scaled.floor() as usize).min(stops.len() - 2);
    let frac = scaled - i as f64;
    let (a, b) = (stops[i], stops[i + 1]);
    [
        (a[0] as f64 + (b[0] as f64 - a[0] as f64) * frac) as u8,
        (a[1] as f64 + (b[1] as f64 - a[1] as f64) * frac) as u8,
        (a[2] as f64 + (b[2] as f64 - a[2] as f64) * frac) as u8,
    ]
}

/// Render a logo with a per-line brand gradient (Phase 7.6): the first color
/// token (${1}) is replaced with a truecolor code interpolated across the
/// theme's gradient stops by row index — the classic neofetch/fastfetch
/// vertical fade. Other tokens keep their static colors.
pub fn render_gradient(logo: &Logo, target_height: usize, stops: &[[u8; 3]]) -> Vec<String> {
    let mut out = Vec::with_capacity(target_height);
    let total = logo.lines.len().max(1);
    for (idx, line) in logo.lines.iter().enumerate() {
        let t = if total <= 1 {
            0.0
        } else {
            idx as f64 / (total - 1) as f64
        };
        let rgb = interpolate_stops(stops, t);
        let code = format!("\x1b[38;2;{};{};{}m", rgb[0], rgb[1], rgb[2]);
        let mut s = line.to_string();
        for (i, color) in logo.colors.iter().enumerate() {
            let n = i + 1;
            let braced = format!("${{{}}}", n);
            let plain = format!("${}", n);
            if i == 0 {
                s = s.replace(&braced, &code);
                s = s.replace(&plain, &code);
            } else {
                s = s.replace(&braced, color);
                s = s.replace(&plain, color);
            }
        }
        out.push(s);
    }
    while out.len() < target_height {
        out.push(String::new());
    }
    out
}

use crate::fastfetch_logos::{fastfetch_logo, make_logo};
use crate::logo_data::*;

/// Cache of made fastfetch logos keyed by their source string. `make_logo`
/// leaks a `Box` per call (returns `&'static`), so caching avoids a fresh
/// leak on every `detect()` — important for `--live` refresh loops.
static FF_LOGO_CACHE: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<&'static str, &'static Logo>>,
> = std::sync::OnceLock::new();

fn cached_fastfetch_logo(src: &'static str) -> &'static Logo {
    let cache =
        FF_LOGO_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Some(logo) = cache.lock().unwrap().get(src) {
        return logo;
    }
    let logo = make_logo(src);
    cache.lock().unwrap().insert(src, logo);
    logo
}

pub fn detect(module_type: &str) -> &'static Logo {
    // First check our custom high-quality logos
    if let Some(custom) = detect_custom(module_type) {
        // Prefer the larger art: the compact custom logos (e.g. CachyOS ~10
        // lines) get lost next to a 20-line info block — fastfetch's versions
        // are often 2-3x taller. Whichever has more lines wins.
        if let Some(src) = fastfetch_logo(module_type) {
            let ff = cached_fastfetch_logo(src);
            if ff.lines.len() > custom.lines.len() {
                return ff;
            }
        }
        return custom;
    }
    // Then check fastfetch-sourced logos (527+ distros)
    if let Some(src) = fastfetch_logo(module_type) {
        return cached_fastfetch_logo(src);
    }
    // macOS fallback
    if cfg!(target_os = "macos") {
        return &MACOS_LOGO;
    }
    &GENERIC_LOGO
}

fn detect_custom(module_type: &str) -> Option<&'static Logo> {
    match module_type {
        "arch" | "arcolinux" => Some(&ARCH_LOGO),
        "cachyos" => Some(&CACHYOS_LOGO),
        "artix" => Some(&ARTIX_LOGO),
        "manjaro" => Some(&MANJARO_LOGO),
        "endeavouros" => Some(&ENDEAVOUROS_LOGO),
        "garuda" => Some(&GARUDA_LOGO),
        "debian" | "raspbian" => Some(&DEBIAN_LOGO),
        "ubuntu" => Some(&UBUNTU_LOGO),
        "ubuntu-budgie" | "budgie" => Some(&UBUNTU_BUDGIE_LOGO),
        "ubuntu-mate" | "ubuntumate" => Some(&UBUNTU_MATE_LOGO),
        "ubuntu-kylin" | "kylin" => Some(&UBUNTU_KYLIN_LOGO),
        "linuxmint" => Some(&LINUX_MINT_LOGO),
        "pop" | "popos" | "pop_os" => Some(&POP_OS_LOGO),
        "elementary" | "elementaryos" => Some(&ELEMENTARY_LOGO),
        "zorin" | "zorinos" => Some(&ZORIN_LOGO),
        "fedora" => Some(&FEDORA_LOGO),
        "nixos" => Some(&NIXOS_LOGO),
        "gentoo" => Some(&GENTOO_LOGO),
        "alpine" => Some(&ALPINE_LOGO),
        "void" => Some(&VOID_LOGO),
        "centos" => Some(&CENTOS_LOGO),
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => Some(&OPENSUSE_LOGO),
        "kali" => Some(&KALI_LOGO),
        "mx" | "mxlinux" | "mx-linux" => Some(&MX_LINUX_LOGO),
        "antix" => Some(&ANTIX_LOGO),
        "pclinuxos" | "pclinux" => Some(&PCLINUXOS_LOGO),
        "slackware" => Some(&SLACKWARE_LOGO),
        "puppy" | "puppylinux" | "puppy-linux" => Some(&PUPPY_LOGO),
        "tinycore" | "tinycorelinux" | "tiny-core" => Some(&TINYCORE_LOGO),
        "archarm" | "arch-arm" | "archlinuxarm" => Some(&ARCH_ARM_LOGO),
        "biglinux" => Some(&BIGLINUX_LOGO),
        "linuxlite" | "linux-lite" | "lite" => Some(&LINUX_LITE_LOGO),
        "peppermint" => Some(&PEPPERMINT_LOGO),
        "bodhi" | "bodhilinux" | "bodhi-linux" => Some(&BODHI_LOGO),
        "trisquel" | "trisquelinux" => Some(&TRISQUEL_LOGO),
        "pureos" | "pure-os" => Some(&PUREOS_LOGO),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_len_plain_text() {
        assert_eq!(visible_len("hello"), 5);
        assert_eq!(visible_len(""), 0);
    }

    #[test]
    fn visible_len_ansi_codes_not_counted() {
        assert_eq!(visible_len("\x1b[31mred\x1b[0m"), 3);
    }

    #[test]
    fn visible_len_osc_hyperlink() {
        let input = "\x1b]8;;https://example.com\x1b\\click here\x1b]8;;\x1b\\";
        assert_eq!(visible_len(input), 10);
    }

    #[test]
    fn detect_returns_static_logo() {
        let logo = detect("arch");
        assert!(!logo.lines.is_empty());
    }

    #[test]
    fn detect_unknown_distro_returns_generic() {
        let logo = detect("unknown_distro_xyz");
        assert!(!logo.lines.is_empty());
    }

    #[test]
    fn logo_struct_has_matching_line_color_counts() {
        // Colors map to ${1}, ${2}, ${3} tokens in lines, not to line count.
        // All logos use at least ${1} and ${2} (2 colors), some use ${3} (3 colors).
        let logos = [&ARCH_LOGO, &UBUNTU_LOGO, &FEDORA_LOGO, &GENERIC_LOGO];
        for logo in &logos {
            assert!(
                logo.colors.len() >= 2 && logo.colors.len() <= 3,
                "Logo colors count unexpected: {} colors (expected 2-3)",
                logo.colors.len()
            );
        }
    }
}
