---
id: edit-ask-proposededit
kind: doc
target_file: src/edit_ask.rs
line_hint: 26
author: 
created: 
title: "ProposedEdit"
tags: []
signature: "pub struct ProposedEdit"
---

The file change an agent's edit tool asks for.

Kept from the permission request so the IDE shows it as a diff before allowing it.

# Examples

```rust
use crawrad_agent::ProposedEdit;
use serde_json::json;

let call = json!({ "rawInput": { "file_path": "a.rs", "old_string": "x", "new_string": "y" } });
let edit = ProposedEdit::from_tool_call(&call).unwrap();
assert_eq!(edit.path, std::path::PathBuf::from("a.rs"));
```
