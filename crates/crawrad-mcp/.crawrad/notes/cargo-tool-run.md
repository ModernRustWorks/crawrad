---
id: cargo-tool-run
kind: doc
target_file: src/cargo_tool.rs
line_hint: 19
author: 
created: 
title: "run"
tags: []
signature: "pub fn run (dir : & Path , args : & Value) -> Result < (Value , String , bool) , String >"
---

Runs cargo in a project directory.

Runs cargo in `dir`: the JSON answer, the sentence, and whether it
succeeded; `Err` when cargo could not be started.

# Examples

```rust,no_run
// Spawns cargo: compiled here, never run as a doctest.
use crawrad_mcp::cargo_tool::run;
use std::path::Path;

let res = run(Path::new("."), &serde_json::json!({}));
assert!(res.is_ok(), "cargo could not start");
```

# Errors

Returns an error message if the cargo process cannot be spawned.
