//! Phase 5.7 — plugin registry (`flexfetch plugin search|install|list|update`).
//!
//! Lua plugins are distributed through a hosted TOML registry
//! (`registry/plugins.toml`, raw on GitHub). Every download is SHA-256
//! verified against the registry entry before it touches the plugins dir
//! (`~/.config/flexfetch/plugins/`), and `min_flexfetch_version` is checked
//! against the running binary — a bad or incompatible plugin can never land.
//! Pure std; the only external commands are curl (download) and
//! sha256sum/shasum (checksum), both consistent with install.sh.

use std::path::PathBuf;

/// The hosted registry (raw.githubusercontent, same channel as install.sh and
/// the hardware DB). Overridable for testing / mirrors.
const REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/mahesh-diwan/flexfetch/main/registry/plugins.toml";

/// Phase 8.12 — the project's trusted Ed25519 publisher key (base64). Every
/// registry entry that carries a `signature` must verify against this key, or
/// the install is refused. Generate a new one with
/// `cargo run --example registry_sign -- <plugin.lua> <seed-hex>` and update
/// BOTH this const and the registry entries.
const TRUSTED_PUBLISHER_KEY: &str = "IHoGeJKCHiXXcPH7oMR8Ef9LgT5UFi7Onrg54HYjGrY=";

/// One registry entry.
pub struct PluginEntry {
    pub name: String,
    pub description: String,
    pub version: String,
    pub min_flexfetch_version: String,
    pub url: String,
    pub sha256: String,
    /// Phase 8.12: base64 Ed25519 signature over the plugin bytes, made with
    /// the project's private key. `None` for entries that predate signing
    /// (sha256-only, with a notice).
    pub signature: Option<String>,
}

/// The directory installed plugins live in (matches the `--list-modules`
/// hint: `~/.config/flexfetch/plugins/`).
pub fn plugins_dir() -> PathBuf {
    let xdg = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".config")
        });
    xdg.join("flexfetch").join("plugins")
}

/// Download the registry TOML and parse it (best-effort field extraction; no
/// serde derive needed — the `toml` crate's Value type suffices).
fn fetch_registry() -> Result<Vec<PluginEntry>, String> {
    let out = run_cmd("curl", &["-fsSL", REGISTRY_URL])?;
    parse_registry(&out)
}

/// Parse registry TOML text into entries (also used by tests, offline).
fn parse_registry(text: &str) -> Result<Vec<PluginEntry>, String> {
    let doc: toml::Value = text
        .parse()
        .map_err(|e| format!("registry is not valid TOML: {e}"))?;
    let arr = doc
        .get("plugins")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "registry has no [[plugins]] section".to_string())?;

    let mut entries = Vec::new();
    for item in arr {
        let get = |k: &str| -> Result<String, String> {
            item.get(k)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| format!("registry entry missing `{k}`"))
        };
        entries.push(PluginEntry {
            name: get("name")?,
            description: get("description")?,
            version: get("version")?,
            min_flexfetch_version: get("min_flexfetch_version")?,
            url: get("url")?,
            sha256: get("sha256")?,
            signature: item
                .get("signature")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        });
    }
    Ok(entries)
}

/// `flexfetch plugin search <query>` — case-insensitive name/description match.
fn search(query: &str) {
    let q = query.trim().to_lowercase();
    let entries = match fetch_registry() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("plugin search failed: {e}");
            std::process::exit(1);
        }
    };
    let hits: Vec<&PluginEntry> = entries
        .iter()
        .filter(|e| e.name.to_lowercase().contains(&q) || e.description.to_lowercase().contains(&q))
        .collect();
    if hits.is_empty() {
        println!("no plugins match '{query}' in the registry");
        return;
    }
    println!("{} plugin(s) match '{query}':", hits.len());
    for e in hits {
        println!("  {:<14} v{:<8} {}", e.name, e.version, e.description);
    }
}

