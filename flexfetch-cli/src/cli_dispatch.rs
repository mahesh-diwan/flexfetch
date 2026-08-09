use flexfetch_cli::Commands;

/// Handle subcommands that run before config load.
pub fn handle_subcommands(cli: &flexfetch_cli::Cli) -> bool {
    if let Some(command) = &cli.command {
        match command {
            #[cfg(feature = "completions")]
            Commands::Completions { shell } => {
                use clap::CommandFactory;
                let mut cmd = flexfetch_cli::Cli::command();
                clap_complete::generate(*shell, &mut cmd, "flexfetch", &mut std::io::stdout());
            }
            Commands::Plugin { action } => crate::registry::run(action),
        }
        return true;
    }
    false
}

/// Handle flags that run before config load.
pub fn handle_preflags(cli: &flexfetch_cli::Cli) -> bool {
    if cli.gen_config {
        crate::generate_config();
        return true;
    }
    if cli.list_modules {
        crate::list_modules();
        return true;
    }
    if cli.list_presets {
        crate::list_presets();
        return true;
    }
    if cli.list_themes {
        for name in flexfetch_core::theme::preset_names() {
            println!("{name}");
        }
        return true;
    }
    if let Some(ref shell) = cli.hook {
        crate::tools::print_hook(shell);
        return true;
    }
    if cli.update {
        crate::tools::self_update();
        return true;
    }
    if cli.update_db {
        match flexfetch_core::hardware_db::refresh() {
            Ok(msg) => println!("{msg}"),
            Err(e) => {
                eprintln!("update-db: {e}");
                std::process::exit(1);
            }
        }
        return true;
    }
    false
}
