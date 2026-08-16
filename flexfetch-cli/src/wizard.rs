//! Interactive config wizard (`--wizard`): a ratatui multi-step TUI that lets
//! the user pick modules, a theme, and a layout, then writes
//! `~/.config/flexfetch/config.toml`. Built on ratatui + crossterm (the same
//! deps as `--live`), so it lives behind the `live` feature.
//!
//! Steps: 1) module checklist  2) theme picker (with preview)  3) layout
//! (box style + frame)  4) summary + save.

use flexfetch_core::Config;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::io::IsTerminal;
use std::time::Duration;

/// All modules the wizard can toggle (matches `list_modules`).
const MODULES: &[&str] = &[
    "title",
    "separator",
    "os",
    "host",
    "kernel",
    "uptime",
    "locale",
    "datetime",
    "loadavg",
    "keyboard",
    "editor",
    "initsystem",
    "version",
    "bios",
    "board",
    "chassis",
    "brightness",
    "tpm",
    "cpu",
    "cpucache",
    "cpuusage",
    "memory",
    "swap",
    "disk",
    "gpu",
    "network",
    "localip",
    "dns",
    "display",
    "bluetooth",
    "media",
    "battery",
    "temperature",
    "processes",
    "packages",
    "shell",
    "terminal",
    "de",
    "wm",
    "colors",
    "custom",
    "publicip",
    "wifi",
    "git",
    "project",
    "context",
    "health",
];

/// Theme presets offered by the wizard (subset of theme.rs names).
const THEMES: &[&str] = &[
    "none",
    "catppuccin",
    "dracula",
    "nord",
    "gruvbox",
    "tokyo-night",
    "solarized-dark",
    "solarized-light",
    "rose-pine",
    "rose-pine-dawn",
    "everforest-dark",
    "everforest-light",
    "bamboo",
    "oxocarbon-dark",
    "one-dark",
    "one-light",
    "monokai",
    "monokai-pro",
    "ayu-dark",
    "palenight",
    "material-ocean",
    "kanagawa",
    "mellow-purple",
];

const BOX_STYLES: &[&str] = &["rounded", "double", "thick", "dotted", "ascii"];
const FRAMES: &[&str] = &["none", "single", "double"];

enum Step {
    Modules,
    Theme,
    Layout,
    Save,
}

struct Wizard {
    step: Step,
    selected: Vec<bool>,
    list_state: ListState,
    theme_idx: usize,
    box_idx: usize,
    frame_idx: usize,
}

impl Wizard {
    fn new() -> Self {
        let mut selected = vec![false; MODULES.len()];
        // Default on the most common modules (mirrors default preset).
        let defaults = Config::default_modules();
        for (i, name) in MODULES.iter().enumerate() {
            if defaults.iter().any(|d| d == name) {
                selected[i] = true;
            }
        }
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Wizard {
            step: Step::Modules,
            selected,
            list_state,
            theme_idx: 0,
            box_idx: 0,
            frame_idx: 0,
        }
    }

    fn module_count(&self) -> usize {
        self.selected.iter().filter(|&&s| s).count()
    }

    fn selected_modules(&self) -> Vec<String> {
        MODULES
            .iter()
            .zip(&self.selected)
            .filter(|(_, &on)| on)
            .map(|(&name, _)| name.to_string())
            .collect()
    }

