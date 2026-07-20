use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str { "packages" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        if let Ok(count) = count_lines("/var/lib/dpkg/info", Some(".list")) {
            map.insert("dpkg".into(), count.to_string());
        }
        if let Ok(count) = run_count("pacman", &["-Q"]) {
            map.insert("pacman".into(), count.to_string());
        }
        if let Ok(count) = run_count("rpm", &["-qa"]) {
            map.insert("rpm".into(), count.to_string());
        }
        if let Ok(count) = run_count("flatpak", &["list"]) {
            map.insert("flatpak".into(), count.to_string());
        }
        if let Ok(count) = run_count("snap", &["list"]) {
            map.insert("snap".into(), count.to_string());
        }
        if let Ok(count) = run_count("brew", &["list", "--formula"]) {
            map.insert("brew_formula".into(), count.to_string());
        }
        if let Ok(count) = run_count("brew", &["list", "--cask"]) {
            map.insert("brew_cask".into(), count.to_string());
        }

        Ok(InfoValue::Map(map))
    }
}

fn count_lines(dir: &str, ext: Option<&str>) -> Result<usize> {
    let dir = std::fs::read_dir(dir).map_err(|e| crate::Error::Io(e))?;
    let count = dir.flatten()
        .filter(|e| {
            if let Some(ext) = ext {
                e.path().extension().map(|e| e == &ext[1..]).unwrap_or(false)
            } else {
                true
            }
        })
        .count();
    Ok(count)
}

fn run_count(cmd: &str, args: &[&str]) -> Result<usize> {
    let out = std::process::Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| crate::Error::Io(e))?;
    if out.status.success() {
        let s = String::from_utf8_lossy(&out.stdout);
        Ok(s.lines().count())
    } else {
        Err(crate::Error::Parse(format!("{cmd} failed")))
    }
}