/// `flexfetch plugin install <name>` — verify version + checksum, then write
/// to the plugins dir.
fn install(name: &str) {
    let entries = match fetch_registry() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("plugin install failed: {e}");
            std::process::exit(1);
        }
    };
    let entry = match entries.iter().find(|e| e.name == name) {
        Some(e) => e,
        None => {
            eprintln!(
                "plugin '{name}' not found in the registry (try `flexfetch plugin search {name}`)"
            );
            std::process::exit(1);
        }
    };

    // Min-version gate: refuse to install a plugin that needs a newer flexfetch.
    let running = env!("CARGO_PKG_VERSION");
    if !version_ge(running, &entry.min_flexfetch_version) {
        eprintln!(
            "plugin '{name}' requires flexfetch >= {} (running {running}) — update flexfetch first",
            entry.min_flexfetch_version
        );
        std::process::exit(1);
    }

    // Download to a temp file, verify the checksum, then move it into place.
    let dir = plugins_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("cannot create plugins dir {}: {e}", dir.display());
        std::process::exit(1);
    }
    let tmp = std::env::temp_dir().join(format!("flexfetch-plugin-{name}-{}", std::process::id()));
    if let Err(e) = run_cmd("curl", &["-fsSL", &entry.url, "-o", &tmp.to_string_lossy()]) {
        eprintln!("download failed for '{name}': {e}");
        std::process::exit(1);
    }

    // Phase 8.12: verify the Ed25519 signature over the raw plugin bytes
    // BEFORE the SHA-256 check, so a tampered payload fails closed even if
    // the checksum file were also compromised.
    if let Some(sig_b64) = &entry.signature {
        let bytes = match std::fs::read(&tmp) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read download: {e}");
                let _ = std::fs::remove_file(&tmp);
                std::process::exit(1);
            }
        };
        if let Err(e) = verify_signature(&bytes, sig_b64, TRUSTED_PUBLISHER_KEY) {
            eprintln!(
                "signature verification failed for '{name}': {e}\nrefusing to install (the entry is not signed by the project's publisher key)."
            );
            let _ = std::fs::remove_file(&tmp);
            std::process::exit(1);
        }
    } else {
        eprintln!(
            "note: '{name}' has no Ed25519 signature (pre-8.12 registry entry) — sha256 check only."
        );
    }

    let actual = match sha256_hex(&tmp) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{e}");
            let _ = std::fs::remove_file(&tmp);
            std::process::exit(1);
        }
    };
    if !actual.eq_ignore_ascii_case(&entry.sha256) {
        eprintln!(
            "checksum mismatch for '{name}':\n  registry: {}\n  download: {}\nrefusing to install (possible tampering or a stale registry entry).",
            entry.sha256, actual
        );
        let _ = std::fs::remove_file(&tmp);
        std::process::exit(1);
    }

    let dest = dir.join(format!("{}.lua", entry.name));
    match std::fs::rename(&tmp, &dest) {
        Ok(()) => println!(
            "installed {name} v{} → {} (sha256 verified)",
            entry.version,
            dest.display()
        ),
        Err(e) => {
            eprintln!("write failed: {e}");
            let _ = std::fs::remove_file(&tmp);
            std::process::exit(1);
        }
    }
}

/// `flexfetch plugin list` — installed plugins + registry availability.
fn list() {
    let dir = plugins_dir();
    let installed: Vec<String> = match std::fs::read_dir(&dir) {
        Ok(entries) => entries
            .flatten()
            .filter(|e| e.path().extension().map(|x| x == "lua").unwrap_or(false))
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    if installed.is_empty() {
        println!(
            "no plugins installed ({})\n  try: flexfetch plugin search <query>",
            dir.display()
        );
    } else {
        println!("installed plugins ({}):", dir.display());
        for name in installed {
            println!("  {name}");
        }
    }
    println!(
        "\nregistry: {REGISTRY_URL}\n  ({} entry/entries — network required)",
        match fetch_registry() {
            Ok(e) => e.len(),
            Err(_) => 0,
        }
    );
}

/// `flexfetch plugin update` — re-install every installed plugin that is still
/// in the registry (idempotent, re-verifies checksums + min-version each time).
fn update() {
    let dir = plugins_dir();
    let installed: Vec<String> = match std::fs::read_dir(&dir) {
        Ok(entries) => entries
            .flatten()
            .filter(|e| e.path().extension().map(|x| x == "lua").unwrap_or(false))
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            })
            .collect(),
        Err(_) => Vec::new(),
    };
    if installed.is_empty() {
        println!("no plugins installed — nothing to update");
        return;
    }
    let entries = match fetch_registry() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("plugin update failed: {e}");
            std::process::exit(1);
        }
    };
    let mut updated = 0;
    let mut skipped = 0;
    for name in &installed {
        if entries.iter().any(|e| &e.name == name) {
            install(name);
            updated += 1;
        } else {
            println!("  {name}: no longer in the registry (left as-is)");
            skipped += 1;
        }
    }
    println!("plugin update done: {updated} updated, {skipped} skipped");
}

