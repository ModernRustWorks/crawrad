---
id: generator-plan
kind: doc
target_file: src/generator.rs
line_hint: 248
author: 
created: 
title: "Plan"
tags: []
signature: "pub struct Plan"
---

Everything decided about one generation, before a single byte is written.

# Examples

```rust
use crawrad_project::generator::Plan;

let plan = Plan {
    saved: None,
    expired: Vec::new(),
    files: Ok(Vec::new()),
};
assert!(plan.expired.is_empty());
```
