## グローバル設定ファイル

デフォルトで `build`, `run` が設定されていて、上書き可能

```yaml
# ~/.atcoder.yml
version: "1"
profiles:
  rust:
    init: "cargo init ${taskName}"
    build: "cargo build --release --quit --offline"
    run: "./target/release/${taskName}"
```

## タスク設定ファイル

cwd から遡って探索

```yaml
# atcoder.yml
version: "1"
contest: "abs"
task: "practice_1"
profile: "rust"
# ここで直接プロファイル書けるようにもしたい
```

## 機能

### info コマンド

グローバル・スコープの状況を出力

```sh
$ atcoder info
# Global configuration: /home/user/.atcoder.yml
# Task configuration: /home/user/atcoder/abs/practice_1/atcoder.yml
#   Contest: abs
#   Task   : practice_1
#   Profile: rust
```

### init コマンド

`profile.init` を使って初期化

```sh
# タスクディレクトリの初期化
# path のデフォルト: <contest>/<task>
$ atcoder init <profile> <contest> <task> [path]
$ atcoder init <profile> <contest> * [path]
```

### run コマンド

テストケースを fetch し、ビルドして実行

- `--manual` オプション: 手入力

1. `profile.build` (あれば)
2. `profile.run` (必須)

### test コマンド

テストケースを fetch し、ビルドして実行 (`profile.test` に指定されたコマンドを使用)

1. `profile.test` (必須)

### url コマンド

コンテストの URL を表示

```sh
$ atcoder url
# Contest page: https://atcoder.jp/contests/abs
```

### login コマンド

ログイン (cookie の取得)

```sh
$ atcoder login
# User name >
# Password >
# Login succeeded
```

### submit コマンド

提出

```sh
$ atcoder submit
# Submit succeeded
# Your submissions: https://atcoder.jp/contests/abs/submissions/me
```
