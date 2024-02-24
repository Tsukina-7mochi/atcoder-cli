use scraper::Html;

use crate::util::scraper_element_text_content::TextContent;

use super::client;
use super::url;

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

pub fn get_contest_tasks(contest_name: &str) -> Vec<Task> {
    let url = url::contest_tasks(contest_name);
    let res = client::new_client().unwrap().get(url).send().unwrap();
    let body = res.text().unwrap();
    let document = Html::parse_document(&body);

    let task_names: Vec<_> = document
        .select(&selectors::TASK_NAME_ANCHOR)
        .map(|e| {
            e.attr("href")
                .unwrap()
                .trim_end_matches('/')
                .rsplit_once('/')
                .unwrap()
                .1
                .to_owned()
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

    tasks
}
