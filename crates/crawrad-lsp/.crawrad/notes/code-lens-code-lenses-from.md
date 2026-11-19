---
id: code-lens-code-lenses-from
kind: doc
target_file: src/code_lens.rs
line_hint: 85
author: 
created: 
title: "code_lenses_from"
tags: []
signature: "pub fn code_lenses_from (val : & serde_json :: Value) -> Vec < CodeLens >"
---

The lenses of a `textDocument/codeLens` answer: a list, or `null` for none.

# Examples

```rust
use crawrad_lsp::code_lens::code_lenses_from;
use serde_json::json;

let lenses = code_lenses_from(&json!([]));
assert!(lenses.is_empty());
```
