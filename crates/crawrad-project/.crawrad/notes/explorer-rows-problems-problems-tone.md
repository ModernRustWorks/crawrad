---
id: explorer-rows-problems-problems-tone
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 55
author: 
created: 
title: "Problems::tone"
tags: []
signature: "pub fn tone (& self) -> Option < & 'static str >"
---

The suffix of the row's template: an error wins over a warning.

# Examples

```rust
use crawrad_project::explorer_rows::problems::Problems;

let p = Problems { errors: 1, warnings: 0 };
assert_eq!(p.tone(), Some("Error"));
```
