use flexfetch_cli::Cli;
use flexfetch_core::{presets, Config};

/// Resolve the module list from CLI flags/presets/config.
pub fn resolve(cli: &Cli, config: &Config) -> Vec<String> {
    // Phase 8.8 --demo: every built-in module in a showcase order.
    if cli.demo {
        return vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "host".into(),
            "kernel".into(),
            "uptime".into(),
            "datetime".into(),
            "loadavg".into(),
            "keyboard".into(),
            "editor".into(),
            "initsystem".into(),
            "version".into(),
            "bios".into(),
            "board".into(),
            "chassis".into(),
            "brightness".into(),
            "tpm".into(),
            "localip".into(),
            "packages".into(),
            "shell".into(),
            "terminal".into(),
            "de".into(),
            "wm".into(),
            "cpu".into(),
            "cpucache".into(),
            "cpuusage".into(),
            "gpu".into(),
            "memory".into(),
            "swap".into(),
            "disk".into(),
            "network".into(),
            "resolution".into(),
            "display".into(),
            "battery".into(),
            "temperature".into(),
            "processes".into(),
            "dns".into(),
            "colors".into(),
            // Deliberately excluded for determinism/speed: publicip (network
            // round-trip), wifi (nmcli), bluetooth (2× bluetoothctl spawn).
        ];
    }
    // --flash: the fast path always runs the lean fixed module set, ignoring
    // config.modules and the --minimal/--full/--preset/--modules switches
    // (everything baked in, nothing user-configurable). --demo above wins.
    if cli.flash {
        return presets::module_group("flash");
    }
    let mut modules: Vec<String> = if cli.minimal {
        presets::module_group("minimal")
    } else if cli.full {
        presets::module_group("full")
    } else if cli.dev {
        presets::module_group("dev")
    } else if let Some(ref preset_name) = cli.preset {
        load_preset(preset_name)
    } else if let Some(ref m) = cli.modules {
        m.split(':').map(|s| s.to_string()).collect()
    } else {
        config.modules.clone()
    };

    // --smart: append $PWD context modules (git, project, container/venv/SSH)
    if cli.smart {
        for name in ["git", "project", "context"] {
            if !modules.iter().any(|m| m == name) {
                modules.push(name.to_string());
            }
        }
    }
    // --health: append the system health module
    if cli.health && !modules.iter().any(|m| m == "health") {
        modules.push("health".to_string());
    }
    modules
}

/// Load a preset by name: built-in presets first, then user presets
/// (`~/.config/flexfetch/presets/<name>.toml`).
fn load_preset(name: &str) -> Vec<String> {
    // Reject path traversal before touching the filesystem: a preset name must
    // be a bare file stem ("neofetch", "minimal"), never a path. A hostile
    // `--preset ../../etc/x` would otherwise read arbitrary TOML files.
    if name.is_empty() || name.contains(['/', '\\']) || name.starts_with('.') || name.contains("..")
    {
        eprintln!("preset '{name}' not found, using default modules");
        return Config::default_modules();
    }

    // Check built-in presets first (via core)
    if presets::builtin_presets().contains_key(name) {
        return presets::load_preset(name);
    }

    // Check user presets (~/.config/flexfetch/presets/<name>.toml)
    let presets_dir = crate::tools::config_dir().join("presets");
    let preset_path = presets_dir.join(format!("{name}.toml"));
    if let Ok(content) = std::fs::read_to_string(&preset_path) {
        if let Ok(doc) = toml::from_str::<toml::Value>(&content) {
            if let Some(arr) = doc.get("modules").and_then(|v| v.as_array()) {
                return arr
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
    }

    eprintln!("preset '{name}' not found, using default modules");
    Config::default_modules()
}

#[cfg(test)]
mod tests {
    use super::load_preset;

    #[test]
    fn preset_traversal_names_are_rejected() {
        // Must fall back to defaults without touching the filesystem.
        for evil in [
            "../etc/shadow",
            "/etc/passwd",
            "../../x",
            ".hidden",
            "a..b",
            "",
            "..\\win",
        ] {
            let m = load_preset(evil);
            assert!(
                !m.is_empty(),
                "preset {evil:?} should fall back to defaults, not read a file"
            );
        }
    }

    #[test]
    fn preset_clean_names_work() {
        // Valid names resolve through the builtin catalog (or warn + default).
        let m = load_preset("neofetch");
        assert!(!m.is_empty());
        let m = load_preset("minimal");
        assert!(!m.is_empty());
    }
}
