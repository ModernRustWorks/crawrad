---
id: textline
kind: doc
target_file: src/lib.rs
line_hint: 212
author: 
created: 
title: "TextLine"
tags: []
signature: "pub enum TextLine"
---

Where the text of a single-line component is drawn.

# Examples

```rust
use crawrad_registry::TextLine;

let line = TextLine::Centered;
assert_eq!(line.line(0.0, 20.0), 10.0 + (TextLine::ASCENT - TextLine::DESCENT) / 2.0);
```
