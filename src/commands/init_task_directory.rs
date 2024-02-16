use std::fs;
use std::io;
use std::path::Path;

use subprocess::Exec;

use crate::config::WorkspaceConfig;
use crate::profile::Profile;

pub fn init_task_directory(
    path: &Path,
    profile: &Profile,
    contest_name: &str,
    task_name: &str,
) -> io::Result<()> {
    if path.exists() {
        if !path.is_dir() {
            panic!("Path {:?} is not directory.", path);
        }
    } else {
        fs::create_dir_all(path).unwrap();
    }
    println!("{:?}", path);

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
