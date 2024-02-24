use std::env;
use std::fs;
use std::path::PathBuf;

use super::profile::{self, Profile};
use crate::util;

use super::global_config;
use super::{GlobalConfig, WorkspaceConfig};

pub struct GetConfigsResult {
    pub cwd: Option<PathBuf>,
    pub workspace_config_path: Option<PathBuf>,
    pub workspace_path: Option<PathBuf>,
    pub workspace_config: Option<WorkspaceConfig>,
    pub global_config_path: Option<PathBuf>,
    pub global_config: Option<GlobalConfig>,
    pub session_cookie: Option<String>,
}

impl GetConfigsResult {
    pub fn get_profile(&self, profile_name: Option<&str>) -> Option<Profile> {
        let profile_name = profile_name.or(self
            .workspace_config
            .as_ref()
            .map(|config| config.profile.as_str()))?;
        let profile = profile::get_default(&profile_name);

        if let Some(gp) = self
            .global_config
            .as_ref()
            .and_then(|config| config.profiles.get(profile_name))
        {
            match profile {
                Some(p) => Some(gp.merge_default(&p)),
                None => gp.clone().to_profile(&profile_name),
            }
        } else {
            profile
        }
    }
}

pub fn get_config() -> GetConfigsResult {
    let cwd = env::current_dir().ok();

    let workspace_config_path = cwd
        .as_ref()
        .and_then(|cwd| util::search_file_in_ancestors(cwd, "atcoder.yml"));
    let workspace_path = workspace_config_path
        .as_ref()
        .and_then(|path| path.parent())
        .map(|p| p.to_path_buf());
    let workspace_config = workspace_path
        .as_ref()
        .map(|path| fs::read_to_string(path.join("atcoder.yml")).unwrap())
        .map(|content| serde_yaml::from_str::<WorkspaceConfig>(&content).unwrap());

    let global_config_path = global_config::get_config_path();
    let global_config = global_config_path
        .as_ref()
        .map(|path| fs::read_to_string(path).unwrap())
        .map(|content| serde_yaml::from_str::<GlobalConfig>(&content).unwrap());
    let session_cookie = env::var("ATCODER_SESSION").ok();

    return GetConfigsResult {
        cwd,
        workspace_config_path,
        workspace_path,
        workspace_config,
        global_config_path,
        global_config,
        session_cookie,
    };
}
