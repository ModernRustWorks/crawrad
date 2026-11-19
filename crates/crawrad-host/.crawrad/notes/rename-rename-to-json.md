---
id: rename-rename-to-json
kind: doc
target_file: src/rename.rs
line_hint: 216
author: 
created: 
title: "Rename::to_json"
tags: []
signature: "pub fn to_json (& self) -> serde_json :: Value"
---

What MCP reads (`editor.get_rename`).

# Examples

```rust
use crawrad_host::rename::Rename;

let rename = Rename::default();
let json = rename.to_json();
assert_eq!(json["open"], false);
```
