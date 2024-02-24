pub mod defaults;

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
