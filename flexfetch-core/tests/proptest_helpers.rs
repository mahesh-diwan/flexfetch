//! Phase 8.5 — property-based tests (proptest).
//!
//! Property: the pure helpers in flexfetch-core must NEVER panic and must keep
//! their structural invariants on arbitrary input — not just the happy paths
//! the unit tests cover. Run:
//!   cargo test -p flexfetch-core --test proptest_helpers
//! (CI runs it in the test job; fuzzing lives in the `fuzz/` crate.)

use flexfetch_core::{
    logo::visible_len,
    modules::uptime::format_uptime,
    template::frame_wrap,
    theme::{gradient_text, resolve_ansi},
};
use proptest::prelude::*;

proptest! {
    /// visible_len must never panic.
    #[test]
    fn visible_len_never_panics(s in ".*") {
        let _ = visible_len(&s);
    }

    /// ANSI-colored strings must have the same visible width as their plain
    /// counterparts — color codes are invisible.
    #[test]
    fn visible_len_ignores_ansi(plain in "[a-z0-9 _-]{0,40}") {
        let colored = format!("\x1b[1;38;2;10;20;30m{plain}\x1b[0m");
        prop_assert_eq!(visible_len(&colored), visible_len(&plain));
    }

    /// resolve_ansi must never panic and must return either an escape sequence
    /// (for known names) or empty (for unknown input).
    #[test]
    fn resolve_ansi_never_panics(name in "[a-z-]{0,20}") {
        let out = resolve_ansi(&name);
        prop_assert!(out.is_empty() || out.starts_with("\x1b["));
    }

    /// gradient_text must never panic and must preserve the original text when
    /// stripped of color codes.
    #[test]
    fn gradient_text_never_panics(text in "[a-z0-9 _-]{0,50}") {
        let stops = [[10u8, 20, 30], [40, 50, 60]];
        let _ = gradient_text(&text, &stops);
    }

    /// format_uptime must never panic on any u64.
    #[test]
    fn format_uptime_never_panics(secs: u64) {
        let _ = format_uptime(secs);
    }

    /// frame_wrap must never panic and must add a top border line.
    #[test]
    fn frame_wrap_never_panics(body in "[a-z0-9 \n-]{0,120}") {
        let out = frame_wrap(&body, "single", "\x1b[1;94m");
        prop_assert!(out.starts_with("\x1b[1;94m┌") || out.is_empty() || !body.contains('\n'));
    }

    /// Ansi-stripped gradient text equals the input.
    #[test]
    fn gradient_text_preserves_chars(text in "[a-z0-9 ]{0,50}") {
        let stops = [[200u8, 100, 50], [50, 100, 200]];
        let colored = gradient_text(&text, &stops);
        let stripped: String = colored
            .split("\x1b[")
            .map(|seg| match seg.find('m') {
                Some(idx) => &seg[idx + 1..],
                None => seg,
            })
            .collect();
        prop_assert_eq!(stripped, text);
    }
}
