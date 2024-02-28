use std::time::Duration;

use scraper::Html;

use super::url;
use crate::api;
use crate::util::scraper_element_text_content::TextContent;

mod selectors {
    use once_cell::sync::Lazy;
    use scraper::Selector;

    pub const TASK_NAME_ANCHOR: Lazy<Selector> =
        Lazy::new(|| Selector::parse("tbody tr td:first-child a").unwrap());
    pub const TASK_LABEL: Lazy<Selector> =
        Lazy::new(|| Selector::parse("tbody tr td:nth-child(2)").unwrap());
}

pub struct Task {
    pub name: String,
    pub label: String,
}

pub fn get_contest_tasks(contest_name: &str) -> api::Result<Vec<Task>> {
    let url = url::contest_tasks(contest_name);
    let res = ureq::get(&url).timeout(Duration::from_secs(5)).call()?;
    let body = res.into_string()?;
    let document = Html::parse_document(&body);

    let task_names: Vec<_> = document
        .select(&selectors::TASK_NAME_ANCHOR)
        .filter_map(|e| -> Option<_> {
            e.attr("href")?
                .trim_end_matches('/')
                .rsplit_once('/')
                .map(|t| t.1.to_owned())
        })
        .collect();
    let task_labels: Vec<_> = document
        .select(&selectors::TASK_LABEL)
        .map(|e| e.text_content().trim().to_owned())
        .collect();

    let tasks: Vec<_> = task_names
        .into_iter()
        .zip(task_labels)
        .map(|(name, label)| Task { name, label })
        .collect();

    Ok(tasks)
}
