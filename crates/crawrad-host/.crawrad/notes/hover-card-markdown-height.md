---
id: hover-card-markdown-height
kind: doc
target_file: src/hover_card.rs
line_hint: 104
author: 
created: 
title: "markdown_height"
tags: []
signature: "pub fn markdown_height (markdown : & str , line_height : f64 , cols : usize , max : f64) -> f64"
---

Estimates the height needed to display rendered markdown.

The rich card's body height for `markdown`: its lines wrapped at
`cols`, a code block's fences and a heading counted as one more line
each, at most `max` (it scrolls past that).

# Examples

```rust
use crawrad_host::hover_card::markdown_height;

let h = markdown_height("hello", 14.0, 80, 200.0);
assert!(h > 0.0);
```
