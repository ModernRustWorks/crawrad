---
id: edit-ask-proposededit-from-tool-call
kind: doc
target_file: src/edit_ask.rs
line_hint: 34
author: 
created: 
title: "ProposedEdit::from_tool_call"
tags: []
signature: "pub fn from_tool_call(tool_call: &serde_json::Value) -> Option<Self>"
---

Reads the edit an ACP tool call asks for.

Takes `rawInput.file_path` with `content` (Write), `old_string`/`new_string` (Edit)
or `edits` (MultiEdit). `None` when the call is not a file edit.

# Examples

```rust
use crawrad_agent::ProposedEdit;
use serde_json::json;

assert!(ProposedEdit::from_tool_call(&json!({ "rawInput": { "command": "ls" } })).is_none());
```
