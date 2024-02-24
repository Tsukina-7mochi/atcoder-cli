use serde::{Deserialize, Serialize};

use crate::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialProfile {
    pub build: Option<String>,
    pub language_id: Option<u32>,
    pub run: Option<String>,
    pub test: Option<String>,
    pub init: Option<String>,
    pub filename: Option<String>,
}

impl PartialProfile {
    pub fn merge_default(&self, another: &Profile) -> Profile {
        let build = self.build.as_ref().or(another.build.as_ref());
        let language_id = self.language_id.unwrap_or(another.language_id);
        let run = self.run.as_ref().unwrap_or(&another.run);
        let test = self.test.as_ref().or(another.test.as_ref());
        let init = self.init.as_ref().or(another.init.as_ref());
        let filename = self.filename.as_ref().unwrap_or(&another.filename);

        Profile {
            name: another.name.to_owned(),
            language_id,
            build: build.map(|s| s.to_owned()),
            run: run.to_owned(),
            test: test.map(|s| s.to_owned()),
            init: init.map(|s| s.to_owned()),
            filename: filename.to_owned(),
        }
    }

    pub fn to_profile(self, name: &str) -> Option<Profile> {
        Some(Profile {
            name: name.to_owned(),
            language_id: self.language_id?,
            build: self.build,
            run: self.run?,
            test: self.test,
            init: self.init,
            filename: self.filename?,
        })
    }
}
