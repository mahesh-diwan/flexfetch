//! Phase 5.6 — critical health notifications (feature `notifications`).
//!
//! `--daemon`: polls cpu/mem/disk/temp every `interval` seconds (default 60)
//! and fires a desktop notification when a metric crosses its threshold. A
//! per-metric cooldown prevents spam: after firing, that metric stays silenced
//! until it recovers below (threshold − 5) and breaches again — so you get one
//! notification per critical episode, not one per poll.
//!
//! Notifications use notify-rust (pure-Rust zbus backend on Linux/BSD,
//! mac-notification-sys on macOS, tauri-winrt on Windows — all feature-gated
//! here). Falls back to a stderr banner if the session has no notification
//! daemon (e.g. headless/SSH).

use crate::monitor;
use flexfetch_core::Context;
use std::collections::HashMap;

/// Default thresholds: cpu 90 %, mem 90 %, disk 90 %, temp 85 °C.
const DEFAULT_THRESHOLDS: [(&str, f64); 4] =
    [("cpu", 90.0), ("mem", 90.0), ("disk", 90.0), ("temp", 85.0)];
/// A metric must drop this far below its threshold before it can fire again.
const REARM_HYSTERESIS: f64 = 5.0;

pub struct Thresholds {
    map: HashMap<String, f64>,
}

impl Thresholds {
    /// Parse `--threshold cpu=95,mem=88` overrides on top of the defaults.
    /// Unknown keys are ignored (with a warning to stderr).
    pub fn new(overrides: &[String]) -> Self {
        let mut map: HashMap<String, f64> = DEFAULT_THRESHOLDS
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect();
        for o in overrides {
            let (key, val) = match o.split_once('=') {
                Some((k, v)) => (k.trim().to_lowercase(), v.trim().parse::<f64>()),
                None => {
                    eprintln!(
                        "[flexfetch] --threshold {o}: expected key=value (cpu/mem/disk/temp)"
                    );
                    continue;
                }
            };
            match val {
                Ok(v) if map.contains_key(&key) => {
                    map.insert(key, v);
                }
                Ok(_) => {
                    eprintln!("[flexfetch] --threshold {o}: unknown metric (use cpu/mem/disk/temp)")
                }
                Err(_) => eprintln!("[flexfetch] --threshold {o}: value must be a number"),
            }
        }
        Thresholds { map }
    }

    pub fn get(&self, key: &str) -> f64 {
        self.map.get(key).copied().unwrap_or(f64::INFINITY)
    }
}

/// Send a desktop notification (or a stderr banner when no notifier is usable).
fn notify(title: &str, body: &str) {
    let sent = notify_rust::Notification::new()
        .summary(title)
        .body(body)
        .appname("flexfetch")
        .show();
    match sent {
        Ok(_) => {}
        Err(e) => {
            // Headless/SSH: still surface the breach in the terminal.
            eprintln!("\x1b[1;31m[flexfetch] {title}: {body}\x1b[0m (notify failed: {e})");
        }
    }
}

/// Per-metric breach state: armed = will fire on the next breach.
struct Armed {
    cpu: bool,
    mem: bool,
    disk: bool,
    temp: bool,
}

impl Armed {
    fn new() -> Self {
        Armed {
            cpu: true,
            mem: true,
            disk: true,
            temp: true,
        }
    }
}

/// Check one metric; returns the message to notify (None = no breach or armed).
fn check(
    value: Option<f64>,
    threshold: f64,
    armed: &mut bool,
    label: &str,
    unit: &str,
) -> Option<(String, String)> {
    let v = value?;
    if v > threshold {
        if *armed {
            *armed = false;
            Some((
                format!("flexfetch: {label} critical"),
                format!("{label} at {v:.1}{unit} — threshold {threshold:.0}{unit}"),
            ))
        } else {
            None
        }
    } else {
        // Re-arm only after recovering below threshold − hysteresis.
        if v < threshold - REARM_HYSTERESIS {
            *armed = true;
        }
        None
    }
}

/// `--daemon`: poll + notify loop until Ctrl+C.
pub fn run(ctx: Context, interval_secs: u64, thresholds: Thresholds) {
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    let _ = ctrlc::set_handler(move || r.store(false, std::sync::atomic::Ordering::SeqCst));
    let mut armed = Armed::new();

    eprintln!(
        "[flexfetch] daemon: polling every {interval_secs}s (cpu>{:.0}% mem>{:.0}% disk>{:.0}% temp>{:.0}°C)",
        thresholds.get("cpu"),
        thresholds.get("mem"),
        thresholds.get("disk"),
        thresholds.get("temp")
    );
    eprintln!("[flexfetch] daemon running (Ctrl+C to stop)");

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let h = monitor::sample_health(&ctx);
        let mut fired = false;

        for (v, th, arm, label, unit) in [
            (h.cpu_pct, thresholds.get("cpu"), &mut armed.cpu, "CPU", "%"),
            (
                h.mem_pct.map(f64::from),
                thresholds.get("mem"),
                &mut armed.mem,
                "Memory",
                "%",
            ),
            (
                h.disk_pct.map(f64::from),
                thresholds.get("disk"),
                &mut armed.disk,
                "Disk",
                "%",
            ),
            (
                h.temp_c,
                thresholds.get("temp"),
                &mut armed.temp,
                "Temperature",
                "°C",
            ),
        ] {
            if let Some((title, body)) = check(v, th, arm, label, unit) {
                notify(&title, &body);
                fired = true;
            }
        }

        if !fired && ctx.debug {
            eprintln!(
                "[flexfetch] daemon: ok (cpu {:?} mem {:?} disk {:?} temp {:?})",
                h.cpu_pct, h.mem_pct, h.disk_pct, h.temp_c
            );
        }

        for _ in 0..interval_secs.max(1) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if !running.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
        }
    }
    eprintln!("[flexfetch] daemon stopped");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thresholds_parse_overrides() {
        let t = Thresholds::new(&["cpu=95".into(), "mem=88".into(), "bogus=1".into()]);
        assert_eq!(t.get("cpu"), 95.0);
        assert_eq!(t.get("mem"), 88.0);
        assert_eq!(t.get("disk"), 90.0); // default preserved
    }

    #[test]
    fn breach_fires_then_cooldowns_then_rearms() {
        let mut armed = true;
        // First breach → fires.
        let r1 = check(Some(95.0), 90.0, &mut armed, "CPU", "%");
        assert!(r1.is_some(), "first breach fires");
        // Still breached → silent.
        let r2 = check(Some(96.0), 90.0, &mut armed, "CPU", "%");
        assert!(r2.is_none(), "cooldown active");
        // Recovers below 85 → re-armed.
        let r3 = check(Some(80.0), 90.0, &mut armed, "CPU", "%");
        assert!(r3.is_none(), "recovery is silent");
        // Breaches again → fires.
        let r4 = check(Some(91.0), 90.0, &mut armed, "CPU", "%");
        assert!(r4.is_some(), "re-armed fires again");
    }

    #[test]
    fn recovery_within_hysteresis_does_not_rearm() {
        let mut armed = false;
        // Recovered to 87 (above 85 rearm line) → stays disarmed.
        check(Some(87.0), 90.0, &mut armed, "CPU", "%");
        assert!(!armed);
        // Recovers to 84 → re-armed.
        check(Some(84.0), 90.0, &mut armed, "CPU", "%");
        assert!(armed);
    }

    #[test]
    fn unknown_metric_gets_infinity() {
        let t = Thresholds::new(&[]);
        assert_eq!(t.get("gpu"), f64::INFINITY);
    }
}
