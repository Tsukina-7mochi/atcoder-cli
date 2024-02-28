use std::io::{self, Read};
use std::path::Path;

use subprocess::{Exec, Redirection};

use crate::api;
use crate::commands;
use crate::commands::error::RunTestErrorKind;

enum Tests {
    Manual(String),
    Auto(Vec<api::get_task_tests::Test>),
}

struct TestResult {
    index: usize,
    kind: TestResultKind,
}

enum TestResultKind {
    Success,
    Failure { actual: String, expected: String },
    Error(RunTestErrorKind),
}

pub fn run_test(
    cwd: &Path,
    build_command: Option<&str>,
    run_command: &str,
    contest_task_name: Option<(&str, &str)>,
) -> commands::Result {
    let tests: Tests = if let Some((contest_name, task_name)) = contest_task_name {
        // fetch remote
        let tests = api::get_task_tests::get_task_tests(contest_name, task_name)?;
        Tests::Auto(tests)
    } else {
        // read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Tests::Manual(buffer)
    };

    if let Some(build_command) = build_command {
        println!("Building...");
        let result = Exec::shell(build_command)
            .cwd(cwd)
            .capture()
            .map_err(|err| RunTestErrorKind::BuildCommandFailed(err.into()))?;
        if !result.success() {
            return Err(RunTestErrorKind::BuildCommandFailed(result.exit_status.into()).into());
        }
    }

    match tests {
        Tests::Manual(input) => {
            let result = Exec::shell(run_command)
                .cwd(cwd)
                .stdin(input.as_str())
                .stdout(Redirection::Pipe)
                .capture()
                .map_err(|err| RunTestErrorKind::RunCommandFailed(err.into()))?;
            if !result.success() {
                return Err(RunTestErrorKind::RunCommandFailed(result.exit_status.into()).into());
            }
            println!("{}", result.stdout_str());
        }
        Tests::Auto(tests) => {
            // TODO: implement parallel tests
            let mut results: Vec<TestResult> = vec![];
            for (index, test) in tests.iter().enumerate() {
                println!("Running test {}", index + 1);

                let result = Exec::shell(run_command)
                    .cwd(cwd)
                    .stdin(test.input.as_str())
                    .stdout(Redirection::Pipe)
                    .capture()
                    .map_err(|err| RunTestErrorKind::RunCommandFailed(err.into()));

                let kind = match result {
                    Err(err) => TestResultKind::Error(err),
                    Ok(result) => {
                        if !result.success() {
                            TestResultKind::Error(RunTestErrorKind::RunCommandFailed(
                                result.exit_status.into(),
                            ))
                        } else {
                            let actual = result.stdout_str();
                            let actual = actual.trim().to_owned();
                            let expected = test.output.trim().to_owned();
                            if actual == expected {
                                TestResultKind::Success
                            } else {
                                TestResultKind::Failure { actual, expected }
                            }
                        }
                    }
                };

                match kind {
                    TestResultKind::Success => println!("Test {}..Success", index + 1),
                    TestResultKind::Failure {
                        ref actual,
                        ref expected,
                    } => {
                        println!("Test {}..Failure", index + 1);
                        if actual.contains('\n') || expected.contains('\n') {
                            println!("Actual:\n{}\n\nExpected:\n{}\n", actual, expected);
                        } else if actual.len() > 10 || expected.len() > 10 {
                            println!("Actual  : {}\nExpected: {}", actual, expected);
                        } else {
                            println!("Actual: {}, Expected: {}", actual, expected);
                        }
                    }
                    TestResultKind::Error(ref err) => {
                        println!("Test {}..Error: {}", index + 1, &err);
                    }
                }

                results.push(TestResult { index, kind });
            }

            println!("== Test results ==");
            results.sort_by_key(|r| r.index);
            for r in results {
                let res = match r.kind {
                    TestResultKind::Success => "Success",
                    TestResultKind::Failure { .. } => "Failure",
                    TestResultKind::Error(_) => "Error",
                };
                println!("Test {}..{}", r.index + 1, res);
            }
        }
    }

    Ok(())
}
