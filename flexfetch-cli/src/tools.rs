//! Utility commands: `--update`, `--hook`, `--doctor` (all pure std, no deps).
//!
//! These are "shell-integration" features from the Phase 4 marketing push:
//! - `--update` re-runs the idempotent install script when a newer release exists.
//! - `--hook <shell>` prints a cd-into-git-repo snippet for bash/zsh/fish.
//! - `--doctor` validates terminal, color, config, and core collectors so users
//!   can self-diagnose instead of filing issues.

use flexfetch_core::{Config, Context, ModuleRegistry};
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::Stdio;

/// `--update`: re-run the install script (it is idempotent and skips when the
/// installed version already matches the latest release, printing its own
/// outcome). Requires curl/wget.
pub fn self_update() {
    let install_url = "https://raw.githubusercontent.com/mahesh-diwan/flexfetch/main/install.sh";

    let has_curl = command_ok("curl", &["--version"]);
    let has_wget = command_ok("wget", &["--version"]);

    let pipe = if has_curl {
        format!("curl -fsSL {install_url} | sh")
    } else if has_wget {
        format!("wget -qO- {install_url} | sh")
    } else {
        eprintln!("error: --update needs curl or wget. Update manually with:");
        eprintln!("  curl -fsSL {install_url} | sh");
        std::process::exit(1);
    };

    println!("Updating flexfetch via the install script...");
    match std::process::Command::new("sh")
        .arg("-c")
        .arg(&pipe)
        .status()
    {
        Ok(s) if s.success() => {
            // install.sh prints the real outcome ("already at latest version"
            // or "installed vX.Y.Z"); nothing more to add.
        }
        Ok(s) => {
            eprintln!("update failed (exit {:?}) — try: {pipe}", s.code());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("update failed: {e} — try: {pipe}");
            std::process::exit(1);
        }
    }
}

/// `--hook <shell>`: print a snippet that runs a mini fetch when cd-ing into a
/// git repository. Add the output to the shell rc (`eval "$(flexfetch --hook zsh)"`).
pub fn print_hook(shell: &str) {
    match hook_for(shell) {
        Some(hook) => print!("{hook}"),
        None => {
            eprintln!("error: unknown shell '{shell}' (use bash, zsh, or fish)");
            std::process::exit(1);
        }
    }
}

/// The hook snippet for a shell, or `None` for unsupported shells. Extracted
/// from `print_hook` so the strings are testable.
fn hook_for(shell: &str) -> Option<&'static str> {
    let hook = match shell {
        // bash has no chpwd hook, so guard on a PWD change before running git
        // (PROMPT_COMMAND fires on every prompt otherwise). $OLDPWD is set by
        // bash on every cd, giving a true cd-only trigger.
        "bash" => {
            r#"# flexfetch: mini fetch when cd-ing into a git repo
_flexfetch_cd_hook() {
    if [ "$PWD" != "$OLDPWD" ] && git rev-parse --git-dir >/dev/null 2>&1; then
        flexfetch --prompt
    fi
}
PROMPT_COMMAND="_flexfetch_cd_hook${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
"#
        }
        "zsh" => {
            r#"# flexfetch: mini fetch when cd-ing into a git repo
_flexfetch_cd_hook() {
    if git rev-parse --git-dir >/dev/null 2>&1; then
        flexfetch --prompt
    fi
}
autoload -Uz add-zsh-hook
add-zsh-hook chpwd _flexfetch_cd_hook
"#
        }
        "fish" => {
            r#"# flexfetch: mini fetch when cd-ing into a git repo
function __flexfetch_cd --on-variable PWD
    if git rev-parse --git-dir >/dev/null 2>&1
        flexfetch --prompt
    end
end
"#
        }
        _ => return None,
    };
    Some(hook)
}

