pub fn atcoder() -> String {
    "https://atcoder.jp".to_owned()
}

pub fn contests() -> String {
    atcoder() + "/contests"
}

pub fn contest(contest_name: &str) -> String {
    contests() + "/" + contest_name
}

pub fn contest_tasks(contest_name: &str) -> String {
    contest(contest_name) + "/tasks"
}

pub fn contest_task(contest_name: &str, task_name: &str) -> String {
    contest_tasks(contest_name) + "/" + task_name
}

pub fn contest_submissions(contest_name: &str) -> String {
    contest(contest_name) + "/"
}
