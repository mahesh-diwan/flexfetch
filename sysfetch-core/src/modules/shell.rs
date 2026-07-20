use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct ShellModule;

impl Module for ShellModule {
    fn name(&self) -> &'static str { "shell" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let shell_path = std::env::var("SHELL").unwrap_or_default();
        let name = shell_path.rsplit('/').next().unwrap_or("unknown").to_string();
        let version = get_version(&name);

        let mut map = HashMap::new();
        map.insert("name".into(), name);
        if let Some(v) = version {
            map.insert("version".into(), v);
        }
        Ok(InfoValue::Map(map))
    }
}

fn get_version(shell: &str) -> Option<String> {
    match shell {
        "bash" | "zsh" | "fish" | "nu" => {
            let out = std::process::Command::new(shell)
                .arg("--version").output().ok()?;
            let s = String::from_utf8_lossy(&out.stdout);
            s.lines().next().map(|l| l.to_string())
        }
        _ => None,
    }
}
