---
id: references-references-to-json
kind: doc
target_file: src/references.rs
line_hint: 201
author: 
created: 
title: "References::to_json"
tags: []
signature: "pub fn to_json (& self) -> serde_json :: Value"
---

Formats reference results and row metrics into a JSON representation.

What MCP reads (`editor.get_references`): the title and every file
with its marks and its occurrences (1-based lines).

# Examples

```rust
use crawrad_host::references::References;

let refs = References::default();
let json = refs.to_json();
assert_eq!(json["total"], 0);
```
