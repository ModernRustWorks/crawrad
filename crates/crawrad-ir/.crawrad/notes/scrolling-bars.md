---
id: scrolling-bars
kind: doc
target_file: src/scrolling.rs
line_hint: 62
author: 
created: 
title: "Bars"
tags: []
signature: "pub struct Bars"
---

Which bars a container shows this frame.

# Examples

```rust
use crawrad_ir::scrolling::Bars;

let bars = Bars::default();
assert!(!bars.vertical);
assert!(!bars.horizontal);
```
