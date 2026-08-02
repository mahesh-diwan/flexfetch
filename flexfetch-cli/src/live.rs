//! Live dashboard (`--live`): real-time system monitor built on ratatui + crossterm.
//!
//! Reuses existing collectors where possible (memory via `run_individual`) and adds
//! lightweight `/proc` + `/sys` samplers for the real-time data that the one-shot
//! modules don't expose (CPU %, per-process CPU, network rates).

use flexfetch_core::{Config, Context, InfoValue, ModuleRegistry};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Cell, Gauge, Paragraph, Row, Sparkline, Table, Wrap},
    Frame,
};
use std::collections::{HashMap, VecDeque};
use std::io::IsTerminal;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// How many history samples each sparkline keeps.
const HISTORY: usize = 60;
/// Tick interval between samples.
const TICK: Duration = Duration::from_millis(1000);

pub fn run(ctx: Context, config_path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    if !std::io::stdout().is_terminal() {
        eprintln!("--live requires a terminal (stdout is not a tty)");
        std::process::exit(1);
    }

    // Owned context so we can rebuild it when the config file changes
    // (mtime-based hot-reload, no external watcher dependency).
    let mut ctx = ctx;
    let mut last_mtime = config_path.as_deref().and_then(file_mtime);

    let mut terminal = ratatui::init();
    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut app = App::new(&ctx);
        let mut last_tick = Instant::now();

        loop {
            // Config hot-reload: rebuild ctx (custom modules) when the file changes.
            if let Some(path) = &config_path {
                let now = file_mtime(path);
                if now != last_mtime {
                    last_mtime = now;
                    let custom = Config::load(Some(path))
                        .map(|c| c.custom)
                        .unwrap_or_default();
                    ctx = Context::new(
                        ctx.config_dir.clone(),
                        ctx.cache_dir.clone(),
                        ctx.debug,
                        custom,
                    );
                    app.notice = Some("config reloaded".to_string());
                }
            }

            if last_tick.elapsed() >= TICK {
                app.sample(&ctx);
                last_tick = Instant::now();
            }

            terminal.draw(|frame| app.draw(frame))?;

            if crossterm::event::poll(Duration::from_millis(50))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        match key.code {
                            crossterm::event::KeyCode::Char('q')
                            | crossterm::event::KeyCode::Esc => return Ok(()),
                            crossterm::event::KeyCode::Char(' ') => {
                                app.sample(&ctx);
                                last_tick = Instant::now();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    })();

    // Always restore the terminal, even if the loop above exited via `?`
    // (event/redraw error), so the user is never stranded in raw mode.
    ratatui::restore();
    result
}

// ---------------------------------------------------------------------------
// App state & sampling
// ---------------------------------------------------------------------------

struct ProcInfo {
    pid: i32,
    name: String,
    cpu_pct: f64,
    mem_mb: f64,
}

#[cfg_attr(not(target_os = "linux"), allow(dead_code, unused_variables))]
struct App {
    /// CPU % from the last sample.
    cpu_pct: f64,
    cpu_history: VecDeque<u64>,
    /// (total, idle) ticks from `/proc/stat` at the last sample.
    stat_prev: Option<(u64, u64)>,
    /// Total ticks when `proc_prev` was last refreshed (for process CPU% deltas).
    proc_total_prev: u64,
    /// pid -> (utime + stime) ticks at the last sample.
    proc_prev: HashMap<i32, u64>,
    processes: Vec<ProcInfo>,
    /// Number of logical cores.
    cores: u64,
    mem_pct: u8,
    mem_used: String,
    mem_total: String,
    mem_history: VecDeque<u64>,
    /// iface -> (rx_bytes, tx_bytes) at the last sample.
    net_prev: HashMap<String, (u64, u64)>,
    net_rates: Vec<(String, f64, f64)>,
    /// When the last sample was taken (for rate computations).
    last_sample: Instant,
    /// Transient status message (e.g. "config reloaded") shown in the header.
    notice: Option<String>,
}

impl App {
    fn new(ctx: &Context) -> Self {
        let mut app = App {
            cpu_pct: 0.0,
            cpu_history: VecDeque::with_capacity(HISTORY),
            stat_prev: None,
            proc_total_prev: 0,
            proc_prev: HashMap::new(),
            processes: Vec::new(),
            cores: logical_cores(),
            mem_pct: 0,
            mem_used: String::new(),
            mem_total: String::new(),
            mem_history: VecDeque::with_capacity(HISTORY),
            net_prev: HashMap::new(),
            net_rates: Vec::new(),
            last_sample: Instant::now(),
            notice: None,
        };
        app.sample(ctx);
        app
    }

    fn sample(&mut self, ctx: &Context) {
        // Actual elapsed time since the last sample (space-bar refresh and slow
        // terminal draws make this differ from TICK — rates must use the real span).
        let elapsed = self.last_sample.elapsed().as_secs_f64().max(0.001);
        self.last_sample = Instant::now();

        // --- CPU % (delta of /proc/stat totals, no sleep needed) ---
        if let Some((total, idle)) = read_stat() {
            if let Some((pt, pi)) = self.stat_prev {
                let dt = total.saturating_sub(pt);
                let di = idle.saturating_sub(pi);
                if dt > 0 {
                    self.cpu_pct = (dt - di) as f64 / dt as f64 * 100.0;
                }
            }
            self.stat_prev = Some((total, idle));
        }
        push(&mut self.cpu_history, self.cpu_pct as u64);

        // --- Memory: reuse the existing collector via the registry ---
        if let Some((pct, used, total)) = sample_memory(ctx) {
            self.mem_pct = pct;
            self.mem_used = used;
            self.mem_total = total;
        }
        push(&mut self.mem_history, self.mem_pct as u64);

        // --- Top processes (Linux /proc) ---
        self.sample_processes();

        // --- Network rates (Linux /sys) ---
        self.sample_network(elapsed);
    }

    fn sample_processes(&mut self) {
        #[cfg(target_os = "linux")]
        {
            let stat_total = self.stat_prev.map(|(t, _)| t).unwrap_or(0);
            let mut cur: HashMap<i32, u64> = HashMap::new();
            let mut procs: Vec<ProcInfo> = Vec::new();

            if let Ok(entries) = std::fs::read_dir("/proc") {
                for entry in entries.flatten() {
                    let pid: i32 = match entry.file_name().to_str().and_then(|s| s.parse().ok()) {
                        Some(pid) => pid,
                        None => continue,
                    };
                    let base = entry.path();
                    let name = std::fs::read_to_string(base.join("comm"))
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();
                    let stat = std::fs::read_to_string(base.join("stat")).unwrap_or_default();
                    // Format: "pid (comm) state ppid ... utime stime ...". comm may
                    // contain spaces/parens, so split after the LAST ')'.
                    let Some((_, rest)) = stat.rsplit_once(')') else {
                        continue;
                    };
                    let fields: Vec<&str> = rest.split_whitespace().collect();
                    if fields.len() <= 12 {
                        continue;
                    }
                    let utime: u64 = fields[11].parse().unwrap_or(0);
                    let stime: u64 = fields[12].parse().unwrap_or(0);
                    let ticks = utime + stime;
                    cur.insert(pid, ticks);

                    let rss_pages: u64 = std::fs::read_to_string(base.join("statm"))
                        .ok()
                        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
                        .unwrap_or(0);

                    if let Some(&prev_ticks) = self.proc_prev.get(&pid) {
                        let dproc = ticks.saturating_sub(prev_ticks);
                        let dtotal = stat_total.saturating_sub(self.proc_total_prev);
                        // First sample has no baseline for processes yet; skip it.
                        if dtotal > 0 {
                            let cpu_pct = dproc as f64 / dtotal as f64 * self.cores as f64 * 100.0;
                            let mem_mb = rss_pages as f64 * 4096.0 / (1024.0 * 1024.0);
                            if name.is_empty() || name == "kthreadd" {
                                continue;
                            }
                            procs.push(ProcInfo {
                                pid,
                                name,
                                cpu_pct,
                                mem_mb,
                            });
                        }
                    }
                }
            }

            procs.sort_by(|a, b| {
                b.cpu_pct
                    .partial_cmp(&a.cpu_pct)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            procs.truncate(10);
            self.proc_prev = cur;
            self.proc_total_prev = stat_total;
            self.processes = procs;
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.processes = Vec::new();
        }
    }

    #[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
    fn sample_network(&mut self, elapsed: f64) {
        #[cfg(target_os = "linux")]
        {
            let mut rates = Vec::new();
            if let Ok(entries) = std::fs::read_dir("/sys/class/net/") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name == "lo"
                        || name.starts_with("docker")
                        || name.starts_with("br-")
                        || name.starts_with("veth")
                        || name.starts_with("virbr")
                    {
                        continue;
                    }
                    let base = entry.path().join("statistics");
                    let rx = read_u64(base.join("rx_bytes"));
                    let tx = read_u64(base.join("tx_bytes"));
                    if let Some(&(prx, ptx)) = self.net_prev.get(&name) {
                        rates.push((
                            name.clone(),
                            rx.saturating_sub(prx) as f64 / elapsed,
                            tx.saturating_sub(ptx) as f64 / elapsed,
                        ));
                    }
                    self.net_prev.insert(name, (rx, tx));
                }
            }
            rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            self.net_rates = rates;
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.net_rates = Vec::new();
        }
    }

    // -----------------------------------------------------------------------
    // Rendering
    // -----------------------------------------------------------------------

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        // Header bar
        let mut spans = vec![Span::styled(
            " flexfetch --live ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )];
        if let Some(notice) = &self.notice {
            spans.push(Span::styled(
                format!("  {notice}  "),
                Style::default().fg(Color::Yellow),
            ));
        }
        spans.push(Span::styled(
            "  [q] quit  [space] refresh  ",
            Style::default().fg(Color::DarkGray),
        ));
        let header = Line::from(spans);
        frame.render_widget(Paragraph::new(header), chunks[0]);

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
            .split(chunks[1]);

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Min(4),
            ])
            .split(main[0]);

        self.draw_cpu(frame, left[0]);
        self.draw_mem(frame, left[1]);
        self.draw_net(frame, left[2]);
        self.draw_procs(frame, main[1]);
    }

    fn draw_cpu(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(Line::from(" CPU "));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(gauge_color(self.cpu_pct)))
            .ratio(self.cpu_pct.clamp(0.0, 100.0) / 100.0)
            .label(format!("{:>5.1}%", self.cpu_pct));
        frame.render_widget(gauge, Rect::new(inner.x, inner.y, inner.width, 1));

        let data: Vec<u64> = self.cpu_history.iter().copied().collect();
        let spark = Sparkline::default()
            .data(&data)
            .max(100)
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(
            spark,
            Rect::new(
                inner.x,
                inner.y.saturating_add(1),
                inner.width,
                inner.height.saturating_sub(1),
            ),
        );
    }

    fn draw_mem(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(Line::from(" Memory "));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(gauge_color(self.mem_pct as f64)))
            .ratio(self.mem_pct as f64 / 100.0)
            .label(format!(
                "{:>4}%  {} / {}",
                self.mem_pct, self.mem_used, self.mem_total
            ));
        frame.render_widget(gauge, Rect::new(inner.x, inner.y, inner.width, 1));

        let data: Vec<u64> = self.mem_history.iter().copied().collect();
        let spark = Sparkline::default()
            .data(&data)
            .max(100)
            .style(Style::default().fg(Color::Green));
        frame.render_widget(
            spark,
            Rect::new(
                inner.x,
                inner.y.saturating_add(1),
                inner.width,
                inner.height.saturating_sub(1),
            ),
        );
    }

    fn draw_net(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(Line::from(" Network "));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines: Vec<Line> = self
            .net_rates
            .iter()
            .map(|(name, rx, tx)| {
                Line::from(vec![
                    Span::styled(
                        format!("{name:<12}"),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("▼ {}  ", fmt_rate(*rx)),
                        Style::default().fg(Color::Green),
                    ),
                    Span::styled(
                        format!("▲ {}", fmt_rate(*tx)),
                        Style::default().fg(Color::Yellow),
                    ),
                ])
            })
            .collect();
        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "no interfaces",
                Style::default().fg(Color::DarkGray),
            )));
        }
        let para = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(para, inner);
    }

    fn draw_procs(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(Line::from(" Top processes "));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let rows: Vec<Row> = self
            .processes
            .iter()
            .map(|p| {
                Row::new(vec![
                    Cell::from(p.pid.to_string()),
                    Cell::from(format!("{:>6.1}%", p.cpu_pct)),
                    Cell::from(format!("{:>6.0} MB", p.mem_mb)),
                    Cell::from(p.name.clone()),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(7),
            Constraint::Length(9),
            Constraint::Length(10),
            Constraint::Min(10),
        ];
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec![
                    Cell::from("PID"),
                    Cell::from("CPU"),
                    Cell::from("MEM"),
                    Cell::from("COMMAND"),
                ])
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .block(Block::default());
        frame.render_widget(table, inner);
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn push(deque: &mut VecDeque<u64>, value: u64) {
    if deque.len() == HISTORY {
        deque.pop_front();
    }
    deque.push_back(value);
}

fn file_mtime(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

/// Reads (total, idle) ticks from the first line of `/proc/stat`.
fn read_stat() -> Option<(u64, u64)> {
    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/stat").ok()?;
        let line = content.lines().next()?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let total: u64 = parts
            .iter()
            .skip(1)
            .filter_map(|v| v.parse::<u64>().ok())
            .sum();
        let idle: u64 = parts.get(4).and_then(|v| v.parse().ok()).unwrap_or(0);
        Some((total, idle))
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

fn logical_cores() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/stat") {
            let cores = content
                .lines()
                .filter(|l| l.starts_with("cpu"))
                .count()
                .saturating_sub(1) as u64; // minus the aggregate "cpu" line
            if cores > 0 {
                return cores;
            }
        }
        std::thread::available_parallelism()
            .map(|n| n.get() as u64)
            .unwrap_or(1)
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::thread::available_parallelism()
            .map(|n| n.get() as u64)
            .unwrap_or(1)
    }
}

#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn read_u64(path: std::path::PathBuf) -> u64 {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

fn sample_memory(ctx: &Context) -> Option<(u8, String, String)> {
    match ModuleRegistry::get().run_individual("memory", ctx)? {
        InfoValue::Map(m) => {
            let pct = m.get("percent_int")?.parse().ok()?;
            let used = m.get("used")?.clone();
            let total = m.get("total")?.clone();
            Some((pct, used, total))
        }
        _ => None,
    }
}

fn gauge_color(pct: f64) -> Color {
    if pct >= 85.0 {
        Color::Red
    } else if pct >= 60.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}

fn fmt_rate(bps: f64) -> String {
    if bps >= 1024.0 * 1024.0 {
        format!("{:.1} MB/s", bps / (1024.0 * 1024.0))
    } else if bps >= 1024.0 {
        format!("{:.1} KB/s", bps / 1024.0)
    } else {
        format!("{:.0} B/s", bps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_rate() {
        assert_eq!(fmt_rate(0.0), "0 B/s");
        assert_eq!(fmt_rate(500.0), "500 B/s");
        assert_eq!(fmt_rate(2048.0), "2.0 KB/s");
        assert_eq!(fmt_rate(3.0 * 1024.0 * 1024.0), "3.0 MB/s");
    }

    #[test]
    fn test_gauge_color_thresholds() {
        assert_eq!(gauge_color(10.0), Color::Green);
        assert_eq!(gauge_color(60.0), Color::Yellow);
        assert_eq!(gauge_color(70.0), Color::Yellow);
        assert_eq!(gauge_color(85.0), Color::Red);
        assert_eq!(gauge_color(100.0), Color::Red);
    }

    #[test]
    fn test_push_keeps_history_bounded() {
        let mut h = VecDeque::new();
        for i in 0..(HISTORY + 10) {
            push(&mut h, i as u64);
        }
        assert_eq!(h.len(), HISTORY);
        assert_eq!(*h.front().unwrap(), 10);
        assert_eq!(*h.back().unwrap(), (HISTORY + 9) as u64);
    }
}
