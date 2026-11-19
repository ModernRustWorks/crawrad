---
id: generator-target-key
kind: doc
target_file: src/generator.rs
line_hint: 107
author: 
created: 
title: "target_key"
tags: []
signature: "pub fn target_key () -> & 'static str"
---

Key prefix of the target in force for filtering components.

The key prefix of the target in force (`makepad` / `egui`), what the
Inspector and the palette filter by (#491).

# Examples

```rust
use crawrad_project::generator::target_key;

assert!(!target_key().is_empty());
```
