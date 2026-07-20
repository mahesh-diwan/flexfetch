use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str {
        "packages"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        if let Ok(count) = count_dpkg() {
            map.insert("dpkg".into(), count);
        }
        if let Ok(count) = count_rpm() {
            map.insert("rpm".into(), count);
        }
        if let Ok(count) = count_pacman() {
            map.insert("pacman".into(), count);
        }
        if let Ok(count) = count_flatpak() {
            map.insert("flatpak".into(), count);
        }
        if let Ok(count) = count_snap() {
            map.insert("snap".into(), count);
        }

        Ok(InfoValue::Map(map))
    }
}

fn count_dpkg() -> Result<String> {
    let output = std::process::Command::new("dpkg")
        .args(["--list"])
        .output()?;
    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| l.starts_with("ii"))
        .count();
    Ok(count.to_string())
}

fn count_rpm() -> Result<String> {
    let output = std::process::Command::new("rpm").args(["-qa"]).output()?;
    let count = String::from_utf8_lossy(&output.stdout).lines().count();
    Ok(count.to_string())
}

fn count_pacman() -> Result<String> {
    let output = std::process::Command::new("pacman").args(["-Q"]).output()?;
    let count = String::from_utf8_lossy(&output.stdout).lines().count();
    Ok(count.to_string())
}

fn count_flatpak() -> Result<String> {
    let output = std::process::Command::new("flatpak")
        .args(["list"])
        .output()?;
    let count = String::from_utf8_lossy(&output.stdout).lines().count();
    Ok(count.to_string())
}

fn count_snap() -> Result<String> {
    let output = std::process::Command::new("snap").args(["list"]).output()?;
    let count = String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .count();
    Ok(count.to_string())
}
