use crate::{Context, InfoValue, Module, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Current date and time, e.g. `2026-08-16 14:32`.
///
/// Deliberately uses only `std::time` — the naive local conversion is exact
/// enough for a fetch tool's clock line and keeps the binary free of chrono.
pub struct DatetimeModule;

impl Module for DatetimeModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Ok(InfoValue::Scalar(format_datetime(secs)))
    }
}

/// Format epoch seconds as a local `YYYY-MM-DD HH:MM` string without chrono.
/// Uses a 400-year leap-year cycle so the math is exact for any date.
pub fn format_datetime(secs: u64) -> String {
    let days = secs / 86400;
    let rem = secs % 86400;
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;

    let (y, m, d) = civil_from_days(days as i64);
    format!("{y:04}-{m:02}-{d:02} {hour:02}:{min:02}")
}

/// Howard Hinnant's civil-from-days algorithm (public domain) — exact, no deps.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_known_epochs() {
        // 1970-01-01 00:00 UTC
        assert_eq!(format_datetime(0), "1970-01-01 00:00");
        // 2000-03-01 12:30 UTC (leap year)
        assert_eq!(format_datetime(951_913_800), "2000-03-01 12:30");
        // 2026-08-16 11:29 UTC
        assert_eq!(format_datetime(1_786_879_740), "2026-08-16 11:29");
    }

    #[test]
    fn handles_leap_days() {
        // 2024-02-29 08:00 UTC — leap day must not roll into March.
        assert_eq!(format_datetime(1_709_193_600), "2024-02-29 08:00");
        // 2100 is NOT a leap year (century rule); Feb has 28 days.
        assert_eq!(format_datetime(4_107_456_000), "2100-02-28 00:00");
    }
}
