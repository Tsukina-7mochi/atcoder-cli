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
            let profile = config.get_profile(Some(&profile_name)).unwrap();
            let path = path.map(|s| PathBuf::from(s));
            let cwd = &config.cwd.unwrap();

            commands::init_task_directory(
                cwd,
                path.as_deref(),
                &profile,
                &contest_name,
                task_name.as_deref(),
            )
            .unwrap();
        }
        cli::Commands::Url { contest_name } => {
            let contest_name = contest_name.or(config.workspace_config.map(|c| c.contest.clone()));
            commands::show_contest_url(&mut out, &contest_name.unwrap())?;
        }
        cli::Commands::Run { manual } => {
            let cwd = &config.cwd.as_ref().unwrap();
            let profile = config.get_profile(None).unwrap();

            if manual {
                commands::run_test(&cwd, profile.build.as_deref(), &profile.run, None)
            } else {
                let workspace_config = config.workspace_config.unwrap();
                let contest_task_name = (
                    workspace_config.contest.as_str(),
                    workspace_config.task.as_str(),
                );
                commands::run_test(
                    &cwd,
                    profile.build.as_deref(),
                    &profile.run,
                    Some(contest_task_name),
                )
            }
        }
        cli::Commands::Test { manual } => {
            let cwd = &config.cwd.as_ref().unwrap();
            let profile = config.get_profile(None).unwrap();
            let (build_command, run_command) = if let Some(test_command) = profile.test {
                (None, test_command.clone())
            } else {
                (profile.build.as_deref(), profile.run.clone())
            };

            if manual {
                commands::run_test(&cwd, build_command, &run_command, None)
            } else {
                let workspace_config = config.workspace_config.unwrap();
                let contest_task_name = (
                    workspace_config.contest.as_str(),
                    workspace_config.task.as_str(),
                );
                commands::run_test(&cwd, build_command, &run_command, Some(contest_task_name))
            }
        }
        cli::Commands::Login => commands::login(),
        cli::Commands::Submit => {
            let workspace_path = config
                .workspace_path
                .as_ref()
                .expect("workspace path not provided");
            let profile = config.get_profile(None).expect("profile not provided");
            let contest_name = config
                .workspace_config
                .as_ref()
                .map(|c| c.contest.as_ref())
                .expect("contest name not provided");
            let task_name = config
                .workspace_config
                .as_ref()
                .map(|c| c.task.as_ref())
                .expect("contest name not provided");

            commands::submit(&workspace_path, &profile, contest_name, task_name)
        }
    }

    Ok(())
}
