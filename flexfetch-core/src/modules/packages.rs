use crate::{Context, InfoValue, Module, Result};
use rayon::prelude::*;
use std::collections::HashMap;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str {
        "packages"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let commands: Vec<(&str, &[&str])> = vec![
            ("dpkg", &["--list"]),
            ("rpm", &["-qa"]),
            ("pacman", &["-Q"]),
            ("flatpak", &["list"]),
            ("snap", &["list"]),
        ];

        let results: Vec<_> = commands
            .par_iter()
            .filter_map(|(name, args)| {
                if let Ok(output) = std::process::Command::new(name).args(*args).output() {
                    let count = match *name {
                        "dpkg" => String::from_utf8_lossy(&output.stdout)
                            .lines()
                            .filter(|l| l.starts_with("ii"))
                            .count(),
                        "snap" => String::from_utf8_lossy(&output.stdout)
                            .lines()
                            .skip(1)
                            .count(),
                        _ => String::from_utf8_lossy(&output.stdout).lines().count(),
                    };
                    Some((name.to_string(), count.to_string()))
                } else {
                    None
                }
            })
            .collect();

        let mut map = HashMap::new();
        for (name, count) in results {
            map.insert(name.into(), count);
        }
        Ok(InfoValue::Map(map))
    }
}
