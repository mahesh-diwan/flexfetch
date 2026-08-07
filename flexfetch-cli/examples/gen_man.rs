use clap::CommandFactory;
use std::io::Write;

fn main() {
    let cmd = flexfetch_cli::Cli::command().version(env!("CARGO_PKG_VERSION"));
    let man = clap_mangen::Man::new(cmd)
        .title("FLEXFETCH")
        .section("1")
        .date("August 2026");

    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();

    // Append sections that clap_mangen can't derive from clap attrs.
    let mut extra = Vec::new();
    writeln!(extra, ".SH CONFIGURATION").unwrap();
    writeln!(
        extra,
        "Configuration is loaded from ~/.config/flexfetch/config.toml."
    )
    .unwrap();
    writeln!(
        extra,
        "Use \\fBflexfetch --gen-config\\fR to create a default configuration."
    )
    .unwrap();
    writeln!(extra).unwrap();
    writeln!(
        extra,
        "Presets are stored in ~/.config/flexfetch/presets/ as .toml files."
    )
    .unwrap();
    writeln!(extra).unwrap();
    writeln!(
        extra,
        "Feature\\-gated builds: \\fB--no-default-features\\fR drops Lua, the live"
    )
    .unwrap();
    writeln!(
        extra,
        "dashboard, image logos, the Tera template engine, and Rayon for a"
    )
    .unwrap();
    writeln!(
        extra,
        "~1.5 MB minimal binary (plain \\fB||--||\\fR tree connectors instead"
    )
    .unwrap();
    writeln!(extra, "of templates).").unwrap();

    writeln!(extra, ".SH EXAMPLES").unwrap();
    writeln!(extra, "Display default modules:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Show only CPU and memory:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch -m cpu:memory").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Export to SVG:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch --export svg -o system.svg").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Use server preset:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch --preset server").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Pipe\\-friendly output:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch --pipe | head -20").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Shell prompt string:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "PS1=\"$(flexfetch --prompt) $ \"").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Remote hosts over SSH:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch --ssh server1 --ssh server2").unwrap();
    writeln!(extra, ".RE").unwrap();
    writeln!(extra).unwrap();
    writeln!(extra, "Configure interactively:").unwrap();
    writeln!(extra, ".PP").unwrap();
    writeln!(extra, ".RS 4").unwrap();
    writeln!(extra, "flexfetch --wizard").unwrap();
    writeln!(extra, ".RE").unwrap();

    writeln!(extra, ".SH AUTHORS").unwrap();
    writeln!(extra, "Written by Mahesh Diwan and contributors.").unwrap();

    buffer.extend_from_slice(&extra);

    let out = std::path::Path::new("doc/flexfetch.1");
    std::fs::create_dir_all(out.parent().unwrap()).ok();
    std::fs::write(out, &buffer).unwrap();
    println!("wrote {}", out.display());
}
