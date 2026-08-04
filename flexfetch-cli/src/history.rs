//! Phase 5.5 — SQLite metrics history (feature `history`, rusqlite bundled).
//!
//! Records periodic cpu/mem/disk/temp snapshots into `history.db` in the cache
//! dir and renders them back as an ASCII sparkline (`--history-graph`) or CSV
//! (`--history-export`). Rows older than 90 days are pruned on every open.
//!
//! Table: `snapshots(id INTEGER PK, ts INTEGER unix-secs, cpu REAL, mem REAL,
//! disk REAL, temp REAL)` — NULLs for metrics the collector couldn't read.

use crate::monitor::{self, Health};
use flexfetch_core::Context;
use rusqlite::{params, Connection};

pub const PRUNE_DAYS: i64 = 90;

/// Metrics available to `--history-graph`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    Cpu,
    Memory,
    Disk,
    Temp,
}

impl Metric {
    pub fn column(self) -> &'static str {
        match self {
            Metric::Cpu => "cpu",
            Metric::Memory => "mem",
            Metric::Disk => "disk",
            Metric::Temp => "temp",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Metric::Cpu => "CPU",
            Metric::Memory => "Memory",
            Metric::Disk => "Disk",
            Metric::Temp => "Temp",
        }
    }
    pub fn unit(self) -> &'static str {
        match self {
            Metric::Temp => "°C",
            _ => "%",
        }
    }
}

fn db_path(cache_dir: &std::path::Path) -> std::path::PathBuf {
    cache_dir.join("history.db")
}

/// Open (creating if needed) the history DB, then prune rows older than
/// PRUNE_DAYS so the file can't grow unbounded.
pub fn open(cache_dir: &std::path::Path) -> rusqlite::Result<Connection> {
    if let Some(parent) = db_path(cache_dir).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(db_path(cache_dir))?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS snapshots (
            id  INTEGER PRIMARY KEY,
            ts  INTEGER NOT NULL,
            cpu REAL,
            mem REAL,
            disk REAL,
            temp REAL
        );
        CREATE INDEX IF NOT EXISTS idx_snapshots_ts ON snapshots(ts);
        ",
    )?;
    prune(&conn)?;
    Ok(conn)
}

/// Delete snapshots older than 90 days.
pub fn prune(conn: &Connection) -> rusqlite::Result<usize> {
    let cutoff = now() - PRUNE_DAYS * 86_400;
    conn.execute("DELETE FROM snapshots WHERE ts < ?1", params![cutoff])
}

/// Insert one snapshot. Skips entirely when no metric could be read.
pub fn record(conn: &Connection, h: &Health) -> rusqlite::Result<()> {
    if !h.any() {
        return Ok(());
    }
    conn.execute(
        "INSERT INTO snapshots (ts, cpu, mem, disk, temp) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            now(),
            h.cpu_pct,
            h.mem_pct.map(f64::from),
            h.disk_pct.map(f64::from),
            h.temp_c,
        ],
    )?;
    Ok(())
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Row values for one metric over the last `hours`, oldest first.
fn series(conn: &Connection, metric: Metric, hours: i64) -> rusqlite::Result<Vec<f64>> {
    let since = now() - hours * 3600;
    let mut stmt = conn.prepare(&format!(
        "SELECT {} FROM snapshots WHERE ts >= ?1 AND {} IS NOT NULL ORDER BY ts",
        metric.column(),
        metric.column()
    ))?;
    let rows = stmt.query_map(params![since], |row| row.get::<_, f64>(0))?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Render an ASCII sparkline (braille-free, block chars) with a min/avg/max
/// legend. Downsampled to ≤ `max_bars` columns by bucketing.
pub fn sparkline(values: &[f64], max_bars: usize, unit: &str) -> String {
    if values.is_empty() {
        return "no history in range (use --history / --daemon to record)".to_string();
    }
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg = values.iter().sum::<f64>() / values.len() as f64;

    // Bucket into ≤ max_bars columns (mean per bucket) so long ranges fit.
    let bars = values.len().min(max_bars.max(1));
    let bucket = values.len() / bars;
    let bucket = bucket.max(1);
    let mut line = String::with_capacity(bars + 16);
    let range = (max - min).max(1e-9);
    let mut i = 0;
    while i < values.len() {
        let end = (i + bucket).min(values.len());
        let mean = values[i..end].iter().sum::<f64>() / (end - i) as f64;
        let idx = (((mean - min) / range) * 7.0).round() as usize;
        line.push("▁▂▃▄▅▆▇█".chars().nth(idx.min(7)).unwrap());
        i = end;
    }

    format!(
        "{line}\n  min {min:.1}{unit}   avg {avg:.1}{unit}   max {max:.1}{unit}   ({} samples)",
        values.len()
    )
}

/// `--history-graph`: load the metric series over `hours` and print the sparkline.
pub fn graph(conn: &Connection, metric: Metric, hours: i64) -> rusqlite::Result<()> {
    let values = series(conn, metric, hours)?;
    println!("{} — last {hours}h", metric.label());
    println!("{}", sparkline(&values, 80, metric.unit()));
    Ok(())
}

/// `--history-export <path>`: dump the whole snapshots table as CSV.
pub fn export_csv(conn: &Connection, path: &std::path::Path) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT ts, cpu, mem, disk, temp FROM snapshots ORDER BY ts")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, Option<f64>>(1)?,
            row.get::<_, Option<f64>>(2)?,
            row.get::<_, Option<f64>>(3)?,
            row.get::<_, Option<f64>>(4)?,
        ))
    })?;
    let mut out = String::from("ts,cpu,mem,disk,temp\n");
    for r in rows {
        let (ts, cpu, mem, disk, temp) = r?;
        out.push_str(&format!(
            "{ts},{},{},{},{}\n",
            fmt(cpu),
            fmt(mem),
            fmt(disk),
            fmt(temp)
        ));
    }
    let row_count = out.lines().count().saturating_sub(1);
    std::fs::write(path, out).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    println!("wrote {row_count} snapshot rows to {path:?}");
    Ok(())
}