    fn handle_key(&mut self, code: crossterm::event::KeyCode) -> bool {
        // Returns false when the wizard should quit.
        match code {
            crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => return false,
            _ => {}
        }
        match self.step {
            Step::Modules => match code {
                crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                    self.move_sel(-1);
                }
                crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                    self.move_sel(1);
                }
                crossterm::event::KeyCode::Char(' ') => {
                    if let Some(i) = self.list_state.selected() {
                        self.selected[i] = !self.selected[i];
                    }
                }
                crossterm::event::KeyCode::Char('a') => {
                    let all_on = self.selected.iter().all(|&s| s);
                    for s in &mut self.selected {
                        *s = !all_on;
                    }
                }
                crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Right => {
                    self.step = Step::Theme;
                    self.list_state = ListState::default();
                }
                _ => {}
            },
            Step::Theme => match code {
                crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                    self.theme_idx = self.theme_idx.saturating_sub(1);
                }
                crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                    self.theme_idx = (self.theme_idx + 1).min(THEMES.len() - 1);
                }
                crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Right => {
                    self.step = Step::Layout;
                }
                _ => {}
            },
            Step::Layout => match code {
                crossterm::event::KeyCode::Up
                | crossterm::event::KeyCode::Char('k')
                | crossterm::event::KeyCode::Down
                | crossterm::event::KeyCode::Char('j') => {
                    self.box_idx = (self.box_idx + 1) % BOX_STYLES.len();
                }
                crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                    self.frame_idx = self.frame_idx.saturating_sub(1);
                }
                crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                    self.frame_idx = (self.frame_idx + 1).min(FRAMES.len() - 1);
                }
                crossterm::event::KeyCode::Enter => {
                    self.step = Step::Save;
                }
                _ => {}
            },
            Step::Save => match code {
                crossterm::event::KeyCode::Char('y') | crossterm::event::KeyCode::Enter => {
                    return false; // save happens in run() after the loop
                }
                _ => {}
            },
        }
        true
    }

    fn move_sel(&mut self, delta: isize) {
        let len = MODULES.len() as isize;
        let cur = self.list_state.selected().unwrap_or(0) as isize;
        self.list_state
            .select(Some(((cur + delta).rem_euclid(len)) as usize));
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    if !std::io::stdout().is_terminal() {
        eprintln!("--wizard requires a terminal (stdout is not a tty)");
        std::process::exit(1);
    }

    let mut terminal = ratatui::init();
    let mut wizard = Wizard::new();
    let mut should_save = false;

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|frame| wizard.draw(frame))?;

            if crossterm::event::poll(Duration::from_millis(50))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind == crossterm::event::KeyEventKind::Press
                        && !wizard.handle_key(key.code)
                    {
                        // Quit — but if we're on the Save step with a
                        // confirmed 'y' (or Enter), save first. 'q' and Esc
                        // always cancel, even on the Save step.
                        let cancel = matches!(
                            key.code,
                            crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc
                        );
                        if matches!(wizard.step, Step::Save) && !cancel {
                            should_save = true;
                        }
                        return Ok(());
                    }
                }
            }
        }
    })();

    ratatui::restore();
    result?;

    if should_save {
        save_config(&wizard)?;
    } else {
        println!("wizard cancelled — no changes written");
    }
    Ok(())
}

fn save_config(wizard: &Wizard) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default_for_testing();
    config.modules = wizard.selected_modules();
    config.display.theme = Some(THEMES[wizard.theme_idx].to_string());
    config.display.box_style = BOX_STYLES[wizard.box_idx].to_string();
    config.display.frame = FRAMES[wizard.frame_idx].to_string();

    let toml = toml::to_string_pretty(&config)?;

    let config_dir = config_dir();
    std::fs::create_dir_all(&config_dir)?;
    let path = config_dir.join("config.toml");
    std::fs::write(&path, &toml)?;
    println!("wrote config to {path:?}");
    println!("  modules: {}", config.modules.clone().join(", "));
    println!(
        "  theme: {}",
        config.display.theme.as_deref().unwrap_or("none")
    );
    println!(
        "  box style: {}  frame: {}",
        config.display.box_style, config.display.frame
    );
    Ok(())
}

