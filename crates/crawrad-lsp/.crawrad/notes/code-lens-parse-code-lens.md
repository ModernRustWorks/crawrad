---
id: code-lens-parse-code-lens
kind: doc
target_file: src/code_lens.rs
line_hint: 70
author: 
created: 
title: "parse_code_lens"
tags: []
signature: "pub fn parse_code_lens (val : & serde_json :: Value) -> Option < CodeLens >"
---

Parses a single lens from server JSON.

# Examples

```rust
use crawrad_lsp::code_lens::parse_code_lens;
use serde_json::json;

let raw = json!({
    "range": { "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 1 } }
});
let lens = parse_code_lens(&raw);
assert!(lens.is_some());
```
