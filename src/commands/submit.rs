use std::path::Path;
use std::{env, fs};

use crate::api;
use crate::profile::Profile;

pub fn submit(workspace_path: &Path, profile: &Profile, contest_name: &str, task_name: &str) {
    let session_cookie = env::var("ATCODER_SESSION").expect("Cannot get session cookie");
    let filepath = workspace_path.join(&profile.filename);
    let source_code = fs::read_to_string(&filepath).expect("Cannot red source code");

    api::submit::submit(
        contest_name,
        task_name,
        profile.language_id,
        &source_code,
        &session_cookie,
    )
}
