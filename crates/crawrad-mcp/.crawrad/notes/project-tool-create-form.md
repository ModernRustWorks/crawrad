---
id: project-tool-create-form
kind: doc
target_file: src/project_tool.rs
line_hint: 35
author: 
created: 
title: "create_form"
tags: []
signature: "pub fn create_form (root : Option < PathBuf > , args : & Value) -> (Answer , Option < PathBuf >)"
---

Creates a form document and companion unit file in the project folder.

`project.create_form`: the `.crawform` and its unit under `folder`; the
form's path too, for the shell to open in the designer.

# Examples

```rust
use crawrad_mcp::project_tool::create_form;
use serde_json::json;

let (ans, path) = create_form(None, &json!({ "name": "MainForm" }));
assert_eq!(ans.1, "error");
assert!(path.is_none());
```
