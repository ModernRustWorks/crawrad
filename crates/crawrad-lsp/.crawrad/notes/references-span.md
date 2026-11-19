---
id: references-span
kind: doc
target_file: src/references.rs
line_hint: 108
author: 
created: 
title: "Span"
tags: []
signature: "pub struct Span"
---

One reference occurrence represented in editor coordinates.

One reference in the editor's unit: the file, the line (0-based), the
symbol's bytes in that line and the line's text.

# Examples

```rust
use crawrad_lsp::references::Span;
use std::path::PathBuf;

let span = Span {
    file: PathBuf::from("src/main.rs"),
    line: 0,
    start: 3,
    end: 7,
    text: "fn main() {}".to_string(),
};
assert_eq!(span.line, 0);
```
