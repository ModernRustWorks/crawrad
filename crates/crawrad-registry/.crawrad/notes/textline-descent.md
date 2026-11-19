---
id: textline-descent
kind: doc
target_file: src/lib.rs
line_hint: 227
author: 
created: 
title: "TextLine::DESCENT"
tags: []
signature: "pub const DESCENT : f32 = Self :: TEXT_HEIGHT - Self :: ASCENT"
---

From baseline to descender of "g", "p".

# Examples

```rust
use crawrad_registry::TextLine;

assert_eq!(TextLine::DESCENT, 3.0);
```
