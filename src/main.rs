mod api;
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
            let path = path.map(|s| PathBuf::from(s));
            let cwd = &config.cwd.unwrap();

            commands::init_task_directory(
                cwd,
                path.as_deref(),
                &profile,
                &contest_name,
                &task_name,
            )
            .unwrap();
        }
        cli::Commands::Url { contest_name } => {
            let contest_name = contest_name.or(config.workspace_config.map(|c| c.contest.clone()));
            commands::show_contest_url(&mut out, &contest_name.unwrap())?;
        }
    }

    Ok(())
}
