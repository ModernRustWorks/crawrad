---
id: project-tool-create-args
kind: doc
target_file: src/project_tool.rs
line_hint: 111
author: 
created: 
title: "create_args"
tags: []
signature: "pub fn create_args (args : & Value) -> Result < (& str , & str , & str , bool , Option < & str >) , Answer >"
---

Parses arguments for creating a new project.

`project.create`'s arguments: the name, the folder (`folder`, or its old
name `path` when `folder` is not given), the template (`gui_app`),
`git_init`, and optional `framework`.

# Examples

```rust
use crawrad_mcp::project_tool::create_args;
use serde_json::json;

let args = json!({ "name": "demo", "folder": "/tmp/demo" });
let parsed = create_args(&args).unwrap();
assert_eq!(parsed.0, "demo");
assert_eq!(parsed.1, "/tmp/demo");
```

# Errors

Returns `Err` with an error answer if the `name` or `folder`/`path` argument is missing or empty.
