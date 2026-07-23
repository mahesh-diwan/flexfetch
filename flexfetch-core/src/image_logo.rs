use base64::Engine as _;
use std::env;
use std::fs;
use std::path::Path;

use crate::logo::{detect, render};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageProtocol {
    Kitty,
    Iterm2,
    Sixel,
    Block,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoMode {
    Image,
    Ascii,
    Auto,
}

impl ImageProtocol {
    pub fn detect() -> Self {
        // Allow forcing protocol via environment variable
        if let Ok(forced) = env::var("FLEXFETCH_IMAGE_PROTOCOL") {
            return match forced.to_lowercase().as_str() {
                "kitty" => ImageProtocol::Kitty,
                "iterm2" => ImageProtocol::Iterm2,
                "sixel" => ImageProtocol::Sixel,
                "block" => ImageProtocol::Block,
                "none" => ImageProtocol::None,
                _ => ImageProtocol::Block,
            };
        }

        // Check environment variables first
        if env::var("KITTY_WINDOW_ID").is_ok() || env::var("GHOSTTY_RESOURCES_DIR").is_ok() {
            return ImageProtocol::Kitty;
        }

        if env::var("ITERM_SESSION_ID").is_ok()
            || env::var("WEZTERM_PANE").is_ok()
            || env::var("TERM_PROGRAM")
                .map(|v| v == "vscode")
                .unwrap_or(false)
        {
            return ImageProtocol::Iterm2;
        }

        // Check for konsole
        if env::var("KONSOLE_VERSION").is_ok() {
            return ImageProtocol::Kitty;
        }

        // Check TERM for sixel support (only terminals with confirmed sixel)
        if let Ok(term) = env::var("TERM") {
            if term.contains("mlterm") || term.contains("foot") || term.contains("contour") {
                return ImageProtocol::Sixel;
            }
        }

        // No image protocol detected — use ASCII fallback
        ImageProtocol::None
    }
}

#[derive(Debug, Clone)]
pub struct ImageLogo {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ImageLogo {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            width: None,
            height: None,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn render(&self, protocol: ImageProtocol, mode: LogoMode) -> String {
        // Resolve the actual path
        let resolved_path = Self::resolve_path(&self.path);
        let image_data = match fs::read(&resolved_path) {
            Ok(data) => data,
            Err(_) => return String::new(),
        };

        let encoded = base64::engine::general_purpose::STANDARD.encode(&image_data);

        match mode {
            LogoMode::Ascii => {
                // Use ASCII art fallback
                let logo = detect(
                    &self
                        .path
                        .split('/')
                        .last()
                        .unwrap_or("")
                        .replace(".png", ""),
                );
                let lines: Vec<String> = logo.lines.iter().map(|s| s.to_string()).collect();
                let rendered = render(logo, lines.len());
                rendered.join("\n")
            }
            LogoMode::Auto => {
                match protocol {
                    ImageProtocol::None => {
                        // Fallback to ASCII
                        let logo = detect(
                            &self
                                .path
                                .split('/')
                                .last()
                                .unwrap_or("")
                                .replace(".png", ""),
                        );
                        let lines: Vec<String> = logo.lines.iter().map(|s| s.to_string()).collect();
                        let rendered = render(logo, lines.len());
                        rendered.join("\n")
                    }
                    ImageProtocol::Sixel => {
                        Self::render_sixel_protocol(&resolved_path, self.width, self.height)
                    }
                    ImageProtocol::Block => {
                        Self::render_block_fallback(&resolved_path, self.width, self.height)
                    }
                    _ => {
                        // Use image protocol
                        Self::render_with_protocol(&encoded, self.width, self.height, protocol)
                    }
                }
            }
            LogoMode::Image => {
                match protocol {
                    ImageProtocol::Sixel => {
                        Self::render_sixel_protocol(&resolved_path, self.width, self.height)
                    }
                    ImageProtocol::Block => {
                        Self::render_block_fallback(&resolved_path, self.width, self.height)
                    }
                    _ => {
                        // Force image protocol
                        Self::render_with_protocol(&encoded, self.width, self.height, protocol)
                    }
                }
            }
        }
    }

    fn render_with_protocol(
        encoded: &str,
        width: Option<u32>,
        height: Option<u32>,
        protocol: ImageProtocol,
    ) -> String {
        match protocol {
            ImageProtocol::Kitty => Self::render_kitty_protocol(encoded, width, height),
            ImageProtocol::Iterm2 => Self::render_iterm2_protocol(encoded, width, height),
            _ => String::new(),
        }
    }

    fn render_kitty_protocol(encoded: &str, width: Option<u32>, height: Option<u32>) -> String {
        // Kitty graphics protocol: https://sw.kovidgoyal.net/kitty/graphics-protocol/
        let mut payload = String::from("f=100,t=f,a=T,i=1");
        if let Some(w) = width {
            payload.push_str(&format!(",s={}", w));
        }
        if let Some(h) = height {
            payload.push_str(&format!(",v={}", h));
        }

        let mut cmd = String::new();
        if encoded.len() <= 4096 {
            // Small image: single transmission
            cmd.push_str(&format!("\x1b_G{};{}\x1b\\", payload, encoded));
        } else {
            // Large image: chunked transmission
            let mut first = true;
            for chunk in encoded.as_bytes().chunks(4096) {
                let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
                if first {
                    cmd.push_str(&format!("\x1b_G{},m=1;{}\x1b\\", payload, chunk_str));
                    first = false;
                } else {
                    cmd.push_str(&format!("\x1b_Gm=1;{}\x1b\\", chunk_str));
                }
            }
            // Signal end of image data
            cmd.push_str("\x1b_Gm=0;\x1b\\");
        }

        cmd
    }

    fn render_sixel_protocol(path: &str, width: Option<u32>, height: Option<u32>) -> String {
        // Sixel: ESC P q <defs> <row data> ESC \
        // Each char = 6 vertical pixels (bit 0 = top), value = bits + 63
        let img = match image::open(path) {
            Ok(img) => img,
            Err(_) => return String::new(),
        };

        let img = match (width, height) {
            (Some(w), Some(h)) => img.resize(w, h, image::imageops::FilterType::Lanczos3),
            (Some(w), None) => {
                let ratio = w as f64 / img.width() as f64;
                let h = (img.height() as f64 * ratio) as u32;
                img.resize(w, h, image::imageops::FilterType::Lanczos3)
            }
            (None, Some(h)) => {
                let ratio = h as f64 / img.height() as f64;
                let w = (img.width() as f64 * ratio) as u32;
                img.resize(w, h, image::imageops::FilterType::Lanczos3)
            }
            (None, None) => img,
        };

        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();

        // 216-color cube palette (6×6×6)
        let quantize = |r: u8, g: u8, b: u8| -> u8 {
            ((r / 51) as u8) * 36 + ((g / 51) as u8) * 6 + (b / 51) as u8
        };

        let padded_h = ((h + 5) / 6) * 6;

        let mut output = String::from("\x1bPq");

        // Define 216-color palette
        for i in 0u16..216 {
            let r = ((i / 36) % 6) as u8 * 51;
            let g = ((i / 6) % 6) as u8 * 51;
            let b = (i % 6) as u8 * 51;
            output.push_str(&format!(
                "2;{};{};{};{}",
                i,
                r * 100 / 255,
                g * 100 / 255,
                b * 100 / 255
            ));
        }

        // Encode sixel rows
        for sixel_row in (0..padded_h).step_by(6) {
            for color_idx in 0u16..216 {
                let mut sixels = String::new();
                let mut has_any = false;

                for x in 0..w {
                    let mut sixel_bits: u8 = 0;
                    for bit in 0..6u32 {
                        let py = sixel_row + bit;
                        if py < h {
                            let pixel = rgb.get_pixel(x, py);
                            if quantize(pixel[0], pixel[1], pixel[2]) == color_idx as u8 {
                                sixel_bits |= 1 << bit;
                            }
                        }
                    }
                    sixels.push((sixel_bits + 63) as u8 as char);
                    if sixel_bits != 0 {
                        has_any = true;
                    }
                }

                if has_any {
                    output.push_str(&format!("#{}", color_idx));
                    output.push_str(&sixels);
                }
            }

            // New sixel line
            if sixel_row + 6 < padded_h {
                output.push('$'); // Carriage return within sixel
                output.push('-'); // Line feed
            }
        }

        output.push_str("\x1b\\");
        output
    }

    fn render_block_fallback(path: &str, width: Option<u32>, height: Option<u32>) -> String {
        // Unicode block art using ▀▄█ with ANSI truecolor
        let img = match image::open(path) {
            Ok(img) => img,
            Err(_) => return String::new(),
        };

        // Resize: default 40 cols wide, half height in rows (2 pixels per row)
        let target_w = width.unwrap_or(40);
        let target_h = height.unwrap_or(20);
        let img = img.resize(
            target_w,
            target_h * 2,
            image::imageops::FilterType::Lanczos3,
        );
        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();

        let mut lines: Vec<String> = Vec::new();

        // Process two pixel rows at a time
        for y in (0..h).step_by(2) {
            let mut line = String::new();
            for x in 0..w {
                let top = rgb.get_pixel(x, y);
                let bottom = if y + 1 < h {
                    rgb.get_pixel(x, y + 1)
                } else {
                    rgb.get_pixel(x, y)
                };

                let tr = top[0];
                let tg = top[1];
                let tb = top[2];
                let br = bottom[0];
                let bg = bottom[1];
                let bb = bottom[2];

                if tr == br && tg == bg && tb == bb {
                    // Same color: use space with background
                    line.push_str(&format!("\x1b[48;2;{};{};{}m ", tr, tg, tb));
                } else {
                    // Different colors: use ▀ (upper half block) with fg=top, bg=bottom
                    line.push_str(&format!(
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
                        tr, tg, tb, br, bg, bb
                    ));
                }
            }
            line.push_str("\x1b[0m");
            lines.push(line);
        }

        lines.join("\n")
    }

    fn render_iterm2_protocol(encoded: &str, width: Option<u32>, height: Option<u32>) -> String {
        let mut cmd = String::new();
        cmd.push_str("\x1b]1337;File=inline=1");
        if let Some(w) = width {
            cmd.push_str(&format!(";width={}", w));
        }
        if let Some(h) = height {
            cmd.push_str(&format!(";height={}", h));
        }
        cmd.push(':');
        cmd.push_str(encoded);
        cmd.push('\x07'); // BEL

        cmd
    }

    fn resolve_path(path: &str) -> String {
        // First try as-is (absolute path)
        if Path::new(path).exists() {
            return path.to_string();
        }

        // Try relative to flexfetch-core
        let core_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let candidate = Path::new(&core_dir).join(path);
        if candidate.exists() {
            return candidate.to_string_lossy().to_string();
        }

        // Try relative to workspace root
        if let Ok(workspace) = env::var("FLEXFETCH_WORKSPACE_DIR") {
            let candidate = Path::new(&workspace).join(path);
            if candidate.exists() {
                return candidate.to_string_lossy().to_string();
            }
        }

        path.to_string()
    }
}

pub fn get_distro_logo_path(os_id: &str) -> Option<String> {
    let base = "assets/logos/distros";
    let name = match os_id {
        "arch" | "cachyos" | "arcolinux" | "artix" => "arch",
        "manjaro" => "manjaro",
        "endeavouros" => "endeavouros",
        "debian" | "raspbian" => "debian",
        "ubuntu" | "linuxmint" | "pop" | "elementary" | "zorin" => "ubuntu",
        "fedora" => "fedora",
        "nixos" => "nixos",
        "gentoo" => "gentoo",
        "alpine" => "alpine",
        "void" => "void",
        "centos" => "centos",
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => "opensuse",
        "kali" => "kali",
        _ if cfg!(target_os = "macos") => "macos",
        _ => "generic",
    };
    Some(format!("{}/{}.png", base, name))
}

pub fn get_module_logo_path(module: &str) -> Option<String> {
    let base = "assets/logos/modules";
    let name = match module {
        "title" => "title",
        "os" => "os",
        "host" => "host",
        "kernel" => "kernel",
        "uptime" => "uptime",
        "locale" => "locale",
        "shell" => "shell",
        "terminal" => "terminal",
        "de" => "de",
        "wm" => "wm",
        "packages" => "packages",
        "cpu" => "cpu",
        "memory" => "memory",
        "disk" => "disk",
        "gpu" => "gpu",
        "network" => "network",
        "battery" => "battery",
        "processes" => "processes",
        "resolution" => "resolution",
        "colors" => "colors",
        "custom" => "custom",
        _ => return None,
    };
    Some(format!("{}/{}.png", base, name))
}
