use std::fs;
use std::io;
use std::path::Path;

use subprocess::Exec;

use crate::api;
use crate::config::Profile;
use crate::config::WorkspaceConfig;

pub fn init_task_directory(
    cwd: &Path,
    path: Option<&Path>,
    profile: &Profile,
    contest_name: &str,
    task_name: Option<&str>,
) -> io::Result<()> {
    if task_name.is_none() {
        let tasks = api::get_contest_tasks::get_contest_tasks(contest_name);
        for task in tasks {
            println!("{}: {}", task.name, task.label);
            init_task_directory(cwd, path, profile, contest_name, Some(&task.name)).unwrap()
        }
        return Ok(());
    }
    let task_name = task_name.unwrap();

    let path = match path {
        Some(p) => p.to_path_buf(),
        None => cwd.join(contest_name).join(task_name),
    };
    let path = path.as_path();

    if path.exists() {
        if !path.is_dir() {
            panic!("Path {:?} is not directory.", path);
        }
    } else {
        fs::create_dir_all(path).unwrap();
    }

    println!("Initializing {:?}", path);

    let init_command = (&profile.init.as_ref())
        .unwrap()
        .replace("{contestName}", contest_name)
        .replace("{taskName}", task_name);
    Exec::shell(init_command).cwd(path).capture().unwrap();

    let workspace_config_path = path.join("atcoder.yml");
    let workspace_config = WorkspaceConfig {
        contest: contest_name.to_owned(),
        task: task_name.to_owned(),
        profile: profile.name.to_owned(),
    };
    let mut workspace_config = serde_yaml::to_string(&workspace_config).unwrap();
    workspace_config.insert_str(0, "version: \"1\"\n");
    fs::write(workspace_config_path, workspace_config).unwrap();

    Ok(())
}
