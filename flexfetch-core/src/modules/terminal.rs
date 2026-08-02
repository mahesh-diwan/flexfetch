use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str {
        "terminal"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        // Fast env var lookup (no subprocess)
        let name = std::env::var("TERM_PROGRAM")
            .or_else(|_| std::env::var("TERM"))
            .unwrap_or_else(|_| "unknown".to_string());
        map.insert("name".into(), name.clone());

        // Phase 4.7: font detection — kitty via its query command (fast env gate),
        // otherwise a TERMINAL_FONT env hint. Live OSC-50 queries are deliberately
        // avoided: they need raw-mode reads that can hang a one-shot fetch.
        let font = if std::env::var("TERM_PROGRAM").as_deref() == Ok("kitty") {
            std::process::Command::new("kitty")
                .args(["@", "get-font"])
                .output()
                .ok()
                .and_then(|o| {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                })
        } else {
            None
        }
        .or_else(|| {
            std::env::var("TERMINAL_FONT")
                .ok()
                .filter(|s| !s.is_empty())
        });

        if let Some(f) = font {
            map.insert("font".into(), f);
        }

        // Phase 4.7: image protocol negotiation (kitty / iTerm2 / sixel).
        let mut protocols: Vec<&str> = Vec::new();
        if std::env::var("KITTY_WINDOW_ID").is_ok() || name.eq_ignore_ascii_case("kitty") {
            protocols.push("kitty");
        }
        if name.contains("iTerm") {
            protocols.push("iterm2");
        }
        let term = std::env::var("TERM").unwrap_or_default().to_lowercase();
        if term.contains("sixel") || term.contains("foot") || term.contains("mlterm") {
            protocols.push("sixel");
        }
        if !protocols.is_empty() {
            map.insert("image_protocols".into(), protocols.join(","));
        }

        // Truecolor support (COLORTERM=truecolor / 24bit).
        if let Ok(ct) = std::env::var("COLORTERM") {
            if ct.eq_ignore_ascii_case("truecolor") || ct.contains("24bit") {
                map.insert("truecolor".into(), "yes".into());
            }
        }

        // OSC-8 hyperlink support: kitty, wezterm, foot, alacritty, iTerm2,
        // konsole, ghostty, vscode all support it. Env-gated, non-blocking.
        let hyperlink_capable = [
            "kitty",
            "wezterm",
            "foot",
            "alacritty",
            "iterm",
            "konsole",
            "ghostty",
            "vscode",
            "tmux",
        ];
        let tp = std::env::var("TERM_PROGRAM")
            .unwrap_or_default()
            .to_lowercase();
        let term_lc = std::env::var("TERM").unwrap_or_default().to_lowercase();
        if hyperlink_capable
            .iter()
            .any(|h| tp.contains(h) || term_lc.contains(h))
        {
            map.insert("hyperlinks".into(), "yes".into());
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(test)]
mod tests {
    // No unit tests here: the module reads process env vars, which are global
    // and unsafe to mutate in parallel test runs. Behavior is validated by the
    // integration test in flexfetch-core/tests/integration_tests.rs.
}
