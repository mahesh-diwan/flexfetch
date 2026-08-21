//! Live dashboard (`--live`): real-time system monitor built on ratatui + crossterm.
//!
//! Phase 4.2 — lock-free architecture: a dedicated **sampler thread** owns every
//! `/proc` prev-state (stat deltas, per-process ticks, net counters) and pushes
//! `SystemSnapshot` values through a `crossbeam::channel::bounded(1)` **overwrite
//! channel**. The renderer pulls only the newest snapshot each frame:
//!   - collection never blocks rendering (no mutex in the hot path),
//!   - a slow renderer never stalls the sampler — `try_send` on the bounded(1)
//!     channel never blocks, and the renderer drains with `try_recv` so it
//!     always displays the newest snapshot that got through,
//!   - config hot-reload happens on the sampler thread (it owns the Context).

use crossbeam_channel::{Receiver, Sender};
use flexfetch_core::{Config, Context, InfoValue, ModuleRegistry};
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame, Terminal,
};
use std::collections::{HashMap, VecDeque};
use std::io::IsTerminal;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// How many history samples each sparkline keeps.
const HISTORY: usize = 60;
/// Tick interval between samples.
const TICK: Duration = Duration::from_millis(1000);
/// Render budget: at 60 FPS a frame must finish in ~16 ms. If the renderer
/// exceeds it, the overwrite channel drops the stale sample automatically.
const FRAME_BUDGET: Duration = Duration::from_millis(16);

pub fn run(ctx: Context, config_path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    if !std::io::stdout().is_terminal() {
        eprintln!("--live requires a terminal (stdout is not a tty)");
        std::process::exit(1);
    }

    // Phase 4.2: bounded(1) overwrite channel — the sampler's try_send drops the
    // previous snapshot when the renderer hasn't consumed it yet (no blocking).
    let (tx, rx): (Sender<SystemSnapshot>, Receiver<SystemSnapshot>) =
        crossbeam_channel::bounded(1);

    // Cooperative stop: the sampler thread loops forever; flip this on quit.
    let stop = Arc::new(AtomicBool::new(false));
    let stop_sampler = Arc::clone(&stop);

    // Sampler thread: owns Context (for config hot-reload) + all prev-state.
    let sampler = std::thread::spawn(move || {
        sampler_loop(ctx, config_path, tx, stop_sampler);
    });

    let mut terminal = ratatui::init();
    let result = app_loop(&mut terminal, &rx, &stop);
    // Always restore the terminal, even if the loop exited via `?`
    // (event/redraw error), so the user is never stranded in raw mode.
    ratatui::restore();

    // Signal the sampler to exit and reap it (best effort; the process exits
    // right after anyway, but this keeps the thread from outliving the TUI).
    stop.store(true, Ordering::Relaxed);
    let _ = sampler.join();

    result
}

/// The render loop. Pulls the latest snapshot each frame, handles quit/space
/// keys, and keeps frames inside the frame budget.
fn app_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    rx: &Receiver<SystemSnapshot>,
    stop: &Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error>>
