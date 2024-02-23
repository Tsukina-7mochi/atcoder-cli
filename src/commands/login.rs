use std::io::{self, Write};

pub fn login() {
    print!("username> ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_owned();

    let password = rpassword::prompt_password("password> ").unwrap();

    let session_cookie = crate::api::login::login(&username, &password);
    if let Some(session_cookie) = session_cookie {
        // TODO: implement saving session
        println!("{}", session_cookie);
    } else {
        println!("Login failed.");
    }
}
