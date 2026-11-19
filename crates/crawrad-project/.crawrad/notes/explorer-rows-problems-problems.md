---
id: explorer-rows-problems-problems
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 13
author: 
created: 
title: "Problems"
tags: []
signature: "pub struct Problems"
---

Errors and warnings, counted.

# Examples

```rust
use crawrad_project::explorer_rows::problems::Problems;

let p = Problems { errors: 1, warnings: 2 };
assert_eq!(p.errors, 1);
```
