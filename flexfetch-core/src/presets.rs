use crate::Config;
use std::collections::HashMap;

pub fn module_group(name: &str) -> Vec<String> {
    match name {
        "flash" => {
            let mut v = module_group("minimal");
            v.push("memory".into());
            v
        }
        "minimal" => vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
        ],
        "full" => Config::default_modules(),
        "dev" => vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "shell".into(),
            "terminal".into(),
        ],
        _ => Config::default_modules(),
    }
}

pub fn builtin_presets() -> HashMap<String, Vec<String>> {
    let mut presets = HashMap::new();
    presets.insert("default".into(), Config::default_modules());
    presets.insert("minimal".into(), module_group("minimal"));
    presets.insert("full".into(), module_group("full"));
    presets.insert("dev".into(), module_group("dev"));
    presets.insert(
        "server".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "network".into(),
        ],
    );
    presets.insert(
        "laptop".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "battery".into(),
            "shell".into(),
        ],
    );
    presets.insert(
        "ci".into(),
        vec![
            "os".into(),
            "kernel".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "network".into(),
        ],
    );
    presets.insert(
        "neofetch".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "host".into(),
            "kernel".into(),
            "uptime".into(),
            "packages".into(),
            "shell".into(),
            "de".into(),
            "wm".into(),
            "terminal".into(),
            "cpu".into(),
            "gpu".into(),
            "memory".into(),
            "disk".into(),
            "battery".into(),
            "colors".into(),
        ],
    );
    presets
}

pub fn load_preset(name: &str) -> Vec<String> {
    if let Some(modules) = builtin_presets().get(name) {
        return modules.clone();
    }
    // User presets loaded by CLI (needs config_dir path)
    Config::default_modules()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_group_minimal_has_required_modules() {
        let group = module_group("minimal");
        assert!(group.contains(&"title".to_string()));
        assert!(group.contains(&"os".to_string()));
        assert!(group.contains(&"kernel".to_string()));
    }

    #[test]
    fn builtin_presets_contains_default() {
        let presets = builtin_presets();
        assert!(presets.contains_key("default"));
        assert!(presets.contains_key("minimal"));
        assert!(presets.contains_key("neofetch"));
    }

    #[test]
    fn flash_includes_minimal_plus_memory() {
        let flash = module_group("flash");
        let minimal = module_group("minimal");
        assert!(flash.len() > minimal.len());
        assert!(flash.contains(&"memory".to_string()));
    }
}
