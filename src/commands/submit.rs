use std::fs;
use std::path::Path;

use crate::api;
use crate::commands;
use crate::config::Profile;

pub fn submit(
    workspace_path: &Path,
    profile: &Profile,
    contest_name: &str,
    task_name: &str,
    session_cookie: &str,
) -> commands::Result<()> {
    let filepath = workspace_path.join(&profile.filename);
    let source_code = fs::read_to_string(&filepath)?;

    api::submit::submit(
        contest_name,
        task_name,
        profile.language_id,
        &source_code,
        &session_cookie,
    )?;

    println!("Submitted successfully");

    return Ok(());
}
