//! Remote fetch (`--ssh <host>...`): run `flexfetch --format json` on remote
//! hosts via SSH, parse the JSON, and render it locally.
//!
//! Strategy (per ROADMAP 2.5):
//! 1. Try `ssh host flexfetch --format json` (remote has flexfetch installed).
//! 2. If that fails, fall back to scp'ing the current (minimal) binary to the
//!    host and running it from /tmp (covers hosts without flexfetch installed).
//!
//! Hosts run in parallel (one thread each).

use flexfetch_core::SystemInfo;
use std::process::Command;

/// Fetch info from all hosts in parallel, preserving input order.
/// Uses scoped threads so the spawned threads borrow `hosts` for the
/// duration of this call rather than needing 'static data.
pub fn fetch_all(hosts: &[String]) -> Vec<(String, Result<SystemInfo, String>)> {
    std::thread::scope(|scope| {
        let threads: Vec<_> = hosts
            .iter()
            .map(|host| {
                let host = host.clone();
                scope.spawn(move || (host.clone(), fetch_one(&host)))
            })
            .collect();

        threads
            .into_iter()
            .map(|t| {
                t.join()
                    .unwrap_or_else(|_| (String::new(), Err("thread panicked".into())))
            })
            .filter(|(host, _)| !host.is_empty())
            .collect()
    })
}

/// Fetch a single host: prefer the remote binary, fall back to scp + run.
fn fetch_one(host: &str) -> Result<SystemInfo, String> {
    // 1. Remote flexfetch already installed?
    match run_remote_json(host, "flexfetch") {
        Ok(info) => Ok(info),
        Err(remote_err) => {
            // 2. Fallback: upload the local minimal binary via scp.
            let exe = std::env::current_exe().map_err(|e| format!("no local binary: {e}"))?;
            let remote_path = format!("/tmp/flexfetch-{}", std::process::id());
            let scp = Command::new("scp")
                .args([
                    exe.to_string_lossy().as_ref(),
                    &format!("{host}:{remote_path}"),
                ])
                .output()
                .map_err(|e| format!("scp failed: {e}"))?;
            if !scp.status.success() {
                return Err(format!(
                    "remote flexfetch not found and scp fallback failed: {}",
                    String::from_utf8_lossy(&scp.stderr).trim()
                ));
            }
            match run_remote_json(host, &remote_path) {
                Ok(info) => Ok(info),
                Err(e) => Err(format!(
                    "remote flexfetch failed ({remote_err}); scp fallback also failed: {e}"
                )),
            }
        }
    }
}

/// Run `ssh host <binary> --format json` and parse the output.
///
/// `--pipe` is intentionally NOT passed: over SSH stdout is already a pipe, so
/// the remote auto-activates pipe mode; passing it would break the happy path
/// against older remote flexfetch versions that predate the flag.
fn run_remote_json(host: &str, binary: &str) -> Result<SystemInfo, String> {
    let out = Command::new("ssh")
        .args([host, binary, "--format", "json"])
        .output()
        .map_err(|e| format!("ssh: {e}"))?;

    if !out.status.success() {
        return Err(format!(
            "ssh exit {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&out.stdout);
    let trimmed = stdout.trim();
    // The raw parse can fail when the remote shell prints noise (login MOTD,
    // warnings) before the JSON. In that case, slice between the first '{'
    // and the last '}' and re-parse just the JSON object.
    let json: serde_json::Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => {
            let start = trimmed
                .find('{')
                .ok_or_else(|| format!("no JSON in output from {host}"))?;
            let end = trimmed
                .rfind('}')
                .ok_or_else(|| format!("no JSON in output from {host}"))?;
            serde_json::from_str(&trimmed[start..=end])
                .map_err(|e| format!("bad JSON from {host}: {e}"))?
        }
    };

    SystemInfo::from_json(&json).map_err(|e| format!("parse: {e}"))
}
