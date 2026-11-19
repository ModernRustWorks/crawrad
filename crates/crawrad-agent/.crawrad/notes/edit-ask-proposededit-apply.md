---
id: edit-ask-proposededit-apply
kind: doc
target_file: src/edit_ask.rs
line_hint: 54
author: 
created: 
title: "ProposedEdit::apply"
tags: []
signature: "pub fn apply(&self, current: &str) -> Result<String, String>"
---

Applies the change to the current text.

A Write replaces the whole text; replacements run in order.

# Errors

Returns an error when a text to replace is not found, instead of guessing.

# Examples

```rust
use crawrad_agent::ProposedEdit;
use serde_json::json;

let call = json!({ "rawInput": { "file_path": "a.rs", "old_string": "x", "new_string": "y" } });
let edit = ProposedEdit::from_tool_call(&call).unwrap();
assert_eq!(edit.apply("x x").unwrap(), "y x");
assert!(edit.apply("z").is_err());
```
