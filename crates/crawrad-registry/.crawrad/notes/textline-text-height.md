---
id: textline-text-height
kind: doc
target_file: src/lib.rs
line_hint: 223
author: 
created: 
title: "TextLine::TEXT_HEIGHT"
tags: []
signature: "pub const TEXT_HEIGHT : f32 = 14.0"
---

Default font height in design units for text alignment.

Default font height in design units — enough for
alignment; exact measurement is runtime work, not the designer's.

# Examples

```rust
use crawrad_registry::TextLine;

assert_eq!(TextLine::TEXT_HEIGHT, 14.0);
```
