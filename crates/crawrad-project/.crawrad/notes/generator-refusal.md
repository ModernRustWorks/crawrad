---
id: generator-refusal
kind: doc
target_file: src/generator.rs
line_hint: 78
author: 
created: 
title: "refusal"
tags: []
signature: "pub fn refusal (reason : impl std :: fmt :: Display) -> String"
---

Formats a refusal sentence for the status bar.

A refusal sentence, for the status bar. **The only place the prefix is
written.**

# Examples

```rust
use crawrad_project::generator::refusal;

assert_eq!(refusal("error"), "generate: error");
```
