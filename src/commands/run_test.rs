use std::io::{self, Read};
use std::path::Path;

use subprocess::{Exec, Redirection};

use crate::api;

enum Tests {
    Manual(String),
    Auto(Vec<api::get_task_tests::Test>),
}

pub fn run_test(
    cwd: &Path,
    build_command: Option<&str>,
    run_command: &str,
    contest_task_name: Option<(&str, &str)>,
) -> () {
    let tests: Tests = if let Some((contest_name, task_name)) = contest_task_name {
        // fetch remote
        let tests = api::get_task_tests::get_task_tests(contest_name, task_name);
        Tests::Auto(tests)
    } else {
        // read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        Tests::Manual(buffer)
    };

    if let Some(build_command) = build_command {
        println!("Building...");
        let result = Exec::shell(build_command).cwd(cwd).capture().unwrap();
        if !result.success() {
            println!("Build failed: {:?}", result.exit_status);
            return;
        }
    }

    match tests {
        Tests::Manual(input) => {
            let result = Exec::shell(run_command)
                .cwd(cwd)
                .stdin(input.as_str())
                .stdout(Redirection::Pipe)
                .capture()
                .unwrap();
            if !result.success() {
                println!("Process failed: {:?}", result.exit_status);
                return;
            }
            println!("{}", result.stdout_str());
        }
        Tests::Auto(tests) => {
            // TODO: implement parallel tests
            let mut results: Vec<(usize, bool)> = vec![];
            for (i, test) in tests.iter().enumerate() {
                println!("Running test {}", i + 1);

                let result = Exec::shell(run_command)
                    .cwd(cwd)
                    .stdin(test.input.as_str())
                    .stdout(Redirection::Pipe)
                    .capture()
                    .unwrap();
                if !result.success() {
                    println!("Process failed: {:?}", result.exit_status);
                    results.push((i, false));
                    continue;
                }

                let actual = result.stdout_str();
                let actual = actual.trim();
                let expected = test.output.trim();
                if actual == expected {
                    println!("Test {}..Success", i + 1);
                    results.push((i, true));
                } else {
                    println!("Test {}..Failure", i + 1);
                    if actual.contains('\n') || expected.contains('\n') {
                        println!("Actual:\n{}\n\nExpected:\n{}\n", actual, expected);
                    } else if actual.len() > 10 || expected.len() > 10 {
                        println!("Actual  : {}\nExpected: {}", actual, expected);
                    } else {
                        println!("Actual: {}, Expected: {}", actual, expected);
                    }
                    results.push((i, false));
                }
            }

            println!("== Test results ==");
            for (i, succeeded) in results {
                let res = if succeeded { "Success" } else { "Failure" };
                println!("Test {}..{}", i + 1, res);
            }
        }
    }
}
