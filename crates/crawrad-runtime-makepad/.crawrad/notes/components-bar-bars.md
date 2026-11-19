---
id: components-bar-bars
kind: doc
target_file: src/components/bar.rs
line_hint: 210
author: 
created: 
title: "Bars"
tags: []
signature: "pub struct Bars"
---

Which bars a scrolling container shows, **and how**.

# Examples

```rust
use crawrad_runtime_makepad::components::bar::Bars;

let bars = Bars::default();
assert_eq!(bars, Bars::NONE);
```
