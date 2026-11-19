---
id: shadercount
kind: doc
target_file: src/lib.rs
line_hint: 38
author: 
created: 
title: "ShaderCount"
tags: []
signature: "pub struct ShaderCount"
---

Counts shaders ready, total, and compiled from source.

Shaders ready so far, of how many, and how many of those had to be
compiled (the rest came from the disk cache), as the graphics layer
reports them.

# Examples

```rust
use crawrad_splash::ShaderCount;

let count = ShaderCount { done: 5, total: 10, compiled: 2 };
assert_eq!(count.done, 5);
```
