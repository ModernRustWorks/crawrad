---
id: creation-cargo-with-output
kind: doc
target_file: src/creation.rs
line_hint: 265
author: 
created: 
title: "cargo_with_output"
tags: []
signature: "pub fn cargo_with_output (args : & [String] , at : & Path) -> Result < String , String >"
---

Runs cargo and returns its merged output or first line of error.

Runs `cargo` and returns what it **wrote to screen**, or its first line of error.

Output matters, and this is new: when `cargo new` edits someone else's manifest,
it announces

```text
      Adding `x` as member of workspace at `C:\dev\some-repo`
```

and that line is the only proof that this happened. Prior to this, output of a
successful cargo run was discarded.

`stderr` and `stdout` merged because cargo writes progress to `stderr` — that is
where `Creating`, `Adding`, and `error` appear — and relying on which one for an
informative line would rely on a detail that has changed before. What keeps both
WHOLE is [`process::Outcome`]; here they are only joined.

**Output returns without color** (issue #63), and [`process::CargoCall::command`]
explains why.

# Examples

```rust,no_run
// Does not spawn processes during tests.
use std::path::Path;
use crawrad_project::creation::cargo_with_output;

let args = vec!["--version".to_string()];
let result = cargo_with_output(&args, Path::new("."));
assert!(result.is_ok());
```

# Errors

Returns `Err` with the first error line if cargo fails or the process cannot be executed.
