---
id: drawing-textsource
kind: doc
target_file: src/drawing.rs
line_hint: 85
author: 
created: 
title: "TextSource"
tags: []
signature: "pub enum TextSource"
---

Where the text of a verb that writes comes from.

**Where the text of a verb that writes comes from.**

Either it is a manifest constant, or it is a component property — and the
second is what makes the drawing follow what the user types in the Object
Inspector, instead of always showing the same word.

# Examples

```rust
use crawrad_ir::drawing::TextSource;

let fixed = TextSource::Fixed("OK".to_string());
assert_eq!(fixed, TextSource::Fixed("OK".to_string()));
```
