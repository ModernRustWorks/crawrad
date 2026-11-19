---
id: frameworks-badge
kind: doc
target_file: src/frameworks.rs
line_hint: 64
author: 
created: 
title: "badge"
tags: []
signature: "pub fn badge (f : & Framework) -> & 'static str"
---

The short badge of a row: empty when it can be chosen.

# Examples

```rust
use crawrad_project::frameworks::{badge, CATALOGUE};

assert_eq!(badge(&CATALOGUE[0]), "");
```
