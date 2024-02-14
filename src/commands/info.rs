use std::io;
use std::path::Path;

use crate::config::WorkspaceConfig;

pub fn info(
    out: &mut impl io::Write,
    global_config_path: Option<&Path>,
    workspace_config_path: Option<&Path>,
    workspace_path: Option<&Path>,
    workspace_config: Option<&WorkspaceConfig>,
) -> io::Result<()> {
    if let Some(path) = global_config_path {
        let path = path.to_string_lossy();
        writeln!(out, "Global configuration file: {}", path)?;
    } else {
        writeln!(out, "Global configuration file: None")?;
    }

    if let Some(path) = workspace_config_path {
        let path = path.to_string_lossy();
        writeln!(out, "Task configuration file: {}", path)?;
    } else {
        writeln!(out, "Task configuration file: None")?;
    }

    if let Some(path) = workspace_path {
        let path = path.to_string_lossy();
        writeln!(out, "Task directory: {}", path)?;
    } else {
        writeln!(out, "Task directory: None")?;
    }

    if let Some(config) = workspace_config {
        writeln!(out, "Task configuration:")?;
        writeln!(out, "  contest: {}", config.contest)?;
        writeln!(out, "  task   : {}", config.task)?;
        writeln!(out, "  profile: {}", config.profile)?;
    } else {
        writeln!(out, "Task configuration: None")?;
    }

    Ok(())
}
