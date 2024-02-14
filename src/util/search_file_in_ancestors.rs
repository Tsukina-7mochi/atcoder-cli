use std::fs;
use std::path::{Path, PathBuf};

pub fn search_file_in_ancestors(root: &Path, file_name: &str) -> Option<PathBuf> {
    for path in root.ancestors() {
        let config_path = path.join(file_name);
        if fs::metadata(&config_path).ok()?.is_file() {
            return Some(config_path);
        }
    }
    None
}
