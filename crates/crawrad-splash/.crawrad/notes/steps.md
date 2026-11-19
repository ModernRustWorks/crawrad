---
id: steps
kind: doc
target_file: src/lib.rs
line_hint: 82
author: 
created: 
title: "Steps"
tags: []
signature: "pub struct Steps"
---

The steps shown so far, with their times.

# Examples

```rust
use crawrad_splash::Steps;

let steps = Steps::default();
assert!(steps.list().is_empty());
```