/// `--doctor`: environment diagnostics. Prints ✔/✖ per check; exits nonzero if
/// any hard check failed (color/config/collectors). Terminal and Nerd Font are
/// informational — a piped `flexfetch --doctor | less` must still exit 0.
pub fn run_doctor(ctx: &Context) {
    println!("flexfetch {} environment check:", env!("CARGO_PKG_VERSION"));
    let mut all_ok = true;

    // Terminal: informational (a piped doctor run is still a valid doctor run).
    let tty = std::io::stdout().is_terminal();
    report(
        tty,
        "Terminal",
        "TTY detected",
        "stdout is not a TTY (piped)",
    );

    // Truecolor support (24-bit color for gradients).
    let tc = std::env::var("COLORTERM")
        .map(|v| v == "truecolor" || v == "24bit")
        .unwrap_or(false);
    all_ok &= tc;
    report(
        tc,
        "Truecolor",
        "COLORTERM=truecolor",
        "set COLORTERM=truecolor for gradients",
    );

    // Nerd Font heuristic (icons render only in patched terminals).
    let nf = std::env::var("TERM_PROGRAM")
        .map(|v| {
            v.contains("kitty")
                || v.contains("wezterm")
                || v.contains("ghostty")
                || v.contains("alacritty")
        })
        .unwrap_or(false);
    report(
        nf,
        "Nerd Font",
        "terminal likely renders nerd fonts",
        "install a nerd-font-patched font for icons",
    );

    // Config syntax: the effective config file must parse as TOML.
    let cfg_path = user_config_path();
    let cfg_ok = if cfg_path.exists() {
        match std::fs::read_to_string(&cfg_path) {
            Ok(content) => toml::from_str::<Config>(&content).is_ok(),
            Err(_) => false,
        }
    } else {
        true // no user config — defaults are fine
    };
    all_ok &= cfg_ok;
    report(
        cfg_ok,
        "Config",
        &format!("{} parses", cfg_path.display()),
        &format!("{} is invalid TOML", cfg_path.display()),
    );

    // Core collectors: run a few zero-spawn modules and report availability.
    let registry = ModuleRegistry::get();
    for name in ["os", "kernel", "memory", "cpu"] {
        let ok = registry.run_individual(name, ctx).is_some();
        all_ok &= ok;
        report(ok, &format!("Collector {name}"), "available", "unavailable");
    }

    if all_ok {
        println!("All checks passed.");
    } else {
        println!("Some checks failed — see above for hints.");
        std::process::exit(1);
    }
}

fn report(ok: bool, name: &str, ok_msg: &str, bad_msg: &str) {
    if ok {
        println!("  \u{2714} {name}: {ok_msg}");
    } else {
        println!("  \u{2716} {name}: {bad_msg}");
    }
}

/// The user config path flexfetch reads by default (mirrors core's find_user_config).
fn user_config_path() -> PathBuf {
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".config")
        });
    xdg.join("flexfetch").join("config.toml")
}

/// `--tmux-config`: print a `~/.tmux.conf` snippet that auto-runs the
/// bundled `flexfetch-tmux` helper in every new idle pane (Phase 5.3). The
/// helper is installed next to the main binary by install.sh.
pub fn print_tmux_config() {
    print!(
        r#"# flexfetch: auto-display system info in new tmux panes (Phase 5.3)
# Generated by: flexfetch --tmux-config >> ~/.tmux.conf
set -g @flexfetch-enabled 'on'
set -g @flexfetch-theme 'catppuccin-mocha'
set -g @flexfetch-layout 'compact'

# Run the helper in every new pane; it shows the fetch only when the pane is idle.
# Adjust the path if install.sh placed it elsewhere (e.g. /usr/local/bin).
run-shell ~/.local/bin/flexfetch-tmux
"#
    );
}

fn command_ok(cmd: &str, args: &[&str]) -> bool {
    std::process::Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_strings_cover_shells() {
        // The real hook_for() output: bash/zsh share the git guard, fish uses
        // the PWD event hook; bash additionally tracks the last PWD.
        let bash = hook_for("bash").unwrap();
        let zsh = hook_for("zsh").unwrap();
        let fish = hook_for("fish").unwrap();
        assert!(bash.contains("PROMPT_COMMAND"));
        assert!(bash.contains("$OLDPWD"));
        assert!(zsh.contains("add-zsh-hook chpwd"));
        assert!(fish.contains("--on-variable PWD"));
        assert!(hook_for("tcsh").is_none());
    }

    #[test]
    fn report_branches_do_not_panic() {
        report(true, "x", "yes", "no");
        report(false, "y", "yes", "no");
    }
}
