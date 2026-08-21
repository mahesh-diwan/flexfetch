//! `--serve`: NDJSON daemon mode. Emits one compact JSON object per tick
//! (default 2 s) on stdout — panels and scripts consume the stream directly
//! (`flexfetch --serve | jq -c`). Static modules are served from the snapshot
//! cache, so ticks only re-collect dynamic values. Stops cleanly when the
//! `running` flag flips false (SIGINT handled by the caller).

use flexfetch_core::ModuleRegistry;
use std::io::Write;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc};
use std::time::Duration;

pub fn run(
    modules: &[String],
    ctx: &Arc<flexfetch_core::Context>,
    registry: &'static ModuleRegistry,
    template_content: &str,
    interval: u64,
    running: &Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cache = std::collections::HashMap::new();
    let mut stdout = std::io::stdout();
    while running.load(Ordering::SeqCst) {
        let info = registry.run_selected_cached(modules, ctx, template_content, &mut cache);
        // A closed consumer (head, jq quit, panel restart) surfaces as EPIPE —
        // that's a normal shutdown for a daemon, not an error.
        if writeln!(stdout, "{}", serve_line(&info)).is_err() {
            break;
        }
        stdout.flush().ok();
        if running.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_secs(interval.max(1)));
        }
    }
    Ok(())
}

/// One compact single-line JSON serialization of the collected info.
fn serve_line(info: &flexfetch_core::SystemInfo) -> String {
    serde_json::to_string(&info.to_json()).unwrap_or_else(|_| "{}".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_line_is_compact_single_line_json() {
        let mut info = flexfetch_core::SystemInfo::new();
        info.add("os", flexfetch_core::InfoValue::scalar("Linux"));
        let line = serve_line(&info);
        assert!(!line.contains('\n'), "NDJSON must be single-line");
        assert!(line.starts_with('{'));
        assert!(line.contains("\"os\""));
    }
}
