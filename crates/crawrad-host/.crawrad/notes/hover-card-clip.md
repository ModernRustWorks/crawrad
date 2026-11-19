---
id: hover-card-clip
kind: doc
target_file: src/hover_card.rs
line_hint: 125
author: 
created: 
title: "clip"
tags: []
signature: "pub fn clip (text : & str , max : usize) -> String"
---

At most `max` lines of `text`; a cut card ends in `…`.

# Examples

```rust
use crawrad_host::hover_card::clip;

assert_eq!(clip("a\nb\nc", 2), "a\nb\n…");
```
