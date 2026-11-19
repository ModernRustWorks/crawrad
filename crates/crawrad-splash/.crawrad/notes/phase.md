---
id: phase
kind: doc
target_file: src/lib.rs
line_hint: 63
author: 
created: 
title: "phase"
tags: []
signature: "pub fn phase (shaders : ShaderCount , before : usize) -> usize"
---

Determines the current splash phase from the shader counter.

Which step the shader counter says the start is in. Never goes back:
a later batch of shaders does not reopen a finished step.

# Examples

```rust
use crawrad_splash::{phase, ShaderCount};

let shaders = ShaderCount { done: 0, total: 0, compiled: 0 };
assert_eq!(phase(shaders, 0), 0);
```
