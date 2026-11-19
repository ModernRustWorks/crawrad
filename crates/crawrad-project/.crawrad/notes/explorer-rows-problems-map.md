---
id: explorer-rows-problems-map
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 33
author: 
created: 
title: "map"
tags: []
signature: "pub fn map (counts : Vec < (String , usize , usize) >) -> ProblemMap"
---

The map from the language client's `(file key, errors, warnings)`.

# Examples

```rust
use crawrad_project::explorer_rows::problems::map;

let m = map(vec![("src/main.rs".to_string(), 1, 0)]);
assert_eq!(m.get("src/main.rs").unwrap().errors, 1);
```
