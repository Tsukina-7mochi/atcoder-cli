use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::PartialProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub version: String,
    pub profiles: BTreeMap<String, PartialProfile>,
}

pub fn get_config_path() -> Option<PathBuf> {
    env::var("ATCODER_GLOBAL_CONFIG")
        .map(|path| PathBuf::from(path))
        .ok()
        .or_else(|| dirs::home_dir().map(|home| home.join(".atcoder.yml")))
}
