use flexfetch_cli::Cli;
use flexfetch_core::{Config, SystemInfo};

/// Render output in the requested format.
pub fn render(info: &SystemInfo, config: &Config, cli: &Cli, ssh: bool) {
    crate::render_output(info, config, cli, ssh)
}

/// Handle export to file.
pub fn export(
    info: &SystemInfo,
    config: &Config,
    format: &str,
    output: Option<&std::path::Path>,
) -> bool {
    crate::handle_export(info, config, format, output)
}
