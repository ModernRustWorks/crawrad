---
id: edit-absolute-path-arg
kind: doc
target_file: src/edit.rs
line_hint: 88
author: 
created: 
title: "absolute_path_arg"
tags: []
signature: "pub fn absolute_path_arg (args : & Value , bases : & [Option < PathBuf >]) -> Option < Value >"
---

Rewrites a relative path argument to an absolute path against workspace bases.

When `args["path"]` is a non-empty relative path, resolves it against candidate
base directories (a `None` base, such as no open project, is skipped), choosing the first base where the path exists, or falling back
to the first base if none exists yet. Absolute, empty, or missing paths return
`None`.

# Examples

```rust
use crawrad_mcp::edit::absolute_path_arg;
use serde_json::json;
use std::path::PathBuf;

let args = json!({ "path": "src/lib.rs" });
let bases = [None, Some(PathBuf::from("."))];
let rewritten = absolute_path_arg(&args, &bases).unwrap();
assert!(rewritten["path"].as_str().unwrap().ends_with("src/lib.rs"));
```
