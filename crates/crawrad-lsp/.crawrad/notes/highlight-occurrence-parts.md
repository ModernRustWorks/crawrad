---
id: highlight-occurrence-parts
kind: doc
target_file: src/highlight.rs
line_hint: 50
author: 
created: 
title: "Occurrence::parts"
tags: []
signature: "pub fn parts (& self) -> (usize , usize , usize , bool)"
---

`(line, start, end, write)` — the editor's own shape for it.

# Examples

```rust
use crawrad_lsp::highlight::Occurrence;

let occ = Occurrence { line: 1, start: 2, end: 6, write: true };
assert_eq!(occ.parts(), (1, 2, 6, true));
```
