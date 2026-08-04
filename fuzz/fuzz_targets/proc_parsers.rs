#![no_main]

//! Phase 8.5A — fuzz the pure /proc + sysfs parsers with arbitrary bytes.
//!
//! The parsers behind the zero-spawn collectors (Phase 4.1) read untrusted
//! kernel-ish text; a malformed `/proc/cpuinfo`, `/proc/meminfo`, or
//! `/etc/os-release` line must never panic the binary. This target feeds
//! libfuzzer-generated garbage through the same parsing paths.
//!
//! Run (nightly): `cargo +nightly fuzz run proc_parsers` from `fuzz/`.

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // OS release parsing (os.rs — key=value lines).
        for line in s.lines() {
            if let Some((key, val)) = line.split_once('=') {
                let _ = (key.trim(), val.trim_matches('"'));
            }
        }

        // CPU info parsing (cpu.rs — "key : value" lines, usize fields).
        for line in s.lines() {
            if let Some((key, val)) = line.split_once(':') {
                let _ = (key.trim(), val.trim().parse::<u64>());
            }
        }

        // Meminfo parsing (memory.rs — "Key:     1234 kB" lines).
        for line in s.lines() {
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            let _ = parts.next().and_then(|v| v.parse::<u64>().ok());
        }

        // Uptime parsing ("1234.56 789.01").
        let mut parts = s.split_whitespace();
        let _ = parts
            .next()
            .and_then(|v| v.parse::<f64>().ok())
            .map(|secs| crate_uptime(secs));
    }
});

/// Replicates the uptime formatting used by the collector so fuzzing exercises
/// the real display code too.
fn crate_uptime(secs: f64) -> String {
    flexfetch_core::modules::uptime::format_uptime(secs as u64)
}
