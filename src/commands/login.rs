use crate::commands;
use crate::config;

pub fn login(env_session: bool) -> commands::Result {
    let username = rprompt::prompt_reply("username> ").unwrap();
    let password = rpassword::prompt_password("password> ").unwrap();

    let session_cookie = crate::api::login::login(&username, &password)?;

    if env_session {
        config::session_cookie::env::set(&session_cookie);
        println!("{}", session_cookie);
    } else {
        config::session_cookie::keyring::set(&session_cookie)?;
        println!("Login succeeded.");
    }

    Ok(())
}
