use std::io;

use crate::api::url;

pub fn show_contest_url(out: &mut impl io::Write, contest_name: &str) -> io::Result<()> {
    writeln!(out, "{}", url::contest(contest_name))?;

    Ok(())
}
