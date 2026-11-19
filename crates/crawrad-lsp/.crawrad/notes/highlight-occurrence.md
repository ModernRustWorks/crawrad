---
id: highlight-occurrence
kind: doc
target_file: src/highlight.rs
line_hint: 40
author: 
created: 
title: "Occurrence"
tags: []
signature: "pub struct Occurrence"
---

One symbol occurrence on a line expressed in editor units.

One occurrence on a line, in the editor's units: zero-based line, byte
range in the line, and whether the symbol is written there.

# Examples

```rust
use crawrad_lsp::highlight::Occurrence;

let occ = Occurrence { line: 0, start: 0, end: 5, write: false };
assert_eq!(occ.line, 0);
```
