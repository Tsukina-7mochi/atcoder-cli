pub mod env {
    use std::env;

    const ENV_KEY: &str = "ATCODER_SESSION";

    pub fn get() -> Result<String, env::VarError> {
        env::var(ENV_KEY)
    }

    pub fn set(value: &str) {
        env::set_var(ENV_KEY, value);
    }
}

pub mod keyring {
    const SERVICE: &str = "atcoder-cli";
    const USERNAME: &str = "session";

    fn entry() -> keyring::Result<keyring::Entry> {
        keyring::Entry::new(SERVICE, USERNAME)
    }

    pub fn get() -> keyring::Result<String> {
        entry().and_then(|entry| entry.get_password())
    }

    pub fn set(value: &str) -> keyring::Result<()> {
        entry().and_then(|entry| entry.set_password(value))
    }
}
