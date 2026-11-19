---
id: resolve-documentation
kind: doc
target_file: src/resolve.rs
line_hint: 69
author: 
created: 
title: "documentation"
tags: []
signature: "pub fn documentation (doc : & Value) -> Option < String >"
---

Extracts plain documentation text for the completion panel.

A `documentation` (a string, or `MarkupContent`) made plain for the
panel; `None` when empty.

# Examples

```rust
use crawrad_lsp::resolve::documentation;
use serde_json::json;

let val = json!("Returns the length of this string.");
assert_eq!(
    documentation(&val),
    Some("Returns the length of this string.".to_string())
);
```