fn config_dir() -> std::path::PathBuf {
    crate::tools::config_dir()
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

impl Wizard {
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let title = match self.step {
            Step::Modules => "flexfetch config wizard — 1/4 modules",
            Step::Theme => "flexfetch config wizard — 2/4 theme",
            Step::Layout => "flexfetch config wizard — 3/4 layout",
            Step::Save => "flexfetch config wizard — 4/4 save",
        };

        let header = Line::from(vec![
            Span::styled(
                format!(" {title} "),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  [↑/↓] move  [space] toggle  [a] all  [enter] next  [q] quit  ",
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        frame.render_widget(
            Paragraph::new(header),
            Rect::new(area.x, area.y, area.width, 1),
        );

        let body = Rect::new(
            area.x,
            area.y + 1,
            area.width,
            area.height.saturating_sub(1),
        );

        match self.step {
            Step::Modules => self.draw_modules(frame, body),
            Step::Theme => self.draw_theme(frame, body),
            Step::Layout => self.draw_layout(frame, body),
            Step::Save => self.draw_save(frame, body),
        }
    }

    fn draw_modules(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = MODULES
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let mark = if self.selected[i] { "[x]" } else { "[ ]" };
                ListItem::new(Line::from(vec![
                    Span::styled(
                        mark,
                        Style::default().fg(if self.selected[i] {
                            Color::Green
                        } else {
                            Color::DarkGray
                        }),
                    ),
                    Span::raw(" "),
                    Span::styled(*name, Style::default()),
                ]))
            })
            .collect();

        let count = self.module_count();
        let block = Block::bordered().title(format!(
            " Modules ({count}/{}) — space toggles ",
            MODULES.len()
        ));

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let mut state = self.list_state;
        frame.render_stateful_widget(list, area, &mut state);
        self.list_state = state;
    }

    fn draw_theme(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
            .split(area);

        let items: Vec<ListItem> = THEMES
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let cur = if i == self.theme_idx { "> " } else { "  " };
                ListItem::new(Line::from(vec![
                    Span::styled(cur, Style::default().fg(Color::Cyan)),
                    Span::styled(*name, Style::default()),
                ]))
            })
            .collect();
        let list = List::new(items)
            .block(Block::bordered().title(" Theme "))
            .highlight_style(Style::default().bg(Color::DarkGray));
        let mut state = ListState::default();
        state.select(Some(self.theme_idx));
        frame.render_stateful_widget(list, chunks[0], &mut state);

        // Preview: show a fake fetch line in the selected theme
        let mut preview_cfg = Config::default_for_testing();
        let name = THEMES[self.theme_idx];
        if name != "none" {
            preview_cfg.display.theme = Some(name.to_string());
        }
        let theme = flexfetch_core::theme::resolve(&preview_cfg);
        let line = format!(
            "{}┌─ {}OS: {}Preview Distro{} {}┐{}",
            theme.section, theme.keys, theme.values, theme.reset, theme.sep, theme.reset,
        );
        let gradient =
            flexfetch_core::theme::gradient_text("flexfetch preview", &theme.gradient_colors);
        let preview_lines = vec![
            Line::from(Span::raw("")),
            Line::from(Span::raw(&gradient)),
            Line::from(Span::raw(&line)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "enter: accept   ↑/↓: change",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        frame.render_widget(
            Paragraph::new(preview_lines)
                .block(Block::bordered().title(" Preview "))
                .wrap(Wrap { trim: false }),
            chunks[1],
        );
    }

    fn draw_layout(&self, frame: &mut Frame, area: Rect) {
        let items = vec![
            ListItem::new(Line::from(vec![
                Span::styled("box style: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    BOX_STYLES[self.box_idx].to_string(),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled("   (↑/↓ to change)", Style::default().fg(Color::DarkGray)),
            ])),
            ListItem::new(Line::from(vec![
                Span::styled("frame:     ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    FRAMES[self.frame_idx].to_string(),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled("   (←/→ to change)", Style::default().fg(Color::DarkGray)),
            ])),
            ListItem::new(Line::from(vec![Span::styled(
                "enter: continue",
                Style::default().fg(Color::DarkGray),
            )])),
        ];
        frame.render_widget(
            List::new(items).block(Block::bordered().title(" Layout ")),
            area,
        );
    }

    fn draw_save(&self, frame: &mut Frame, area: Rect) {
        let modules = self.selected_modules();
        let lines = vec![
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "Review your configuration:",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                format!("  modules ({})", modules.len()),
                Style::default().fg(Color::Cyan),
            )),
        ];
        let mut all = lines;
        for chunk in modules.chunks(6) {
            all.push(Line::from(Span::styled(
                format!("    {}", chunk.join(", ")),
                Style::default().fg(Color::Green),
            )));
        }
        all.push(Line::from(Span::raw("")));
        all.push(Line::from(Span::styled(
            format!("  theme: {}", THEMES[self.theme_idx]),
            Style::default().fg(Color::Cyan),
        )));
        all.push(Line::from(Span::styled(
            format!(
                "  box style: {}   frame: {}",
                BOX_STYLES[self.box_idx], FRAMES[self.frame_idx]
            ),
            Style::default().fg(Color::Cyan),
        )));
        all.push(Line::from(Span::raw("")));
        all.push(Line::from(Span::styled(
            format!("  will write: {}/config.toml", config_dir().display()),
            Style::default().fg(Color::DarkGray),
        )));
        all.push(Line::from(Span::raw("")));
        all.push(Line::from(Span::styled(
            "  [y] save and exit   [q] cancel",
            Style::default().fg(Color::Yellow),
        )));

        frame.render_widget(
            Paragraph::new(all).block(Block::bordered().title(" Save ")),
            area,
        );
    }
}
