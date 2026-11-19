---
id: code-lens-rows
kind: doc
target_file: src/code_lens.rs
line_hint: 120
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (lenses : & [CodeLens]) -> Vec < (usize , String) >"
---

One (line, title) per lens that has a command, in order.

# Examples

```rust
use crawrad_lsp::code_lens::rows;

let res = rows(&[]);
assert!(res.is_empty());
```
