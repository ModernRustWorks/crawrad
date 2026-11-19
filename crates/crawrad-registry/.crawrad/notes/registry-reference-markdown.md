---
id: registry-reference-markdown
kind: doc
target_file: src/reference.rs
line_hint: 13
author: 
created: 
title: "reference_markdown"
tags: []
signature: "pub fn reference_markdown (& self) -> String"
---

The manual page listing every palette component, its properties and events.

# Examples

```rust
use crawrad_registry::Registry;

let md = Registry::builtin().reference_markdown();
assert!(md.starts_with("# Components and properties"));
```
