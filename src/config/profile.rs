#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub language_id: u32,
    pub build: Option<String>,
    pub run: String,
    pub test: Option<String>,
    pub init: Option<String>,
    pub filename: String,
}

impl Profile {
    pub fn new(
        name: &str,
        language_id: u32,
        build: &str,
        run: &str,
        test: &str,
        init: &str,
        filename: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            language_id,
            build: Some(build.to_owned()),
            run: run.to_owned(),
            test: Some(test.to_owned()),
            init: Some(init.to_owned()),
            filename: filename.to_owned(),
        }
    }
}

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
