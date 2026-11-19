---
id: list-editorcomplete-to-json
kind: doc
target_file: src/list.rs
line_hint: 8
author: 
created: 
title: "EditorComplete::to_json"
tags: []
signature: "pub fn to_json (& self) -> serde_json :: Value"
---

The list as the MCP reads it (`editor.get_completion`).

# Examples

```rust
use crawrad_insight::list::EditorComplete;

let complete = EditorComplete::default();
let json = complete.to_json();
assert_eq!(json["open"], false);
```
