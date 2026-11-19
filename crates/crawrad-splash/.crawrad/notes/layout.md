---
id: layout
kind: doc
target_file: src/lib.rs
line_hint: 157
author: 
created: 
title: "layout"
tags: []
signature: "pub fn layout (steps : & Steps , shaders : ShaderCount) -> Frame"
---

Everything painted over the picture, for these steps and this counter.

# Examples

```rust
use crawrad_splash::{layout, ShaderCount, Steps};

let steps = Steps::default();
let frame = layout(&steps, ShaderCount::default());
assert!(!frame.texts.is_empty());
```
