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

        // Check TERM for sixel support
        if let Ok(term) = env::var("TERM") {
            if term.contains("mlterm")
                || term.contains("foot")
                || term.contains("contour")
                || term.contains("xterm")
            {
                return ImageProtocol::Sixel;
            }
        }

        // Default to block fallback for unknown terminals
        ImageProtocol::Block
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
                    ImageProtocol::None | ImageProtocol::Block => {
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
                    _ => {
                        // Use image protocol
                        Self::render_with_protocol(&encoded, self.width, self.height, protocol)
                    }
                }
            }
            LogoMode::Image => {
                // Force image protocol
                Self::render_with_protocol(&encoded, self.width, self.height, protocol)
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
            ImageProtocol::Sixel => String::new(), // Not implemented
            ImageProtocol::Block => String::new(),
            ImageProtocol::None => String::new(),
        }
    }

    fn render_kitty_protocol(encoded: &str, width: Option<u32>, height: Option<u32>) -> String {
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
