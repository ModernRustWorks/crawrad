---
id: cargo-tool-form-unit
kind: doc
target_file: src/cargo_tool.rs
line_hint: 206
author: 
created: 
title: "form_unit"
tags: []
signature: "pub fn form_unit (name : & str) -> String"
---

`project.create_form`'s unit: the form's struct and its `new`.

# Examples

```rust
use crawrad_mcp::cargo_tool::form_unit;

let code = form_unit("MainForm");
assert!(code.contains("pub struct MainForm"));
```
