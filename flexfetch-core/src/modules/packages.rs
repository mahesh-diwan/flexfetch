use crate::{Context, InfoValue, Module, Result};

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str {
        "packages"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let commands: Vec<(&str, &str, &[&str])> = vec![
            ("apt", "dpkg", &["--list"]),
            ("rpm", "rpm", &["-qa"]),
            ("pacman", "pacman", &["-Q"]),
            ("flatpak", "flatpak", &["list"]),
            ("snap", "snap", &["list"]),
        ];

        let count_one = |(label, bin, args): &(&str, &str, &[&str])| {
            if let Ok(output) = std::process::Command::new(bin).args(*args).output() {
                let count = match *bin {
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
                if count > 0 {
                    // Owned label: the closure's output can't borrow from its
                    // parameter when collected across rayon's par_iter().
                    Some((label.to_string(), count))
                } else {
                    None
                }
            } else {
                None
            }
        };

        #[cfg(feature = "parallel")]
        let results: Vec<_> = {
            use rayon::prelude::*;
            commands.par_iter().filter_map(count_one).collect()
        };
        #[cfg(not(feature = "parallel"))]
        let results: Vec<_> = commands.iter().filter_map(count_one).collect();

        let total: usize = results.iter().map(|(_, c)| c).sum();
        if total == 0 {
            return Ok(InfoValue::Scalar("0".into()));
        }

        let breakdown: Vec<String> = results
            .iter()
            .map(|(name, count)| format!("{}: {}", name, count))
            .collect();
        Ok(InfoValue::Scalar(format!(
            "{} ({})",
            total,
            breakdown.join(", ")
        )))
    }
}
