---
id: references-references-title
kind: doc
target_file: src/references.rs
line_hint: 138
author: 
created: 
title: "References::title"
tags: []
signature: "pub fn title (& self) -> String"
---

The panel's title line.

# Examples

```rust
use crawrad_host::references::References;

let refs = References::default();
assert!(refs.title().contains("Find All References"));
```
