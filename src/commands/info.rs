use std::io;
use std::path::Path;

use crate::config::WorkspaceConfig;

pub fn info(
    global_config_path: Option<&Path>,
    workspace_config_path: Option<&Path>,
    workspace_path: Option<&Path>,
    workspace_config: Option<&WorkspaceConfig>,
) -> io::Result<()> {
    if let Some(path) = global_config_path {
        let path = path.to_string_lossy();
        println!("Global configuration file: {}", path);
    } else {
        println!("Global configuration file: None");
    }

    if let Some(path) = workspace_config_path {
        let path = path.to_string_lossy();
        println!("Task configuration file: {}", path);
    } else {
        println!("Task configuration file: None");
    }

    if let Some(path) = workspace_path {
        let path = path.to_string_lossy();
        println!("Task directory: {}", path);
    } else {
        println!("Task directory: None");
    }

    if let Some(config) = workspace_config {
        println!("Task configuration:");
        println!("  contest: {}", config.contest);
        println!("  task   : {}", config.task);
        println!("  profile: {}", config.profile);
    } else {
        println!("Task configuration: None");
    }

    Ok(())
}
