use std::time::Duration;

use scraper::{Element, Html};

use super::url;
use crate::api::{self, error::GetTaskTestsErrorKind};
use crate::util::scraper_element_text_content::TextContent;

mod selectors {
    use once_cell::sync::Lazy;
    use scraper::Selector;

    pub const PARTS: Lazy<Selector> = Lazy::new(|| {
        Selector::parse("div#task-statement > span > span:first-child > div").unwrap()
    });
    pub const TESTS_PRE: Lazy<Selector> = Lazy::new(|| Selector::parse("pre").unwrap());
}

mod class_names {
    use once_cell::sync::Lazy;
    use scraper::selector::CssLocalName;

    pub const IO_STYLE: Lazy<CssLocalName> = Lazy::new(|| "io-style".into());
}

#[derive(Debug, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

pub fn get_task_tests(contest_name: &str, task_name: &str) -> api::Result<Vec<Test>> {
    let url = url::contest_task(contest_name, task_name);
    let res = ureq::get(&url).timeout(Duration::from_secs(5)).call()?;
    let body = res.into_string()?;
    let document = Html::parse_document(&body);

    let tests: Vec<_> = document
        .select(&selectors::PARTS)
        .skip_while(|el| {
            !el.has_class(
                &class_names::IO_STYLE,
                scraper::CaseSensitivity::AsciiCaseInsensitive,
            )
        })
        .skip(1)
        .filter_map(|el| -> Option<_> {
            el.select(&selectors::TESTS_PRE)
                .next()
                .map(|el| el.text_content())
                .map(|s| s.trim().to_owned())
        })
        .collect();

    if tests.len() % 2 != 0 {
        return Err(GetTaskTestsErrorKind::IncorrectTestNumber(tests.len()).into());
    }

    let tests: Vec<_> = (0..(tests.len() / 2))
        .map(|i| Test {
            input: tests[i * 2].to_owned(),
            output: tests[i * 2 + 1].to_owned(),
        })
        .collect();

    Ok(tests)
}