/// Dispatch a `flexfetch plugin` subcommand. The action enum lives at the
/// crate root (clap derive in main.rs); this module only implements the logic.
pub fn run(action: &crate::PluginAction) {
    match action {
        crate::PluginAction::Search { query } => search(query),
        crate::PluginAction::Install { name } => install(name),
        crate::PluginAction::List => list(),
        crate::PluginAction::Update => update(),
    }
}

/// `a >= b` for dotted numeric versions ("1.4.2" >= "1.4"). Non-numeric
/// segments are ignored; missing segments count as 0.
fn version_ge(a: &str, b: &str) -> bool {
    let nums = |s: &str| -> Vec<u64> {
        s.split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse().ok())
            .collect()
    };
    let (na, nb) = (nums(a), nums(b));
    for i in 0..na.len().max(nb.len()) {
        let av = na.get(i).copied().unwrap_or(0);
        let bv = nb.get(i).copied().unwrap_or(0);
        if av != bv {
            return av > bv;
        }
    }
    true
}

/// Phase 8.12 — verify a base64 Ed25519 signature over `bytes` against the
/// base64 public key. Uses `ed25519-compact` (pure Rust, no C deps).
fn verify_signature(bytes: &[u8], signature_b64: &str, pubkey_b64: &str) -> Result<(), String> {
    use ed25519_compact::{PublicKey, Signature};

    let pk_bytes = base64_decode(pubkey_b64)
        .ok_or_else(|| "trusted publisher key is not valid base64".to_string())?;
    let sig_bytes =
        base64_decode(signature_b64).ok_or_else(|| "signature is not valid base64".to_string())?;

    let pk = PublicKey::from_slice(&pk_bytes).map_err(|e| format!("invalid publisher key: {e}"))?;
    let sig = Signature::from_slice(&sig_bytes).map_err(|e| format!("invalid signature: {e}"))?;

    pk.verify(bytes, &sig)
        .map_err(|e| format!("Ed25519 verify failed: {e}"))
}

/// Minimal standard-base64 decoder (std only; the `base64` crate is gated
/// behind the `qr` feature, so the registry keeps itself dependency-light).
fn base64_decode(input: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let mut buf: u32 = 0;
    let mut bits = 0u32;
    for &c in input.as_bytes() {
        let val = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => return None,
        };
        buf = (buf << 6) | u32::from(val);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Some(out)
}

/// sha256 of a file, via sha256sum (Linux) or shasum -a 256 (macOS).
fn sha256_hex(path: &std::path::Path) -> Result<String, String> {
    let stdout = |cmd: &str, args: &[&str]| -> Option<String> {
        std::process::Command::new(cmd)
            .args(args)
            .arg(path)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
    };
    let out = stdout("sha256sum", &[]).or_else(|| stdout("shasum", &["-a", "256"]));
    match out {
        Some(s) => s
            .split_whitespace()
            .next()
            .map(|h| h.to_string())
            .ok_or_else(|| "could not parse checksum output".to_string()),
        None => Err(
            "no sha256 tool found (sha256sum or shasum) — cannot verify the download".to_string(),
        ),
    }
}

