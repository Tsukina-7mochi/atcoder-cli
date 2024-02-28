use crate::commands;

pub fn login() -> commands::Result {
    let username = rprompt::prompt_reply("username> ").unwrap();
    let password = rpassword::prompt_password("password> ").unwrap();

    let session_cookie = crate::api::login::login(&username, &password)?;
    // TODO: implement saving session
    println!("{}", session_cookie);

    Ok(())
}
