use super::Profile;

pub fn get_default(name: &str) -> Option<Profile> {
    match name {
        "rust" => Some(Profile::new(
            "rust",
            5054,
            "cargo build --release --offline",
            "./target/release/${taskName}",
            "cargo run",
            "cargo init .",
            "./src/main.rs",
        )),
        _ => None,
    }
}
