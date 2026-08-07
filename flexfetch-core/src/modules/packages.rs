use crate::{Context, InfoValue, Module, Result};

pub struct PackagesModule;

impl Module for PackagesModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut results: Vec<(String, usize)> = Vec::new();

        // Phase 4.1: parse the package databases directly (no subprocesses).
        // Each returns None when the DB isn't present, and we fall back to the
        // CLI for that manager (rpm has no plain-file DB — Berkeley DB only).
        for (label, count) in [
            ("dpkg", count_dpkg()),
            ("pacman", count_pacman()),
            ("flatpak", count_flatpak()),
            ("snap", count_snap()),
            ("rpm", count_rpm()),
        ] {
            if let Some(n) = count {
                results.push((label.to_string(), n));
            }
        }

        // If no DB was readable at all (e.g. macOS, or a foreign distro), fall
        // back to the package CLIs.
        if results.is_empty() {
            results = count_all_cli();
        }

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

/// Count installed dpkg packages from /var/lib/dpkg/status (no `dpkg` spawn).
fn count_dpkg() -> Option<usize> {
    let content = std::fs::read_to_string("/var/lib/dpkg/status").ok()?;
    let count = content
        .lines()
        .filter(|l| l.starts_with("Status: install ok installed"))
        .count();
    if count == 0 {
        None
    } else {
        Some(count)
    }
}

/// Count installed pacman packages: one directory per package in /var/lib/pacman/local/.
/// Skip non-directory entries (ALPM_DB_VERSION, *_NOTICE files) — those are DB
/// metadata, not packages.
fn count_pacman() -> Option<usize> {
    let entries = std::fs::read_dir("/var/lib/pacman/local/").ok()?;
    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .count();
    if count == 0 {
        None
    } else {
        Some(count)
    }
}

/// Count flatpak apps + runtimes by directory (system + per-user installs).
fn count_flatpak() -> Option<usize> {
    let mut count = 0usize;
    for base in ["/var/lib/flatpak/app", "/var/lib/flatpak/runtime"] {
        if let Ok(entries) = std::fs::read_dir(base) {
            count += entries.filter_map(|e| e.ok()).count();
        }
    }
    if count == 0 {
        None
    } else {
        Some(count)
    }
}

/// Count snap packages: one .snap file per revision in /var/lib/snapd/snaps/.
fn count_snap() -> Option<usize> {
    let entries = std::fs::read_dir("/var/lib/snapd/snaps/").ok()?;
    let count = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".snap"))
        .count();
    if count == 0 {
        None
    } else {
        Some(count)
    }
}

/// rpm keeps its database in a Berkeley DB we don't parse — shell out (only
/// fires on rpm-based systems; returns None immediately when `rpm` is absent).
fn count_rpm() -> Option<usize> {
    count_cli("rpm", &["-qa"], |s| s.lines().count())
}

/// Run one package CLI and map its stdout through `count` to a package count.
fn count_cli(bin: &str, args: &[&str], count: impl Fn(&str) -> usize) -> Option<usize> {
    let output = std::process::Command::new(bin).args(args).output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let n = count(&stdout);
    if n > 0 {
        Some(n)
    } else {
        None
    }
}

/// A package-manager CLI probe: (label, binary, args, stdout-to-count mapper).
type PkgCli = (
    &'static str,
    &'static str,
    &'static [&'static str],
    fn(&str) -> usize,
);

/// Fallback: query every package manager CLI (original behavior). Restores the
/// rayon-parallel path when the `parallel` feature is on (only fires when no
/// package DB was readable, e.g. macOS / foreign distro).
fn count_all_cli() -> Vec<(String, usize)> {
    let commands: Vec<PkgCli> = vec![
        ("dpkg", "dpkg", &["--list"], |s| {
            s.lines().filter(|l| l.starts_with("ii")).count()
        }),
        ("rpm", "rpm", &["-qa"], |s| s.lines().count()),
        ("pacman", "pacman", &["-Q"], |s| s.lines().count()),
        ("flatpak", "flatpak", &["list"], |s| s.lines().count()),
        ("snap", "snap", &["list"], |s| s.lines().skip(1).count()),
    ];

    let probe = |(label, bin, args, count): &PkgCli| {
        count_cli(bin, args, *count).map(|n| (label.to_string(), n))
    };

    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        commands.par_iter().filter_map(probe).collect()
    }
    #[cfg(not(feature = "parallel"))]
    {
        commands.iter().filter_map(probe).collect()
    }
}
