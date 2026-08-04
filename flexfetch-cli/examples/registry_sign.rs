//! Publisher signing tool for the plugin registry (Phase 8.12).
//!
//! The client (`flexfetch plugin install`) verifies each plugin against the
//! project's trusted Ed25519 public key embedded in `registry.rs`. This tool
//! produces the `signature` value for a registry entry:
//!
//! ```sh
//! # deterministic key from a seed (keep the seed secret — it is the identity)
//! cargo run --example registry_sign -- ./your.lua 0123456789abcdef...
//! ```
//!
//! It prints two lines: the base64 **public key** (paste into the client's
//! `TRUSTED_PUBLISHER_KEY` once) and the base64 **signature** (paste into the
//! `[[plugins]]` entry, alongside `sha256`). Signatures cover the raw plugin
//! bytes, so the client verifies them before the SHA-256 check.

use ed25519_compact::{KeyPair, Seed};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(file) = args.next() else {
        eprintln!("usage: registry_sign <plugin.lua> <seed-hex-32-bytes>");
        return ExitCode::FAILURE;
    };
    let Some(seed_hex) = args.next() else {
        eprintln!("usage: registry_sign <plugin.lua> <seed-hex-32-bytes>");
        return ExitCode::FAILURE;
    };

    let bytes = match std::fs::read(&file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("read {file}: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Accept 32-byte hex seeds (with or without 0x prefix).
    let seed_hex = seed_hex.strip_prefix("0x").unwrap_or(&seed_hex);
    let seed: [u8; 32] = match decode_hex(seed_hex) {
        Some(s) if s.len() == 32 => s.try_into().expect("length checked"),
        _ => {
            eprintln!("seed must be exactly 32 bytes of hex (64 chars)");
            return ExitCode::FAILURE;
        }
    };

    let keypair = KeyPair::from_seed(Seed::new(seed));
    // `None` noise keeps signatures deterministic across machines/runs.
    let signature = keypair.sk.sign(&bytes, None);

    println!("{}", base64_std(keypair.pk.as_ref()));
    println!("{}", base64_std(signature.as_ref()));
    eprintln!(
        "signed {file} ({} bytes); paste the second line into the registry entry",
        signature.as_ref().len()
    );
    ExitCode::SUCCESS
}

/// Strict hex decoder (no alloc-heavy deps for a tiny tool).
fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for chunk in bytes.chunks(2) {
        let hi = hex_nibble(chunk[0])?;
        let lo = hex_nibble(chunk[1])?;
        out.push((hi << 4) | lo);
    }
    Some(out)
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Minimal standard base64 encode (no extra dep for the example; the main
/// binary already has `base64` behind the `qr` feature, so self-contain here).
fn base64_std(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
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
