use flexfetch_cli::Cli;
use flexfetch_core::{Config, SystemInfo, TeraEngine};

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
        "ansible" => match flexfetch_core::export::export_ansible(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "terraform" => match flexfetch_core::export::export_terraform(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "csv" => match flexfetch_core::export::export_csv(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "prometheus" => match flexfetch_core::export::export_prometheus(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
        "github" => match flexfetch_core::export::export_github(info) {
            Ok(s) => print!("{s}"),
            Err(e) => eprintln!("export error: {e}"),
        },
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
