pub fn login() {
    let username = rprompt::prompt_reply("username> ").unwrap();
    let password = rpassword::prompt_password("password> ").unwrap();

    let session_cookie = crate::api::login::login(&username, &password);
    if let Some(session_cookie) = session_cookie {
        // TODO: implement saving session
        println!("{}", session_cookie);
    } else {
        println!("Login failed.");
    }
}
