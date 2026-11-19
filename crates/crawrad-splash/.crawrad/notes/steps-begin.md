---
id: steps-begin
kind: doc
target_file: src/lib.rs
line_hint: 90
author: 
created: 
title: "Steps::begin"
tags: []
signature: "pub fn begin (& mut self , label : & str , now : Instant)"
---

Closes the running step and starts `label`.

# Examples

```rust
use std::time::Instant;
use crawrad_splash::Steps;

let mut steps = Steps::default();
steps.begin("Step 1", Instant::now());
assert_eq!(steps.list().len(), 1);
```
