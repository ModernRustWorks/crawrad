---
id: steps-current-line
kind: doc
target_file: src/lib.rs
line_hint: 106
author: 
created: 
title: "Steps::current_line"
tags: []
signature: "pub fn current_line (& self , shaders : ShaderCount) -> String"
---

Formats the label for the currently running step.

The running step as the splash prints it: the shader counter rides on
the step that waits for it.

# Examples

```rust
use std::time::Instant;
use crawrad_splash::{ShaderCount, Steps};

let mut steps = Steps::default();
steps.begin("Compiling", Instant::now());
let count = ShaderCount { done: 1, total: 2, compiled: 1 };
assert_eq!(steps.current_line(count), "Compiling  1 / 2");
```
