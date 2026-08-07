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

const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const WHITE: &str = "\x1b[37m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";

const GENERIC_LOGO: Logo = Logo {
    lines: &[
        "           ${1}        ${2}",
        "          ${1}/\\\\       ${2}",
        "         ${1}/  \\\\      ${2}",
        "        ${1}/ /\\\\ \\\\     ${2}",
        "       ${1}/ /  \\\\ \\\\    ${2}",
        "      ${1}/ / /\\\\ \\\\ \\\\   ${2}",
        "     ${1}/ / /  \\\\ \\\\ \\\\  ${2}",
        "    ${1}/ / /    \\\\ \\\\ \\\\ ${2}",
        "   ${1}/_/_/______\\\\_\\\\_\\\\ ${2}",
        "   ${1}|    FLEXFETCH    |${2}",
        "   ${1}|________________|${2}",
        "    ${1}/  /________\\\\  \\\\${2}",
        "   ${1}/  /          \\\\  \\\\${2}",
        "  ${1}/  /            \\\\  \\\\${2}",
        " /  /              \\\\  \\${2}",
        "/  /                \\\\  \\${2}",
        "${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}/${1}${2}",
        "",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const ARCH_LOGO: Logo = Logo {
    lines: &[
        "                      ${1}╱╲${2}",
        "                     ${1}/  \\${2}",
        "                    ${1}/ /\\ \\${2}",
        "                   ${1}/ /  \\ \\${2}",
        "                  ${1}/ / /\\ \\ \\${2}",
        "                 ${1}/ / /  \\ \\ \\${2}",
        "                ${1}/ / /    \\ \\ \\${2}",
        "               ${1}/ / /  ${1}    \\ \\ \\${2}",
        "              ${1}/ / /  ${1}      \\ \\ \\${2}",
        "             ${1}/ / /  ${1}________\\ \\ \\${2}",
        "            ${1}/_/_/  ${1}/________\\_\\_\\${2}",
        "            ${1}  ${1}   ${1}/          \\   ${2}",
        "           ${1}  ${1}   ${1}/            \\   ${2}",
        "          ${1}  ${1}   ${1}/    ${1}A R C H  \\   ${2}",
        "         ${1}  ${1}   ${1}/                \\   ${2}",
        "        ${1}  ${1}   ${1}/                  \\   ${2}",
        "       ${1}  ${1}   ${1}/____________________\\   ${2}",
        "       ${1}  ${1}   ${1}|${1}==================${1}|   ${2}",
        "       ${1}  ${1}   ${1}\\\\__________________//   ${2}",
        "       ${1}  ${1}   ${1} \\\\                //   ${2}",
        "       ${1}  ${1}   ${1}  \\\\______________//    ${2}",
        "       ${1}  ${1}   ${1}   ${1}\\\\            //     ${2}",
        "       ${1}  ${1}   ${1}    ${1}\\\\__________//      ${2}",
        "       ${1}  ${1}   ${1}     ${1}\\\\        //       ${2}",
        "       ${1}  ${1}   ${1}      ${1}\\\\      //        ${2}",
        "       ${1}  ${1}   ${1}       ${1}\\\\    //         ${2}",
        "       ${1}  ${1}   ${1}        ${1}\\\\  //          ${2}",
        "       ${1}  ${1}   ${1}         ${1}\\\\//           ${2}",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const MANJARO_LOGO: Logo = Logo {
    lines: &[
        "  ${1}███╗   ███╗${2}██╗  ${1}██╗${2}██╗   ██╗",
        "  ${1}████╗ ████║${2}██║  ${1}██║${2}██║   ██║",
        "  ${1}██╔████╔██║${2}██║  ${1}██║${2}██║   ██║",
        "  ${1}██║╚██╔╝██║${2}██║  ${1}██║${2}██║   ██║",
        "  ${1}██║ ╚═╝ ██║${2}██║  ${1}██║${2}██║   ██║",
        "  ${1}██║     ██║${2}╚█████╔╝${1}╚██████╔╝${2}",
        "  ${1}╚═╝     ╚═╝${2} ╚════╝  ${1} ╚═════╝${2}",
        "                                    ",
        "  ${1}███╗   ███╗██╗███╗   ██╗██████╗ ███████╗${2}",
        "  ${1}████╗ ████║${2}██║████╗  ${1}██║${2}██╔══██╗${1}██╔════╝${2}",
        "  ${1}██╔████╔██║${2}██║██╔██╗ ${1}██║${2}██║  ${1}██║${2}█████╗  ",
        "  ${1}██║╚██╔╝██║${2}██║██║${1}╚██╗${2}██║${1}██║  ██║${2}██╔══╝  ",
        "  ${1}██║ ╚═╝ ██║${2}██║██║ ${1}╚████║${2}██████╔╝${1}███████╗${2}",
        "  ${1}╚═╝     ╚═╝${2}╚═╝╚═╝  ${1} ╚═══╝${2}╚═════╝ ${1}╚══════╝${2}",
        "                                    ",
        "  ${1}███╗   ███╗██╗███╗   ██╗${2}███████╗${1}███████╗${2}",
        "  ${1}████╗ ████║${2}██║████╗  ${1}██║${2}██╔════╝${1}██╔════╝${2}",
        "  ${1}██╔████╔██║${2}██║██╔██╗ ${1}██║${2}█████╗  ${1}███████╗${2}",
        "  ${1}██║╚██╔╝██║${2}██║██║${1}╚██╗${2}██║${1}██╔══╝  ${2}╚════██║",
        "  ${1}██║ ╚═╝ ██║${2}██║██║ ${1}╚████║${2}███████╗${1}███████║${2}",
        "  ${1}╚═╝     ╚═╝${2}╚═╝╚═╝  ${1} ╚═══╝${2}╚══════╝${1}╚══════╝${2}",
        "",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const ENDEAVOUROS_LOGO: Logo = Logo {
    lines: &[
        "                      ${1}╱\\${2}",
        "                     ${1}/  \\${2}",
        "                    ${1}/ /\\ \\${2}",
        "                   ${1}/ /  \\ \\${2}",
        "                  ${1}/ /    \\ \\${2}",
        "                 ${1}/ / /\\ /\\ \\ \\${2}",
        "                ${1}/ / /  V  \\ \\ \\${2}",
        "               ${1}/ / / [___] \\ \\ \\${2}",
        "              ${1}/ / /  /   \\  \\ \\ \\${2}",
        "             ${1}/ / /  / ENDE \\  \\ \\ \\${2}",
        "            ${1}/ / /  / AVOUROS \\  \\ \\ \\${2}",
        "           ${1}/_/_/  /___________\\  \\_\\_\\${2}",
        "           ${1}      ${1}|             |${2}",
        "           ${1}      ${1}|  ENDEAVOUROS|${2}",
        "           ${1}      ${1}|_____________|${2}",
        "           ${1}      ${1}/             \\${2}",
        "           ${1}     ${1}/               \\${2}",
        "           ${1}    ${1}/                 \\${2}",
        "           ${1}   ${1}/___________________\\${2}",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const DEBIAN_LOGO: Logo = Logo {
    lines: &[
        "            ${1}       ${2}",
        "           ${1}(_)${2}      ",
        "          ${1}/   \\${2}     ",
        "         ${1}/     \\${2}    ",
        "        ${1}/ /\\ /\\ \\${2}   ",
        "       ${1}/ /  V  \\ \\${2}  ",
        "      ${1}/ / [___] \\ \\${2} ",
        "     ${1}/ /  /   \\  \\ \\${2}",
        "    ${1}/ /  / DEB \\  \\ \\${2}",
        "   ${1}/ /  /  IAN  \\  \\ \\${2}",
        "  ${1}/ /  /________\\  \\ \\${2}",
        " ${1}/_/              \\_\\${2}",
        " ${1}____________________${2}",
        " ${1}|${1}  D E B I A N   ${1}|${2}",
        " ${1}|${1}________________${1}|${2}",
        "  ${1}/                  \\${2}",
        " ${1}/                    \\${2}",
        "${1}/______________________\\${2}",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const UBUNTU_LOGO: Logo = Logo {
    lines: &[
        "                  ${1}     ${2}",
        "               ${1}(  _ _  )${2}   ",
        "              ${1}/ ( o o ) \\${2}  ",
        "             ${1}/ /  ===  \\ \\${2} ",
        "            ${1}/ / /\\   /\\ \\ \\${2}",
        "           ${1}/ / /  \\ /  \\ \\ \\${2}",
        "          ${1}/_/ /    V    \\ \\_\\${2}",
        "          ${1}  / /\\       /\\ \\${2} ",
        "         ${1} / /  \\_____/  \\ \\${2}",
        "        ${1}/_/             \\_\\${2}",
        "        ${1}|   U B U N T U   |${2}",
        "        ${1}|_________________|${2}",
        "         ${1}/                 \\${2}",
        "        ${1}/                   \\${2}",
        "       ${1}/_____________________\\${2}",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const UBUNTU_BUDGIE_LOGO: Logo = Logo {
    lines: &[
        "                  ${1}     ${2}",
        "               ${1}(  _ _  )${2}   ",
        "              ${1}/ ( o o ) \\${2}  ",
        "             ${1}/ /  ===  \\ \\${2} ",
        "            ${1}/ / /\\   /\\ \\ \\${2}",
        "           ${1}/ / /  \\ /  \\ \\ \\${2}",
        "          ${1}/_/ /    V    \\ \\_\\${2}",
        "          ${1}  / /\\       /\\ \\${2} ",
        "         ${1} / /  \\_____/  \\ \\${2}",
        "        ${1}/_/             \\_\\${2}",
        "        ${1}| B U D G I E     |${2}",
        "        ${1}|_________________|${2}",
        "         ${1}/                 \\${2}",
        "        ${1}/                   \\${2}",
        "       ${1}/_____________________\\${2}",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const UBUNTU_MATE_LOGO: Logo = Logo {
    lines: &[
        "                  ${1}     ${2}",
        "               ${1}(  _ _  )${2}   ",
        "              ${1}/ ( o o ) \\${2}  ",
        "             ${1}/ /  ===  \\ \\${2} ",
        "            ${1}/ / /\\   /\\ \\ \\${2}",
        "           ${1}/ / /  \\ /  \\ \\ \\${2}",
        "          ${1}/_/ /    V    \\ \\_\\${2}",
        "          ${1}  / /\\       /\\ \\${2} ",
        "         ${1} / /  \\_____/  \\ \\${2}",
        "        ${1}/_/             \\_\\${2}",
        "        ${1}| M A T E         |${2}",
        "        ${1}|_________________|${2}",
        "         ${1}/                 \\${2}",
        "        ${1}/                   \\${2}",
        "       ${1}/_____________________\\${2}",
        "",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const UBUNTU_KYLIN_LOGO: Logo = Logo {
    lines: &[
        "                  ${1}     ${2}",
        "               ${1}(  _ _  )${2}   ",
        "              ${1}/ ( o o ) \\${2}  ",
        "             ${1}/ /  ===  \\ \\${2} ",
        "            ${1}/ / /\\   /\\ \\ \\${2}",
        "           ${1}/ / /  \\ /  \\ \\ \\${2}",
        "          ${1}/_/ /    V    \\ \\_\\${2}",
        "          ${1}  / /\\       /\\ \\${2} ",
        "         ${1} / /  \\_____/  \\ \\${2}",
        "        ${1}/_/             \\_\\${2}",
        "        ${1}| K Y L I N       |${2}",
        "        ${1}|_________________|${2}",
        "         ${1}/                 \\${2}",
        "        ${1}/                   \\${2}",
        "       ${1}/_____________________\\${2}",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const FEDORA_LOGO: Logo = Logo {
    lines: &[
        "                    ${1}________${2}",
        "                   ${1}/        \\${2}",
        "                  ${1}/ /\\    /\\ \\${2}",
        "                 ${1}/ /  \\  /  \\ \\${2}",
        "                ${1}/ /    \\/    \\ \\${2}",
        "               ${1}/ /     ||     \\ \\${2}",
        "              ${1}/ /      ||      \\ \\${2}",
        "             ${1}/ /       ||       \\ \\${2}",
        "            ${1}/_/________||________\\_\\${2}",
        "            ${1}|${1}________________________${1}|${2}",
        "            ${1}|     F E D O R A        |${2}",
        "            ${1}|________________________|${2}",
        "             ${1}/____________________\\${2}",
        "            ${1}/                      \\${2}",
        "           ${1}/________________________\\${2}",
        "",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const NIXOS_LOGO: Logo = Logo {
    lines: &[
        "                   ${1}         ${2}",
        "                  ${1}/\\        ${2}",
        "                 ${1}/  \\       ${2}",
        "                ${1}/ /\\ \\      ${2}",
        "               ${1}/ /  \\ \\     ${2}",
        "              ${1}/ /    \\ \\    ${2}",
        "             ${1}/ / NIXO \\ \\   ${2}",
        "            ${1}/ /   S    \\ \\  ${2}",
        "           ${1}/ /          \\ \\ ${2}",
        "          ${1}/ /    /\\      \\ \\${2}",
        "         ${1}/ /    /  \\      \\ \\${2}",
        "        ${1}/_/    /    \\      \\_\\${2}",
        "        ${1}      /      \\      ${2}",
        "       ${1}     /        \\     ${2}",
        "      ${1}    /          \\    ${2}",
        "     ${1}   /____________\\   ${2}",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const GENTOO_LOGO: Logo = Logo {
    lines: &[
        "                   ${1}       ${2}",
        "                  ${1}  .--.  ${2}",
        "                 ${1} |o_o | ${2}",
        "                ${1}  |:_/ |${2}  ",
        "               ${1} //   \\ \\${2} ",
        "              ${1} (|     | )${2}",
        "             ${1}  /'\\_   _/\\`${2} ",
        "            ${1}  \\___)=(___/${2} ",
        "           ${1}                 ${2}",
        "          ${1}   G E N T O O    ${2}",
        "         ${1}  _______________  ${2}",
        "        ${1} /               \\ ${2}",
        "       ${1}/_________________\\${2}",
        "",
    ],
    colors: &[MAGENTA, RESET, WHITE],
};

const ALPINE_LOGO: Logo = Logo {
    lines: &[
        "                      ${1} ${2}",
        "                     ${1}/ \\${2}",
        "                    ${1}/   \\${2}",
        "                   ${1}/ /\\  \\${2}",
        "                  ${1}/ /  \\  \\${2}",
        "                 ${1}/ /    \\  \\${2}",
        "                ${1}/ /      \\  \\${2}",
        "               ${1}/ /________\\  \\${2}",
        "              ${1}/ /   A L P  \\  \\${2}",
        "             ${1}/ /     I N E  \\  \\${2}",
        "            ${1}/ /              \\  \\${2}",
        "           ${1}/_/________________\\  \\${2}",
        "           ${1}\\(___________________)${2}",
        "            ${1}/                   \\${2}",
        "           ${1}/_____________________\\${2}",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const VOID_LOGO: Logo = Logo {
    lines: &[
        "                   ${1}  ${2}",
        "                  ${1} /|\\ ${2}",
        "                 ${1}/ | \\${2}",
        "                ${1}/  |  \\${2}",
        "               ${1}/   |   \\${2}",
        "              ${1}/    |    \\${2}",
        "             ${1}/  V | I   \\${2}",
        "            ${1}/    |   D   \\${2}",
        "           ${1}/     |        \\${2}",
        "          ${1}/______|_________\\${2}",
        "         ${1}|                 |${2}",
        "         ${1}|   V  O  I  D   |${2}",
        "         ${1}|_________________|${2}",
        "          ${1}/                 \\${2}",
        "         ${1}/___________________\\${2}",
        "",
    ],
    colors: &[MAGENTA, RESET, WHITE],
};

const CENTOS_LOGO: Logo = Logo {
    lines: &[
        "                    ${1}________${2}",
        "                   ${1}/        \\${2}",
        "                  ${1}/ /\\    /\\ \\${2}",
        "                 ${1}/ /  \\  /  \\ \\${2}",
        "                ${1}/ /    \\/    \\ \\${2}",
        "               ${1}/ /     ||     \\ \\${2}",
        "              ${1}/ /      ||      \\ \\${2}",
        "             ${1}/ /       ||       \\ \\${2}",
        "            ${1}/_/________||________\\_\\${2}",
        "            ${1}|${1}________________________${1}|${2}",
        "            ${1}|      C E N T O S       |${2}",
        "            ${1}|________________________|${2}",
        "             ${1}/____________________\\${2}",
        "            ${1}/                      \\${2}",
        "           ${1}/________________________\\${2}",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const OPENSUSE_LOGO: Logo = Logo {
    lines: &[
        "                      ${1}    ${2}",
        "                     ${1} .--. ${2}",
        "                    ${1}/|o  o|\\${2}",
        "                   ${1}/ | __ | \\${2}",
        "                  ${1}/  |/  \\|  \\${2}",
        "                 ${1}/   |    |   \\${2}",
        "                ${1}/    |    |    \\${2}",
        "               ${1}/  S | U  | S E \\${2}",
        "              ${1}/     | S  |      \\${2}",
        "             ${1}/______|____|______\\${2}",
        "            ${1}|                   |${2}",
        "            ${1}|  O P E N S U S E  |${2}",
        "            ${1}|___________________|${2}",
        "             ${1}/                   \\${2}",
        "            ${1}/_____________________\\${2}",
        "",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const KALI_LOGO: Logo = Logo {
    lines: &[
        "                   ${1}         ${2}",
        "                  ${1} /\\   /\\  ${2}",
        "                 ${1}/  \\_/  \\ ${2}",
        "                ${1}/    K    \\${2}",
        "               ${1}/    A     \\${2}",
        "              ${1}/     L     \\${2}",
        "             ${1}/      I     \\${2}",
        "            ${1}/_______________\\${2}",
        "            ${1}|               |${2}",
        "            ${1}|  ${3}K A L I${1}   |${2}",
        "            ${1}|_______________|${2}",
        "            ${1}/               \\${2}",
        "           ${1}/                 \\${2}",
        "          ${1}/___________________\\${2}",
        "",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const LINUX_MINT_LOGO: Logo = Logo {
    lines: &[
        "            ${1}        ${2}",
        "           ${1}  .--.  ${2}",
        "          ${1} /    \\ ${2}",
        "         ${1}| MINT |${2} ",
        "         ${1} \\    / ${2}",
        "          ${1} '--'  ${2}",
        "        ${1} L  I  N  U  X${2}",
        "        ${1} M  I  N  T  ${2}",
        "        ${1}____________${2}",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const POP_OS_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  ______${2}",
        "          ${1} /      \\${2}",
        "         ${1}| POP_OS |${2}",
        "         ${1}|   POP  |${2}",
        "          ${1} \\____/ ${2}",
        "           ${1}  ||  ${2}",
        "           ${1}  ||  ${2}",
        "         ${1} P O P ! _ O S${2}",
        "        ${1}________________${2}",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const ELEMENTARY_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  .--. ${2}",
        "          ${1} /    \\${2}",
        "         ${1}| e e  |${2}",
        "         ${1}|  __  |${2}",
        "          ${1}\\ __/ ${2}",
        "           ${1}'--' ${2}",
        "        ${1} E L E M E N T A R Y${2}",
        "        ${1}____________________${2}",
    ],
    colors: &[WHITE, RESET, CYAN],
};

const ZORIN_LOGO: Logo = Logo {
    lines: &[
        "            ${1}  ____  ${2}",
        "           ${1} / Z  \\ ${2}",
        "          ${1}| O R |${2} ",
        "          ${1}| I N |${2} ",
        "           ${1} \\____/ ${2}",
        "            ${1} |  | ${2}",
        "        ${1} Z O R I N   O S${2}",
        "        ${1}________________${2}",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const CACHYOS_LOGO: Logo = Logo {
    lines: &[
        "            ${1}   /\\   ${2}",
        "          ${1}  /  \\  ${2}",
        "         ${1} / /\\ \\ ${2}",
        "        ${1}/ /  \\ \\${2}",
        "        ${1}\\ \\ CACHY / /${2}",
        "         ${1}\\ \\  / /${2}",
        "          ${1}\\ \\/ / ${2}",
        "           ${1} \\/  ${2}",
        "        ${1} C A C H Y O S${2}",
        "        ${1}________________${2}",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const ARTIX_LOGO: Logo = Logo {
    lines: &[
        "            ${1}   /\\   ${2}",
        "          ${1}  /  \\  ${2}",
        "         ${1} / /\\ \\ ${2}",
        "        ${1}/ /  \\ \\${2}",
        "        ${1}\\ \\ ARTIX / /${2}",
        "         ${1}\\ \\  / /${2}",
        "          ${1}\\ \\/ / ${2}",
        "           ${1} \\/  ${2}",
        "        ${1} A R T I X${2}",
        "        ${1}________________${2}",
    ],
    colors: &[RED, RESET, WHITE],
};

const GARUDA_LOGO: Logo = Logo {
    lines: &[
        "            ${1}    |\\  ${2}",
        "           ${1} /|  | \\ ${2}",
        "          ${1}/ |G |  \\${2}",
        "         ${1}  | A  R |${2}",
        "          ${1}\\| U  D|/${2}",
        "           ${1} | A | ${2}",
        "            ${1}/|   |\\${2}",
        "           ${1}/ |   | \\${2}",
        "        ${1} G A R U D A${2}",
        "        ${1}________________${2}",
    ],
    colors: &[MAGENTA, RESET, WHITE],
};

const MX_LINUX_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  _____  ${2}",
        "          ${1} | MX  | ${2}",
        "          ${1} |__ __| ${2}",
        "          ${1}  |  |  ${2}",
        "          ${1}  |__|  ${2}",
        "        ${1} M X   L I N U X${2}",
        "        ${1}________________${2}",
    ],
    colors: &[WHITE, RESET, CYAN],
};

const ANTIX_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  .-. ${2}",
        "          ${1} ( a )${2}",
        "          ${1}  '-' ${2}",
        "          ${1} /   \\${2}",
        "         ${1}/  X  \\${2}",
        "        ${1}/ /   \\ \\${2}",
        "        ${1}a n t i X${2}",
        "        ${1}____________${2}",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const PCLINUXOS_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  _____  ${2}",
        "          ${1} | P C | ${2}",
        "          ${1} |_____| ${2}",
        "          ${1}  |   | ${2}",
        "          ${1}  |___| ${2}",
        "        ${1} P C L I N U X O S${2}",
        "        ${1}________________${2}",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const SLACKWARE_LOGO: Logo = Logo {
    lines: &[
        "            ${1} /-------\\ ${2}",
        "           ${1} |  SLK  |${2} ",
        "          ${1} |-------|${2}  ",
        "          ${1} |  /\\ |${2}   ",
        "          ${1} | / \\|${2}    ",
        "          ${1} |/   \\${2}    ",
        "        ${1} S L A C K W A R E${2}",
        "        ${1}________________${2}",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const PUPPY_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  /\\_/\\ ${2}",
        "          ${1} ( o.o )${2}",
        "          ${1}  > ^ < ${2}",
        "          ${1} /|   |\\${2}",
        "         ${1}/ | P | \\${2}",
        "           ${1}|_____|${2}",
        "        ${1} P U P P Y   L I N U X${2}",
        "        ${1}______________________${2}",
    ],
    colors: &[WHITE, RESET, CYAN],
};

const TINYCORE_LOGO: Logo = Logo {
    lines: &[
        "            ${1}  ____  ${2}",
        "           ${1} | TC | ${2}",
        "           ${1} |____| ${2}",
        "            ${1}  ||  ${2}",
        "            ${1}  ||  ${2}",
        "        ${1} T I N Y   C O R E${2}",
        "        ${1}____________________${2}",
    ],
    colors: &[WHITE, RESET, GREEN],
};

const ARCH_ARM_LOGO: Logo = Logo {
    lines: &[
        "            ${1}   /\\   ${2}",
        "          ${1}  /  \\  ${2}",
        "         ${1} / /\\ \\ ${2}",
        "        ${1}/ /  \\ \\${2}",
        "        ${1}\\ \\ ARM / /${2}",
        "         ${1}\\ \\  / /${2}",
        "          ${1}\\ \\/ / ${2}",
        "           ${1} \\/  ${2}",
        "        ${1} A R C H   A R M${2}",
        "        ${1}________________${2}",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const BIGLINUX_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  ____  ${2}",
        "          ${1} | B  | ${2}",
        "          ${1} | I  | ${2}",
        "          ${1} | G  | ${2}",
        "          ${1} |____| ${2}",
        "        ${1} B I G   L I N U X${2}",
        "        ${1}__________________${2}",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const LINUX_LITE_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  ____  ${2}",
        "          ${1} |    | ${2}",
        "          ${1} | L  | ${2}",
        "          ${1} |____| ${2}",
        "          ${1}  |  | ${2}",
        "          ${1}  |__| ${2}",
        "        ${1} L I N U X   L I T E${2}",
        "        ${1}____________________${2}",
    ],
    colors: &[WHITE, RESET, YELLOW],
};

const PEPPERMINT_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  .-. ${2}",
        "          ${1} ( P )${2}",
        "          ${1}  '-' ${2}",
        "          ${1} /   \\${2}",
        "         ${1}/     \\${2}",
        "        ${1}/  P P \\${2}",
        "        ${1}p e p p e r m i n t${2}",
        "        ${1}____________${2}",
    ],
    colors: &[RED, RESET, WHITE],
};

const BODHI_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  .--. ${2}",
        "          ${1} ( B  )${2}",
        "          ${1}  '--' ${2}",
        "          ${1} /    \\${2}",
        "         ${1}/  OD  \\${2}",
        "        ${1}/   HI   \\${2}",
        "        ${1}b o d h i${2}",
        "        ${1}____________${2}",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const TRISQUEL_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  .--. ${2}",
        "          ${1} / T  \\${2}",
        "         ${1}| R   |${2}",
        "         ${1}| I   |${2}",
        "          ${1} \\ S / ${2}",
        "           ${1}  '-' ${2}",
        "        ${1} T R I S Q U E L${2}",
        "        ${1}________________${2}",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const PUREOS_LOGO: Logo = Logo {
    lines: &[
        "           ${1}  .--. ${2}",
        "          ${1} / P  \\${2}",
        "         ${1}| U   |${2}",
        "         ${1}| R   |${2}",
        "          ${1} \\ E / ${2}",
        "           ${1}  '-' ${2}",
        "        ${1} P U R E   O S${2}",
        "        ${1}________________${2}",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const MACOS_LOGO: Logo = Logo {
    lines: &[
        "                  ${1}     ${2}",
        "                 ${1}  _   ${2}",
        "                ${1} ,-' '.,${2}",
        "               ${1} /      \\${2}",
        "              ${1} /  /--\\\\  \\${2}",
        "             ${1} |  |    |  |${2}",
        "             ${1} |  | __ |  |${2}",
        "             ${1} |  |/  \\|  |${2}",
        "              ${1} \\   \\/   / ${2}",
        "               ${1} '.____.'  ${2}",
        "                ${1} |    |   ${2}",
        "                ${1} |    |   ${2}",
        "               ${1}/|    |\\  ${2}",
        "              ${1}/ |    | \\ ${2}",
        "             ${1}/__|____|__\\${2}",
        "             ${1}   m a c O S${2}",
        "",
    ],
    colors: &[WHITE, RESET, CYAN],
};
