use flexfetch_cli::Cli;
use flexfetch_core::{Config, Context, InfoValue, ModuleRegistry, SystemInfo, TeraEngine};
use std::collections::HashMap;

/// Render output in the requested format (`--format text|json|markdown|...`).
pub fn render(info: &SystemInfo, config: &Config, cli: &Cli, ssh: bool) {
    match cli.format.as_str() {
        "json" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&info.to_json()).unwrap_or_else(|_| "{}".into())
            );
        }
        "markdown" | "md" if !ssh => match flexfetch_core::export::export_markdown(info, config) {
            Ok(md) => print!("{md}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        // The single-info text exporters all share one shape: build the
        // string from the info, then print it (or report the error once).
        "ansible" | "terraform" | "csv" | "prometheus" | "github" => {
            let exporter: fn(&SystemInfo) -> flexfetch_core::Result<String> =
                match cli.format.as_str() {
                    "ansible" => flexfetch_core::export::export_ansible,
                    "terraform" => flexfetch_core::export::export_terraform,
                    "csv" => flexfetch_core::export::export_csv,
                    "prometheus" => flexfetch_core::export::export_prometheus,
                    _ => flexfetch_core::export::export_github,
                };
            match exporter(info) {
                Ok(s) => print!("{s}"),
                Err(e) => eprintln!("export error: {e}"),
            }
        }
        _ => {
            // --ssh targets render through the full template engine even on
            // --flash (render_info legacy behavior — no flash fast-path).
            if !ssh && cli.flash && !cli.demo {
                println!("{}", flexfetch_core::template::render_flash(info));
                return;
            }
            let engine = TeraEngine::new_default();
            match engine.render(info, config) {
                Ok(output) => {
                    let out = if config.display.frame != "none" {
                        let theme = flexfetch_core::theme::resolve(config);
                        flexfetch_core::template::frame_wrap(
                            &output,
                            &config.display.frame,
                            &theme.section,
                        )
                    } else {
                        output
                    };
                    println!("{out}");
                }
                Err(e) => eprintln!("template error: {e}"),
            }
        }
    }
}

/// Render a 3-column side-by-side diff table (`--diff`). Rows are aligned by
/// module name; differing values are highlighted (red for A, green for B).
pub fn diff(a: &SystemInfo, b: &SystemInfo, name_a: &str, name_b: &str) {
    let a_map: HashMap<&str, &InfoValue> = a.entries.iter().map(|(n, v)| (*n, v)).collect();
    let b_map: HashMap<&str, &InfoValue> = b.entries.iter().map(|(n, v)| (*n, v)).collect();

    // Union of module names, preserving A's order then any B-only modules.
    let mut names: Vec<&str> = a.entries.iter().map(|(n, _)| *n).collect();
    for (n, _) in &b.entries {
        if !names.contains(n) {
            names.push(n);
        }
    }

    let w = 12usize;
    println!("\x1b[1;36m{name_a:<20}\x1b[0m vs \x1b[1;36m{name_b:<20}\x1b[0m");
    println!("{:<w$} | {:<24} | {:<24}", "Property", name_a, name_b);
    println!("{:-<1$}", "", w + 2 + 26 + 26);

    for name in names {
        let va = a_map.get(name).map(|v| v.summary()).unwrap_or_default();
        let vb = b_map.get(name).map(|v| v.summary()).unwrap_or_default();
        let (ca, cb) = if va != vb {
            ("\x1b[31m", "\x1b[32m")
        } else {
            ("", "")
        };
        println!(
            "{:<w$} | {ca}{:<24}\x1b[0m | {cb}{:<24}\x1b[0m",
            name, va, vb
        );
    }
}

/// Build the single-line prompt string (`--prompt`), e.g.
/// `cachyos | CPU 12% | RAM 3.2G/15.3G`.
pub fn prompt(ctx: &Context, modules: &[String]) -> String {
    let registry = ModuleRegistry::get();
    let mut parts: Vec<String> = Vec::new();

    // OS: distro name/logo-ish hint
    if modules.iter().any(|m| m == "os") {
        if let Some(InfoValue::Map(m)) = registry.run_individual("os", ctx) {
            let name = m
                .get("pretty_name")
                .or_else(|| m.get("name"))
                .cloned()
                .unwrap_or_default();
            if !name.is_empty() {
                parts.push(name.to_lowercase());
            }
        }
    }
    // CPU usage
    if modules.iter().any(|m| m == "cpuusage") {
        if let Some(InfoValue::Scalar(s)) = registry.run_individual("cpuusage", ctx) {
            if s != "unknown" {
                parts.push(format!("CPU {s}"));
            }
        }
    }
    // Memory
    if modules.iter().any(|m| m == "memory") {
        if let Some(InfoValue::Map(m)) = registry.run_individual("memory", ctx) {
            let used = m.get("used").cloned().unwrap_or_default();
            let total = m.get("total").cloned().unwrap_or_default();
            if !used.is_empty() && !total.is_empty() {
                parts.push(format!("RAM {used}/{total}"));
            }
        }
    }

    if parts.is_empty() {
        String::new()
    } else {
        parts.join(" | ")
    }
}

/// Handle export to file (`--export svg|html|png|markdown`).
pub fn export(
    info: &SystemInfo,
    config: &Config,
    format: &str,
    output: Option<&std::path::Path>,
) -> bool {
    let path = output.unwrap_or_else(|| match format {
        "svg" => std::path::Path::new("flexfetch.svg"),
        "html" => std::path::Path::new("flexfetch.html"),
        "png" => std::path::Path::new("flexfetch.png"),
        "markdown" | "md" => std::path::Path::new("flexfetch.md"),
        _ => std::path::Path::new("flexfetch.out"),
    });
    match format {
        "svg" => match flexfetch_core::export::export_svg(info, config) {
            Ok(svg) => {
                if let Err(e) = std::fs::write(path, &svg) {
                    eprintln!("write error: {e}");
                } else {
                    println!("wrote {path:?}");
                }
            }
            Err(e) => eprintln!("export error: {e}"),
        },
        "html" => match flexfetch_core::export::export_html(info, config) {
            Ok(html) => {
                if let Err(e) = std::fs::write(path, &html) {
                    eprintln!("write error: {e}");
                } else {
                    println!("wrote {path:?}");
                }
            }
            Err(e) => eprintln!("export error: {e}"),
        },
        "png" => match flexfetch_core::export::export_png(info, config, path) {
            Ok(()) => println!("wrote {path:?}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "markdown" | "md" => match flexfetch_core::export::export_markdown(info, config) {
            Ok(md) => {
                if let Err(e) = std::fs::write(path, &md) {
                    eprintln!("write error: {e}");
                } else {
                    println!("wrote {path:?}");
                }
            }
            Err(e) => eprintln!("export error: {e}"),
        },
        _ => {
            eprintln!("unknown export format: {format} (use svg, html, png, markdown)");
            return false;
        }
    }
    true
}
