mod api;
mod cli;
mod commands;
mod config;
mod error;
mod util;

use std::io::{self, BufWriter};
use std::path::PathBuf;

use clap::Parser;

fn run_test(config: &config::GetConfigsResult, manual: bool, run: bool) -> error::Result {
    let cwd = config.cwd.as_ref().unwrap();
    let profile = config.get_profile(None).unwrap();

    let (build_command, run_command) = if run {
        (profile.build.as_deref(), profile.run.clone())
    } else {
        if let Some(test_command) = profile.test {
            (None, test_command.clone())
        } else {
            (profile.build.as_deref(), profile.run.clone())
        }
    };

    if manual {
        commands::run_test(&cwd, build_command, &run_command, None)?;
    } else {
        let workspace_config = config
            .workspace_config
            .as_ref()
            .ok_or(error::ConfigErrorKind::WorkspaceConfigNotProvided)?;
        let contest_task_name = (
            workspace_config.contest.as_str(),
            workspace_config.task.as_str(),
        );
        commands::run_test(&cwd, build_command, &run_command, Some(contest_task_name))?;
    }

    Ok(())
}

fn run() -> error::Result {
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    let cli = cli::CLI::parse();
    let config = config::get_config(cli.env_session);

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
            let profile = config
                .get_profile(Some(&profile_name))
                .ok_or(error::ConfigErrorKind::ProfileNotProvided)?;
            let path = path.map(|s| PathBuf::from(s));
            let cwd = &config.cwd.ok_or(error::ConfigErrorKind::CWDNotProvided)?;

            commands::init_task_directory(
                cwd,
                path.as_deref(),
                &profile,
                &contest_name,
                task_name.as_deref(),
            )?;
        }
        cli::Commands::Url { contest_name } => {
            let contest_name = contest_name
                .or(config.workspace_config.map(|c| c.contest.clone()))
                .ok_or(error::ConfigErrorKind::ContestNameNotProvided)?;
            commands::show_contest_url(&mut out, &contest_name)?;
        }
        cli::Commands::Run { manual } => {
            run_test(&config, manual, true)?;
        }
        cli::Commands::Test { manual } => {
            run_test(&config, manual, false)?;
        }
        cli::Commands::Login => commands::login(cli.env_session)?,
        cli::Commands::Submit => {
            let workspace_path = config
                .workspace_path
                .as_ref()
                .ok_or(error::ConfigErrorKind::WorkspacePathNotProvided)?;
            let profile = config
                .get_profile(None)
                .ok_or(error::ConfigErrorKind::ProfileNotProvided)?;
            let contest_name = config
                .workspace_config
                .as_ref()
                .map(|c| c.contest.as_ref())
                .ok_or(error::ConfigErrorKind::ContestNameNotProvided)?;
            let task_name = config
                .workspace_config
                .as_ref()
                .map(|c| c.task.as_ref())
                .ok_or(error::ConfigErrorKind::TaskNameNotProvided)?;
            let session_cookie = config
                .session_cookie
                .map_err(|err| error::ConfigErrorKind::SessionCookieNotProvided(err))?;

            commands::submit(
                &workspace_path,
                &profile,
                contest_name,
                task_name,
                &session_cookie,
            )?;
        }
    }

    Ok(())
}

fn main() {
    let result = run();
    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
