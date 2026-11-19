---
id: code-lens-json
kind: doc
target_file: src/code_lens.rs
line_hint: 149
author: 
created: 
title: "json"
tags: []
signature: "pub fn json (lenses : & [CodeLens] , bands : & [(usize , [f64 ; 4])]) -> serde_json :: Value"
---

Formats code lenses as a JSON array for display and MCP inspection.

Formats the lenses as a JSON array for MCP; `bands` are the rows on
screen, `(line, [x, y, w, h])`, and each lens carries its line's (or null).

# Examples

```rust
use crawrad_lsp::code_lens::json;

let val = json(&[], &[]);
assert!(val.as_array().unwrap().is_empty());
```
