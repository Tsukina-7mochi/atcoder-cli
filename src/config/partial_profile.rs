use serde::{Deserialize, Serialize};

use crate::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialProfile {
    pub build: Option<String>,
    pub run: Option<String>,
    pub test: Option<String>,
    pub init: Option<String>,
}

impl PartialProfile {
    pub fn new(build: &str, run: &str, test: &str, init: &str) -> Self {
        Self {
            build: Some(build.to_owned()),
            run: Some(run.to_owned()),
            test: Some(test.to_owned()),
            init: Some(init.to_owned()),
        }
    }

    pub fn merge_default(&self, another: &Profile) -> Profile {
        let build = self.build.as_ref().or(another.build.as_ref());
        let run = self.run.as_ref().unwrap_or(&another.run);
        let test = self.test.as_ref().or(another.test.as_ref());
        let init = self.init.as_ref().or(another.init.as_ref());

        Profile {
            name: another.name.to_owned(),
            build: build.map(|s| s.to_owned()),
            run: run.to_owned(),
            test: test.map(|s| s.to_owned()),
            init: init.map(|s| s.to_owned()),
        }
    }

    pub fn to_profile(self, name: &str) -> Option<Profile> {
        Some(Profile {
            name: name.to_owned(),
            build: self.build,
            run: self.run?,
            test: self.test,
            init: self.init,
        })
    }
}
