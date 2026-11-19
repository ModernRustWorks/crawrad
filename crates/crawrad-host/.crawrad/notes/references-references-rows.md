---
id: references-references-rows
kind: doc
target_file: src/references.rs
line_hint: 156
author: 
created: 
title: "References::rows"
tags: []
signature: "pub fn rows (& self) -> Vec < Row >"
---

The rows of the list: each file, and under an open one its hits.

# Examples

```rust
use crawrad_host::references::References;

let refs = References::default();
assert!(refs.rows().is_empty());
```
