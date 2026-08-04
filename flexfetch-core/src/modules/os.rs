use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str {
        "os"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if let Some((key, val)) = line.split_once('=') {
                        let clean = val.trim_matches('"');
                        match key {
                            "NAME" => {
                                map.insert("name".into(), clean.into());
                            }
                            "PRETTY_NAME" => {
                                map.insert("pretty_name".into(), clean.into());
                            }
                            "VERSION_ID" => {
                                map.insert("version".into(), clean.into());
                            }
                            "ID" => {
                                map.insert("id".into(), clean.into());
                            }
                            "BUILD_ID" => {
                                map.insert("build_id".into(), clean.into());
                            }
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

            // Phase 8.10 — WSL detection: the WSLInterop marker exists on both
            // WSL1 and WSL2; the kernel version string disambiguates them.
            // Only spawns `cmd.exe /c ver` on an actual WSL system (off the
            // default path otherwise, so cold start is unaffected).
            if std::path::Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists() {
                let kernel = std::fs::read_to_string("/proc/sys/kernel/osrelease")
                    .unwrap_or_default()
                    .to_lowercase();
                let wsl = if kernel.contains("wsl2") || kernel.contains("microsoft-standard") {
                    "2"
                } else {
                    "1"
                };
                map.insert("wsl".into(), wsl.to_string());

                // Windows host version via the interop boundary (no-op off WSL).
                if let Ok(output) = std::process::Command::new("cmd.exe")
                    .args(["/c", "ver"])
                    .output()
                {
                    let v = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .replace("Microsoft Windows ", "");
                    if !v.is_empty() {
                        map.insert("windows_host".into(), format!("Windows {v}"));
                    }
                }

                // Append the marker to the pretty name so the template's OS row
                // shows "Ubuntu 24.04 (WSL2)" without a template change.
                if let Some(pretty) = map.get_mut("pretty_name") {
                    if !pretty.contains("(WSL") {
                        pretty.push_str(&format!(" (WSL{wsl})"));
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            map.insert("name".into(), "macOS".into());
            if let Ok(output) = std::process::Command::new("sw_vers")
                .arg("-productVersion")
                .output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() {
                    map.insert("version".into(), v);
                }
            }
        }

        map.insert("arch".into(), std::env::consts::ARCH.to_string());
        Ok(InfoValue::Map(map))
    }
}
