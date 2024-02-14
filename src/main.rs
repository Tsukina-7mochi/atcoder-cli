mod cli;
mod commands;
mod config;
mod profile;
mod util;

use std::io::{self, BufWriter};

use clap::Parser;

fn main() -> io::Result<()> {
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    let cli = cli::CLI::parse();
    let config = config::get_config();

    match cli.command {
        cli::Commands::Info => commands::info(
            &mut out,
            config.global_config_path.as_deref(),
            config.workspace_config_path.as_deref(),
            config.workspace_path.as_deref(),
            config.workspace_config.as_ref(),
        )?,
    }

    Ok(())
}
