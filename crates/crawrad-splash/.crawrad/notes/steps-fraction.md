---
id: steps-fraction
kind: doc
target_file: src/lib.rs
line_hint: 125
author: 
created: 
title: "Steps::fraction"
tags: []
signature: "pub fn fraction (& self , expected : usize , shaders : ShaderCount) -> f32"
---

Calculates the progress fraction from 0.0 to 1.0.

0..=1 for the bar: finished steps out of `expected`, with the shader
counter filling the running one.

# Examples

```rust
use crawrad_splash::{ShaderCount, Steps};

let steps = Steps::default();
let frac = steps.fraction(3, ShaderCount::default());
assert_eq!(frac, 0.0);
```
