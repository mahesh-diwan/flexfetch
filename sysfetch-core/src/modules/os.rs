use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str { "os" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if let Some((key, val)) = line.split_once('=') {
                        let clean = val.trim_matches('"');
                        match key {
                            "NAME" => { map.insert("name".into(), clean.into()); }
                            "PRETTY_NAME" => { map.insert("pretty_name".into(), clean.into()); }
                            "VERSION_ID" => { map.insert("version".into(), clean.into()); }
                            "ID" => { map.insert("id".into(), clean.into()); }
                            "BUILD_ID" => { map.insert("build_id".into(), clean.into()); }
                            _ => {}
                        }
                    }
                }
            }
            if !map.contains_key("name") {
                if let Ok(_arch) = std::fs::read_to_string("/etc/arch-release") {
                    map.insert("name".into(), "Arch Linux".into());
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            map.insert("name".into(), "macOS".into());
            if let Ok(output) = std::process::Command::new("sw_vers")
                .arg("-productVersion").output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() { map.insert("version".into(), v); }
            }
        }

        map.insert("arch".into(), std::env::consts::ARCH.to_string());
        Ok(InfoValue::Map(map))
    }
}
