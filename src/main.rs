mod cli;
mod commands;
mod config;
mod profile;
mod util;

use std::io::{self, BufWriter};
use std::path::PathBuf;

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
        cli::Commands::Init {
            contest_name,
            task_name,
            profile_name,
            path,
        } => {
            let global_config = config.global_config.unwrap();
            let path = path
                .map(|p| PathBuf::from(p))
                .or(config.cwd.clone())
                .unwrap();
            let profile = {
                let profile = profile::defaults::get_default(&profile_name);
                if let Some(gp) = global_config.profiles.get(&profile_name) {
                    match profile {
                        Some(p) => Some(gp.merge_default(&p)),
                        None => gp.clone().to_profile(&profile_name),
                    }
                } else {
                    profile
                }
            }
            .unwrap();

            commands::init_task_directory(&path, &profile, &contest_name, &task_name).unwrap();
        }
    }

    Ok(())
}
