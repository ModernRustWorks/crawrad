---
id: semantic-span
kind: doc
target_file: src/semantic.rs
line_hint: 101
author: 
created: 
title: "Span"
tags: []
signature: "pub struct Span"
---

One classified semantic span within an editor line.

One classified span: line, start and end bytes, class (`None` keeps the
tokenizer's colour), and whether it is struck through as deprecated.

# Examples

```rust
use crawrad_lsp::semantic::{Class, Span};

let span = Span {
    line: 0,
    start: 4,
    end: 10,
    class: Some(Class::Macro),
    deprecated: false,
};
assert_eq!(span.line, 0);
```