fn fmt(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.1}"),
        None => String::new(),
    }
}

/// Standalone `--history` record loop: sample every `interval` seconds until
/// Ctrl+C. Used directly, and by `--watch` (per tick) and `--daemon` (per poll).
pub fn record_loop(ctx: Context, interval_secs: u64) -> rusqlite::Result<()> {
    let conn = open(&ctx.cache_dir)?;
    eprintln!(
        "[flexfetch] recording history every {interval_secs}s → {} (Ctrl+C to stop)",
        db_path(&ctx.cache_dir).display()
    );
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    let _ = ctrlc::set_handler(move || r.store(false, std::sync::atomic::Ordering::SeqCst));
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        let h = monitor::sample_health(&ctx);
        record(&conn, &h)?;
        for _ in 0..interval_secs.max(1) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if !running.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conn_in_memory() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE snapshots (
                id INTEGER PRIMARY KEY,
                ts INTEGER NOT NULL,
                cpu REAL, mem REAL, disk REAL, temp REAL
            );",
        )
        .unwrap();
        c
    }

    #[test]
    fn record_and_series() {
        let c = conn_in_memory();
        let h = Health {
            cpu_pct: Some(37.1),
            mem_pct: Some(54),
            disk_pct: Some(83),
            temp_c: Some(42.0),
        };
        record(&c, &h).unwrap();
        let rows = series(&c, Metric::Cpu, 24).unwrap();
        assert_eq!(rows.len(), 1);
        assert!((rows[0] - 37.1).abs() < 1e-9);
        let mem = series(&c, Metric::Memory, 24).unwrap();
        assert!((mem[0] - 54.0).abs() < 1e-9);
    }

    #[test]
    fn record_skips_empty() {
        let c = conn_in_memory();
        record(&c, &Health::default()).unwrap();
        let rows = series(&c, Metric::Cpu, 24).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn sparkline_bounds() {
        let flat = sparkline(&[50.0; 10], 10, "%");
        assert!(flat.contains("avg 50.0%"));
        let empty = sparkline(&[], 10, "%");
        assert!(empty.contains("no history"));
    }

    #[test]
    fn prune_removes_old_rows() {
        let c = conn_in_memory();
        let old = now() - (PRUNE_DAYS + 10) * 86_400;
        c.execute(
            "INSERT INTO snapshots (ts, cpu) VALUES (?1, 10.0)",
            params![old],
        )
        .unwrap();
        let fresh = Health {
            cpu_pct: Some(5.0),
            ..Health::default()
        };
        record(&c, &fresh).unwrap();
        prune(&c).unwrap();
        let rows = series(&c, Metric::Cpu, 24 * 365).unwrap();
        assert_eq!(rows.len(), 1, "only the fresh row survives the prune");
        assert!((rows[0] - 5.0).abs() < 1e-9);
    }
}
