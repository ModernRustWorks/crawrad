---
id: explorer-rows-problems-problemmap
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 20
author: 
created: 
title: "ProblemMap"
tags: []
signature: "pub type ProblemMap = BTreeMap < String , Problems >"
---

Every file's problems, by [`key`].

# Examples

```rust
use crawrad_project::explorer_rows::problems::ProblemMap;

let map = ProblemMap::new();
assert!(map.is_empty());
```
