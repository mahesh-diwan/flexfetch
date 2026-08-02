//! `flexfetch-tmux` — auto-display the fetch in new idle tmux panes (Phase 5.3).
//!
//! Installed next to the main `flexfetch` binary. Reads `$TMUX_PANE`, checks
//! whether the pane's current command is an interactive shell (i.e. the pane
//! is idle — no long-running command), and if so prints a compact fetch.
//!
//! Hook it up from `~/.tmux.conf`:
//!   run-shell ~/.local/bin/flexfetch-tmux
//! (see `flexfetch --tmux-config` for the full snippet.)
//!
//! Pure std — no deps, so it builds in every feature configuration.

use std::process::Command;

fn main() {
    let Some(pane_id) = std::env::var("TMUX_PANE").ok().filter(|s| !s.is_empty()) else {
        // Not in tmux — nothing to do (this binary is only meaningful inside one).
        std::process::exit(0);
    };

    // Only show the fetch if this pane is idle: its current command is a shell.
    let output = match Command::new("tmux")
        .args(["list-panes", "-F", "#{pane_id} #{pane_current_command}"])
        .output()
    {
        Ok(o) => o,
        Err(_) => std::process::exit(0),
    };
    let idle = String::from_utf8_lossy(&output.stdout).lines().any(|line| {
        let mut parts = line.split_whitespace();
        if parts.next() != Some(pane_id.as_str()) {
            return false;
        }
        let cmd = parts.next().unwrap_or("");
        matches!(cmd, "bash" | "zsh" | "fish" | "sh" | "nu")
    });
    if !idle {
        std::process::exit(0);
    }

    // Compact fetch: minimal modules, no logo (keeps the pane readable).
    let fetch = match Command::new("flexfetch").args(["--minimal"]).output() {
        Ok(o) => o,
        Err(_) => std::process::exit(0),
    };
    if fetch.status.success() {
        print!("{}", String::from_utf8_lossy(&fetch.stdout));
    }
}