where
    <B as Backend>::Error: 'static,
{
    let mut app = App::new();

    // Pull the newest snapshot (overwrite channel: only the latest).
    fn drain_latest(rx: &Receiver<SystemSnapshot>, slot: &mut SystemSnapshot) {
        while let Ok(snap) = rx.try_recv() {
            *slot = snap;
        }
    }

    loop {
        drain_latest(rx, &mut app.snapshot);

        let frame_start = Instant::now();
        terminal.draw(|frame| app.draw(frame))?;
        let frame_time = frame_start.elapsed();
        // If the frame blew the budget, note it (the channel drops stale
        // samples on its own; this is just observability for the header).
        if frame_time > FRAME_BUDGET {
            app.slow_frames += 1;
        }

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => {
                            stop.store(true, Ordering::Relaxed);
                            return Ok(());
                        }
                        crossterm::event::KeyCode::Char(' ') => {
                            // Force a fresh sample: re-pull the latest.
                            drain_latest(rx, &mut app.snapshot);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Sampler thread
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct ProcInfo {
    // Only populated/read on Linux (sample_processes is Linux-only); Windows
    // builds never construct a ProcInfo, so silence the dead-code lint there.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    name: String,
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    cpu_pct: f64,
}

/// Immutable view handed to the renderer each tick. Owned (no borrows), so it
/// can cross the channel freely; the renderer never touches /proc itself.
struct SystemSnapshot {
    cpu_pct: f64,
    cpu_history: VecDeque<u64>,
    mem_pct: u8,
    mem_used: String,
    mem_total: String,
    mem_history: VecDeque<u64>,
    disk_pct: u8,
    processes: Vec<ProcInfo>,
    net_rates: Vec<(String, f64, f64)>,
    net_history: VecDeque<u64>,
    /// Transient status message (e.g. "config reloaded") shown in the header.
    notice: Option<String>,
}

impl Default for SystemSnapshot {
    fn default() -> Self {
        SystemSnapshot {
            cpu_pct: 0.0,
            cpu_history: VecDeque::with_capacity(HISTORY),
            mem_pct: 0,
            mem_used: String::new(),
            mem_total: String::new(),
            mem_history: VecDeque::with_capacity(HISTORY),
            disk_pct: 0,
            processes: Vec::new(),
            net_rates: Vec::new(),
            net_history: VecDeque::with_capacity(HISTORY),
            notice: None,
        }
    }
}

fn sampler_loop(
    mut ctx: Context,
    config_path: Option<PathBuf>,
    tx: Sender<SystemSnapshot>,
    stop: Arc<AtomicBool>,
) {
    // All prev-state lives here, on the sampler thread — the renderer never
    // races with it (no mutex in the hot path).
    let mut last_mtime = config_path.as_deref().and_then(file_mtime);
    let mut last_sample = Instant::now();
    let mut stat_prev: Option<(u64, u64)> = None;
    let mut proc_total_prev: u64 = 0;
    let mut proc_prev: HashMap<i32, u64> = HashMap::new();
    let mut net_prev: HashMap<String, (u64, u64)> = HashMap::new();
    let mut net_history: VecDeque<u64> = VecDeque::with_capacity(HISTORY);
    let mut notice: Option<String> = None;
    let cores = logical_cores();

    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }

        // Config hot-reload: rebuild ctx (custom modules) when the file changes.
        if let Some(path) = &config_path {
            let now = file_mtime(path);
            if now != last_mtime {
                last_mtime = now;
                let cfg =
                    Config::load(Some(path)).unwrap_or_else(|_| Config::default_for_testing());
                let custom = cfg.custom.clone();
                ctx = Context::new(
                    ctx.config_dir.clone(),
                    ctx.cache_dir.clone(),
                    ctx.debug,
                    custom,
                );
                ctx.set_cache_ttl(cfg.cache_ttl);
                notice = Some("config reloaded".to_string());
            }
        }

        // Actual elapsed time (first sample and slow renders make this differ
        // from TICK — rates must use the real span).
        let elapsed = last_sample.elapsed().as_secs_f64().max(0.001);
        last_sample = Instant::now();

        let mut snap = SystemSnapshot {
            notice: notice.clone(),
            ..Default::default()
        };

        // --- CPU % (delta of /proc/stat totals, no sleep needed) ---
        if let Some((total, idle)) = read_stat() {
            if let Some((pt, pi)) = stat_prev {
                let dt = total.saturating_sub(pt);
                let di = idle.saturating_sub(pi);
                if dt > 0 {
                    snap.cpu_pct = (dt - di) as f64 / dt as f64 * 100.0;
                }
            }
            stat_prev = Some((total, idle));
        }
        push(&mut snap.cpu_history, snap.cpu_pct as u64);

        // --- Memory: reuse the existing collector via the registry ---
        if let Some((pct, used, total)) = sample_memory(&ctx) {
            snap.mem_pct = pct;
            snap.mem_used = used;
            snap.mem_total = total;
        }
        push(&mut snap.mem_history, snap.mem_pct as u64);

        // --- Disk usage ---
        if let Some(pct) = sample_disk(&ctx) {
            snap.disk_pct = pct;
        }

        // --- Top processes (Linux /proc) ---
        sample_processes(
            &mut snap,
            &mut proc_prev,
            &mut proc_total_prev,
            stat_prev,
            cores,
        );

        // --- Network rates (Linux /sys) ---
        sample_network(&mut snap, &mut net_prev, elapsed);
        let total_rx: f64 = snap.net_rates.iter().map(|(_, rx, _)| rx).sum();
        // History lives on the sampler thread (like net_prev) — the snapshot
        // is rebuilt from scratch each tick, so a per-snapshot deque would
        // never accumulate more than one sample.
        push(&mut net_history, total_rx as u64);
        snap.net_history = net_history.clone();

        // Non-blocking push: bounded(1) try_send never blocks the sampler. If
        // the renderer is slow, the newest sample is skipped (crossbeam never
        // overwrites) — no backpressure, no stutter. The renderer drains with
        // `while let Ok(snap) = rx.try_recv()` to always show the latest.
        let _ = tx.try_send(snap);

        // Sleep the remainder of the tick (cooperative stop check on wake).
        std::thread::sleep(TICK);
    }
}

#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
fn sample_processes(
    snap: &mut SystemSnapshot,
    proc_prev: &mut HashMap<i32, u64>,
    proc_total_prev: &mut u64,
    stat_prev: Option<(u64, u64)>,
    cores: u64,
) {
    #[cfg(target_os = "linux")]
    {
        let stat_total = stat_prev.map(|(t, _)| t).unwrap_or(0);
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
                let short_name = name.rsplit('/').next().unwrap_or(&name).to_string();
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

                if let Some(&prev_ticks) = proc_prev.get(&pid) {
                    let dproc = ticks.saturating_sub(prev_ticks);
                    let dtotal = stat_total.saturating_sub(*proc_total_prev);
                    // First sample has no baseline for processes yet; skip it.
                    if dtotal > 0 {
                        let cpu_pct = dproc as f64 / dtotal as f64 * cores as f64 * 100.0;
                        if name.is_empty() || name == "kthreadd" {
                            continue;
                        }
                        procs.push(ProcInfo {
                            name: short_name.clone(),
                            cpu_pct,
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
        *proc_prev = cur;
        *proc_total_prev = stat_total;
        snap.processes = procs;
    }
    #[cfg(not(target_os = "linux"))]
    {
        snap.processes = Vec::new();
    }
}

#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
fn sample_network(
    snap: &mut SystemSnapshot,
    net_prev: &mut HashMap<String, (u64, u64)>,
    elapsed: f64,
) {
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
                if let Some(&(prx, ptx)) = net_prev.get(&name) {
                    rates.push((
                        name.clone(),
                        rx.saturating_sub(prx) as f64 / elapsed,
                        tx.saturating_sub(ptx) as f64 / elapsed,
                    ));
                }
                net_prev.insert(name, (rx, tx));
            }
        }
        rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        snap.net_rates = rates;
    }
    #[cfg(not(target_os = "linux"))]
    {
        snap.net_rates = Vec::new();
    }
}

// ---------------------------------------------------------------------------
// Renderer (main thread)
// ---------------------------------------------------------------------------

struct App {
    /// Latest snapshot from the sampler thread.
    snapshot: SystemSnapshot,
    /// Count of frames that exceeded the 16 ms budget (overwrite channel drops
    /// stale samples on its own; this is observability for the header).
    slow_frames: u64,
}

impl App {
    fn new() -> Self {
        App {
            snapshot: SystemSnapshot::default(),
            slow_frames: 0,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let snap = &self.snapshot;
        let area = frame.area();

        // Window chrome: traffic-light dots + title on the left, tick + quit
        // hint on the right (mirrors the site's live-dash mockup).
        let dots = Line::from(vec![
            Span::styled(" ● ", Style::default().fg(Color::Red)),
            Span::styled("● ", Style::default().fg(Color::Yellow)),
            Span::styled("● ", Style::default().fg(Color::Green)),
            Span::styled(
                "flexfetch --live",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        let mut right = vec![Span::styled("1s ", Style::default().fg(Color::DarkGray))];
        if let Some(notice) = &snap.notice {
            right.push(Span::styled(
                format!("· {notice} "),
                Style::default().fg(Color::Yellow),
            ));
        }
        if self.slow_frames > 0 {
            right.push(Span::styled(
                format!("· {} slow ", self.slow_frames),
                Style::default().fg(Color::DarkGray),
            ));
        }
        right.push(Span::styled(
            "[q] quit",
            Style::default().fg(Color::DarkGray),
        ));
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_top(dots.left_aligned())
            .title_top(Line::from(right).right_aligned());
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Inner layout: gauges / processes / network
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // three ring gauges
                Constraint::Length(1), // process header
                Constraint::Length(3), // top processes
                Constraint::Min(3),    // network sparkline + rates
            ])
            .split(inner);

        // --- Ring gauges row: CPU / Memory / Disk ---
        let gauge_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(chunks[0]);
        let rings = [
            ("cpu", snap.cpu_pct, gauge_color(snap.cpu_pct)),
            (
                "memory",
                snap.mem_pct as f64,
                gauge_color(snap.mem_pct as f64),
            ),
            (
                "disk",
                snap.disk_pct as f64,
                gauge_color(snap.disk_pct as f64),
            ),
        ];
        for (i, (label, pct, color)) in rings.iter().enumerate() {
            let lines = ring_lines(label, *pct, *color);
            let para = Paragraph::new(lines).alignment(Alignment::Center);
            frame.render_widget(para, gauge_row[i]);
        }

        // --- Process header ---
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                "processes",
                Style::default().fg(Color::DarkGray),
            )])),
            chunks[1],
        );

        // --- Top processes: name + bar + pct ---
        let proc_lines: Vec<Line> = snap
            .processes
            .iter()
            .take(3)
            .map(|p| {
                let width = chunks[2].width.saturating_sub(22).max(4) as usize;
                let filled = ((p.cpu_pct.clamp(0.0, 100.0) / 100.0) * width as f64) as usize;
                let bar: String = "█".repeat(filled) + &"░".repeat(width.saturating_sub(filled));
                Line::from(vec![
                    Span::styled(
                        format!("{:<16}", truncate(&p.name, 16)),
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(bar, Style::default().fg(gauge_color(p.cpu_pct))),
                    Span::styled(
                        format!(" {:>4.0}%", p.cpu_pct),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            })
            .collect();
        let empty: Vec<Line> = (0..3usize.saturating_sub(proc_lines.len()))
            .map(|_| Line::from(""))
            .collect();
        let all_proc: Vec<Line> = proc_lines.into_iter().chain(empty).collect();
        frame.render_widget(Paragraph::new(all_proc), chunks[2]);

        // --- Network: rates + sparkline ---
        let total_rx: f64 = snap.net_rates.iter().map(|(_, rx, _)| rx).sum();
        let total_tx: f64 = snap.net_rates.iter().map(|(_, _, tx)| tx).sum();
        let net_line = Line::from(vec![
            Span::styled("network", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("  ↓ {}  ↑ {}", fmt_rate(total_rx), fmt_rate(total_tx)),
                Style::default().fg(Color::Cyan),
            ),
        ]);
        frame.render_widget(Paragraph::new(net_line), chunks[3]);

        if !snap.net_history.is_empty() {
            let hist: Vec<u64> = snap.net_history.iter().copied().collect();
            // Dynamic scale: peak of the visible window (floored at 1 KB/s so
            // near-idle traffic still draws a visible bump) instead of a fixed
            // 10 MB/s ceiling that flattens real-world rates to one row.
            let peak = hist.iter().copied().max().unwrap_or(1024).max(1024);
            let spark = Sparkline::default()
                .data(hist)
                .max(peak)
                .style(Style::default().fg(Color::Cyan));
            frame.render_widget(
                spark,
                Rect::new(
                    chunks[3].x,
                    chunks[3].y + 1,
                    chunks[3].width,
                    chunks[3].height.saturating_sub(1),
                ),
            );
        }
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

/// Root-filesystem usage percent via the disk module (reuses the same registry
/// path as memory; no extra /proc parsing in the hot path).
fn sample_disk(ctx: &Context) -> Option<u8> {
    match ModuleRegistry::get().run_individual("disk", ctx)? {
        InfoValue::List(entries) => {
            // The disk module emits "mount: total / used pct%" lines, root first.
            let line = entries.first()?;
            let pct = line.rsplit(' ').next()?.trim_end_matches('%');
            pct.parse().ok()
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
/// A compact 4-row "ring" gauge — the terminal analogue of the site mockup's
/// ring gauges. A 16-cell ring (7 top arc ▄, 2 sides █, 7 bottom arc ▀) fills
/// clockwise in the gauge color as pct rises; dim cells show the unfilled arc.
/// All three rows are 9 cells wide so the ring stays symmetric when centered.
///
/// ```text
///  ▄▄▄▄▄▄▄
/// █  37%  █
///  ▀▀▀▀▀▀▀
///    cpu
/// ```
fn ring_lines(label: &str, pct: f64, color: Color) -> Vec<Line<'static>> {
    let label = label.to_string();
    let total = 16usize; // 7 top + 1 right + 7 bottom + 1 left
    let filled = ((pct.clamp(0.0, 100.0) / 100.0) * total as f64).round() as usize;
    let dim = Style::default().fg(Color::DarkGray);
    let acc = Style::default().fg(color).add_modifier(Modifier::BOLD);

    // Clockwise from top-left: top arc 0-6, right side 7, bottom arc 8-14, left side 15.
    let cell = |i: usize, ch: char| -> Span<'static> {
        if i < filled {
            Span::styled(ch.to_string(), acc)
        } else {
            Span::styled(ch.to_string(), dim)
        }
    };

    let top: Vec<Span> = (0..7usize).map(|i| cell(i, '▄')).collect();
    let bottom: Vec<Span> = (8..15usize).map(|i| cell(i, '▀')).collect();

    let mut line1 = vec![Span::raw(" ")];
    line1.extend(top);
    line1.push(Span::raw(" "));
    // pct centered in the 7-cell inner gap (so 25% and 100% both fit).
    let pct_txt = format!("{:^7}", format!("{:.0}%", pct));
    let line2 = vec![
        cell(15, '█'),
        Span::styled(
            pct_txt,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        cell(7, '█'),
    ];
    let mut line3 = vec![Span::raw(" ")];
    line3.extend(bottom);
    line3.push(Span::raw(" "));

    vec![
        Line::from(line1),
        Line::from(line2),
        Line::from(line3),
        Line::from(Span::styled(label, Style::default().fg(Color::DarkGray))),
    ]
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
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

    #[test]
    fn test_bounded_channel_sender_never_blocks() {
        // crossbeam bounded(1) has no overwrite: a full channel rejects the
        // NEW sample with Err(Full) and keeps the old one. That's exactly what
        // we want here — the sampler's try_send can never block, and the
        // renderer drains with `while let Ok(snap) = rx.try_recv()` so it
        // always displays the newest sample that got through.
        let (tx, rx) = crossbeam_channel::bounded::<u32>(1);
        assert!(tx.try_send(1).is_ok());
        assert!(
            tx.try_send(2).is_err(),
            "channel full — sender never blocks"
        );
        assert_eq!(rx.try_recv(), Ok(1));
    }
}
