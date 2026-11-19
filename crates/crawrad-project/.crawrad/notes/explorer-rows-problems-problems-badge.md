---
id: explorer-rows-problems-problems-badge
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 64
author: 
created: 
title: "Problems::badge"
tags: []
signature: "pub fn badge (& self , word : & str) -> String"
---

The badge with the count before it: `2 · Yours`, or `2` alone.

# Examples

```rust
use crawrad_project::explorer_rows::problems::Problems;

let p = Problems { errors: 2, warnings: 0 };
assert_eq!(p.badge("Yours"), "2 \u{b7} Yours");
```
