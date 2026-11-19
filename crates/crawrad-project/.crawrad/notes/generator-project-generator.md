---
id: generator-project-generator
kind: doc
target_file: src/generator.rs
line_hint: 145
author: 
created: 
title: "project_generator"
tags: []
signature: "pub fn project_generator () -> Box < dyn crawrad_codegen :: Gerador >"
---

The project's generator — **one place that chooses**.

# Examples

```rust
use crawrad_project::generator::project_generator;

let g = project_generator();
assert!(!g.alvo().is_empty());
```
