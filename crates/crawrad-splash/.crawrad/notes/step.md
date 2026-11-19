---
id: step
kind: doc
target_file: src/lib.rs
line_hint: 75
author: 
created: 
title: "Step"
tags: []
signature: "pub struct Step"
---

One line of the list: its text and, once the next step began, how long it took.

# Examples

```rust
use crawrad_splash::Step;

let step = Step { label: "Init".into(), took: None };
assert_eq!(step.label, "Init");
```