/// Run a command, returning trimmed stdout or a readable error.
fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    match std::process::Command::new(cmd).args(args).output() {
        Ok(o) if o.status.success() => Ok(String::from_utf8_lossy(&o.stdout).trim().to_string()),
        Ok(o) => Err(format!(
            "{cmd} failed ({}): {}",
            o.status,
            String::from_utf8_lossy(&o.stderr).trim()
        )),
        Err(e) => Err(format!("{cmd}: {e} (is it installed?)")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[[plugins]]
name = "hello"
description = "Example plugin"
version = "1.0.0"
min_flexfetch_version = "1.0.0"
url = "https://example.com/hello.lua"
sha256 = "cd1357d071f02094ae1b33eac710bec19dc2f51f9f4c79896c603c04d4de5608"
signature = "+g48Ps3mYZC8OXw6URMeAZTB2nxi1TX7/QumdzT/ehxCMvhWH5S+vefh/0kD57MIzH0wAvDfNnHXnFiwUbvSCA=="

[[plugins]]
name = "cpu-heavy"
description = "Shows CPU details"
version = "0.9.1"
min_flexfetch_version = "2.0.0"
url = "https://example.com/cpu.lua"
sha256 = "abc"
"#;

    #[test]
    fn parse_registry_entries() {
        let entries = parse_registry(SAMPLE).expect("sample parses");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "hello");
        assert_eq!(entries[0].signature.as_deref(),
            Some("+g48Ps3mYZC8OXw6URMeAZTB2nxi1TX7/QumdzT/ehxCMvhWH5S+vefh/0kD57MIzH0wAvDfNnHXnFiwUbvSCA=="));
        assert_eq!(entries[1].min_flexfetch_version, "2.0.0");
        assert!(
            entries[1].signature.is_none(),
            "unsigned entries parse as None"
        );
    }

    #[test]
    fn signature_roundtrip_accepts_then_rejects_tampering() {
        use ed25519_compact::{KeyPair, Seed};

        // Self-contained keypair from a fixed seed — deterministic test.
        let kp = KeyPair::from_seed(Seed::new([7u8; 32]));
        let plugin =
            b"return { name = \"x\", collect = function() return { value = \"1\" } end }\n";
        let sig = kp.sk.sign(plugin, None);
        let pk_b64 = base64_encode_std(kp.pk.as_ref());
        let sig_b64 = base64_encode_std(sig.as_ref());

        // Genuine signature verifies.
        assert!(verify_signature(plugin, &sig_b64, &pk_b64).is_ok());

        // Tampered payload fails closed.
        let mut tampered = plugin.to_vec();
        tampered[10] ^= 0xff;
        assert!(verify_signature(&tampered, &sig_b64, &pk_b64).is_err());

        // Wrong key fails.
        let other = KeyPair::from_seed(Seed::new([8u8; 32]));
        assert!(verify_signature(plugin, &sig_b64, &base64_encode_std(other.pk.as_ref())).is_err());

        // Garbage inputs fail gracefully (never panic).
        assert!(verify_signature(b"", "!!not-base64!!", &pk_b64).is_err());
        assert!(verify_signature(plugin, &sig_b64, "not base64").is_err());
    }

    #[test]
    fn base64_decode_roundtrip() {
        let input = b"flexfetch-plugin-sig-test";
        let encoded = base64_encode_std(input);
        assert_eq!(base64_decode(&encoded).as_deref(), Some(&input[..]));
        assert_eq!(base64_decode("!!!!"), None);
        assert_eq!(base64_decode("YWJj"), Some(b"abc".to_vec())); // "abc"
    }

    /// Test-only std base64 encoder (mirrors the registry_sign example).
    fn base64_encode_std(input: &[u8]) -> String {
        const TABLE: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
        for chunk in input.chunks(3) {
            let b = [
                chunk[0],
                *chunk.get(1).unwrap_or(&0),
                *chunk.get(2).unwrap_or(&0),
            ];
            let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
            out.push(TABLE[(n >> 18) as usize & 63] as char);
            out.push(TABLE[(n >> 12) as usize & 63] as char);
            if chunk.len() > 1 {
                out.push(TABLE[(n >> 6) as usize & 63] as char);
            } else {
                out.push('=');
            }
            if chunk.len() > 2 {
                out.push(TABLE[n as usize & 63] as char);
            } else {
                out.push('=');
            }
        }
        out
    }

    #[test]
    fn parse_registry_rejects_missing_fields() {
        assert!(parse_registry("[[plugins]]\nname = \"x\"\n").is_err());
        assert!(parse_registry("not toml at all [").is_err());
        assert!(parse_registry("[]").is_err());
    }

    #[test]
    fn version_gate() {
        assert!(version_ge("1.4.2", "1.4"));
        assert!(version_ge("1.4.2", "1.4.2"));
        assert!(version_ge("2.0", "1.9.9"));
        assert!(version_ge("1.0.0-alpha", "1.0.0"));
        assert!(!version_ge("1.4.2", "1.4.3"));
        assert!(!version_ge("1.0", "2.0"));
    }

    #[test]
    fn sha256_hex_parses_standard_output() {
        // Use a known file: the workspace Cargo.lock if present, else skip.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("Cargo.lock");
        if !path.exists() {
            return;
        }
        let hex = sha256_hex(&path).expect("sha tool present");
        assert_eq!(hex.len(), 64);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
