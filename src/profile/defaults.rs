use super::Profile;

pub fn get_default(name: &str) -> Option<Profile> {
    match name {
        "rust" => Some(Profile::new(
            "rust",
            "cargo build --release --offline",
            "./target/release/${taskName}",
            "cargo run",
            "cargo init .",
        )),
        _ => None,
    }
}
