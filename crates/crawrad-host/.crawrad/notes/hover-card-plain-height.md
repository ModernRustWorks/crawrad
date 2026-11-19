---
id: hover-card-plain-height
kind: doc
target_file: src/hover_card.rs
line_hint: 116
author: 
created: 
title: "plain_height"
tags: []
signature: "pub fn plain_height (text : & str , line_height : f64 , cols : usize) -> f64"
---

The plain card's height for `text`, wrapped at `cols` with margins.

The plain card's height for `text`: its lines wrapped at `cols`, and a
margin.

# Examples

```rust
use crawrad_host::hover_card::plain_height;

let h = plain_height("hello", 14.0, 80);
assert!(h >= 16.0);
```
