# AtCoder CLI

The AtCoder Command Line Interface (CLI) is a powerful tool designed to enhance your performance on [AtCoder](atcoder.jp) by enabling faster setup, testing, and submission processes.

## Features

- **Effortless Workspace Initialization**: Initialize task workspaces with just one command.
- **Automatic Sample Test Case Retrieval**: Fetch sample test cases automatically.
- **Efficient Source Code Submission**: Submit your source code directly from the command line.

## Commands

### info

Retrieve information about the current working directory.

```console
$ atcoder info
Global configuration: /home/user/.atcoder.yml
Task configuration: /home/user/atcoder/abs/practice_1/atcoder.yml
  Contest: abs
  Task   : practice_1
  Profile: rust
```

### init

Initialize a workspace based on a specified profile.

```console
$ # if [path] is not specified, <contest>/<task> will be used.
$ atcoder init <profile> <contest> <task> [path]
$ # fetch all tasks and initialize for each.
$ atcoder init <profile> <contest>
```

### run

Build the application and execute sample test cases automatically (or use --manual for manual testing).

```console
$ atcoder run
Building...
    Finished release [optimized] target(s) in 0.02s
Running test 1
Test 1..Success
Running test 2
Test 2..Success
== Test results ==
Test 1..Succeess
Test 2..Succeess
```

### test

Fetch and execute sample test cases, skipping the build process (or use --manual for manual testing).

```console
$ atcoder test
Running test 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target/debug/practice_1`
Test 1..Success
Running test 2
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target/debug/practice_1`
Test 2..Success
== Test results ==
Test 1..Success
Test 2..Success
```

### url

Show the URL of the contest page.

```console
$ atcoder url
Contest page: https://atcoder.jp/contests/abs
```

### login

Authenticate with AtCoder using your username and password to obtain session information.

```console
$ atcoder login
username> foo
password>
Login succeeded.

$ # use --env-session to print session information instead of storing to keyring
$ export ATCODER_SESSION=$(atcoder --env-session login)
```

### submit

Submit your source code. Sign-in is required.

```console
$ atcoder submit
Submit succeeded

$ # use --env-session to load session information from ATCODER_SESSION
$ atcoder --env-session submit
```

## Configuration Files

Default profiles are provided, but you can customize or add your own profiles in the configuration files.

### Global configuration

```yaml
# ~/.atcoder.yml
version: "1"
profiles:
  rust:
    init: "cargo init ${taskName}"
    build: "cargo build --release --quit --offline"
    run: "./target/release/${taskName}"
```

### Workspace configuration

```yaml
# atcoder.yml
version: "1"
contest: "abs"
task: "practice_1"
profile: "rust"
```
