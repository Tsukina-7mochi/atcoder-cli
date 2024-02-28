use std::fs;
use std::path::Path;

use subprocess::Exec;

use crate::api;
use crate::commands::{self, error::InitTaskDirectoryErrorKind, CommandError};
use crate::config::Profile;
use crate::config::WorkspaceConfig;

pub fn init_task_directory(
    cwd: &Path,
    path: Option<&Path>,
    profile: &Profile,
    contest_name: &str,
    task_name: Option<&str>,
) -> commands::Result {
    let task_name = if let Some(task_name) = task_name {
        task_name
    } else {
        // initialize all tasks in the contest (evaluated as `()`)
        let tasks = api::get_contest_tasks::get_contest_tasks(contest_name)?;
        for task in tasks {
            println!("{}: {}", task.name, task.label);
            init_task_directory(cwd, path, profile, contest_name, Some(&task.name))?;
        }
        return Ok(());
    };

    let path = match path {
        Some(p) => p.to_path_buf(),
        None => cwd.join(contest_name).join(task_name),
    };
    let path = path.as_path();
    if path.exists() {
        if !path.is_dir() {
            return Err(InitTaskDirectoryErrorKind::PathIsNotDirectory(
                path.to_string_lossy().to_string(),
            )
            .into());
        }
    } else {
        fs::create_dir_all(path).map_err(|err| -> CommandError {
            InitTaskDirectoryErrorKind::FailedToCreateDirectory(err).into()
        })?;
    }

    let init_command = (&profile.init.as_ref()).map(|s| {
        s.replace("{contestName}", contest_name)
            .replace("{taskName}", task_name)
    });
    if let Some(init_command) = init_command {
        println!("Running: {}", init_command);
        let result = Exec::shell(init_command)
            .cwd(path)
            .capture()
            .map_err(|err| InitTaskDirectoryErrorKind::InitCommandFailed(err.into()))?;
        if !result.success() {
            return Err(
                InitTaskDirectoryErrorKind::InitCommandFailed(result.exit_status.into()).into(),
            );
        }
    } else {
        println!("No init command specified.")
    }

    let workspace_config_path = path.join("atcoder.yml");
    let workspace_config = WorkspaceConfig {
        contest: contest_name.to_owned(),
        task: task_name.to_owned(),
        profile: profile.name.to_owned(),
    };
    let mut workspace_config = serde_yaml::to_string(&workspace_config).unwrap();
    workspace_config.insert_str(0, "version: \"1\"\n");
    fs::write(workspace_config_path, workspace_config)?;

    Ok(())
}
