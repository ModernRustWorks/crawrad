---
id: highlight-caret
kind: doc
target_file: src/highlight.rs
line_hint: 86
author: 
created: 
title: "Caret"
tags: []
signature: "pub type Caret = (usize , usize)"
---

Where the caret is, in the editor's units: zero-based line and byte.

# Examples

```rust
use crawrad_lsp::highlight::Caret;

let caret: Caret = (0, 4);
assert_eq!(caret.0, 0);
```
