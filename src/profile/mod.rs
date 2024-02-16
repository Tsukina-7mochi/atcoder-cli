pub mod defaults;

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub build: Option<String>,
    pub run: String,
    pub test: Option<String>,
    pub init: Option<String>,
}

impl Profile {
    pub fn new(name: &str, build: &str, run: &str, test: &str, init: &str) -> Self {
        Self {
            name: name.to_owned(),
            build: Some(build.to_owned()),
            run: run.to_owned(),
            test: Some(test.to_owned()),
            init: Some(init.to_owned()),
        }
    }
}
