use std::io;

use crate::api::url;
use crate::commands;

pub fn show_contest_url(out: &mut impl io::Write, contest_name: &str) -> commands::Result<()> {
    writeln!(out, "{}", url::contest(contest_name))?;

    Ok(())
}
