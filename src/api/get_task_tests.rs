use std::time::Duration;

use scraper::selector::CssLocalName;
use scraper::{Element, Html};

use crate::util::scraper_element_text_content::TextContent;

use super::url;

mod selectors {
    use once_cell::sync::Lazy;
    use scraper::Selector;

    pub const PARTS: Lazy<Selector> = Lazy::new(|| {
        Selector::parse("div#task-statement > span > span:first-child > div").unwrap()
    });

    pub const TESTS_PRE: Lazy<Selector> = Lazy::new(|| Selector::parse("pre").unwrap());
}

#[derive(Debug, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

pub fn get_task_tests(contest_name: &str, task_name: &str) -> Vec<Test> {
    let url = url::contest_task(contest_name, task_name);
    let res = ureq::get(&url)
        .timeout(Duration::from_secs(5))
        .call()
        .unwrap();
    let body = res.into_string().unwrap();
    let document = Html::parse_document(&body);

    let io_style_name = CssLocalName::from("io-style");
    let tests: Vec<_> = document
        .select(&selectors::PARTS)
        .skip_while(|el| {
            !el.has_class(
                &io_style_name,
                scraper::CaseSensitivity::AsciiCaseInsensitive,
            )
        })
        .skip(1)
        .map(|el| {
            let content = el
                .select(&selectors::TESTS_PRE)
                .next()
                .map(|el| el.text_content());
            content.unwrap().trim().to_owned()
        })
        .collect();

    assert!(tests.len() % 2 == 0);

    let tests: Vec<_> = (0..(tests.len() / 2))
        .map(|i| Test {
            input: tests[i * 2].to_owned(),
            output: tests[i * 2 + 1].to_owned(),
        })
        .collect();

    tests
}
