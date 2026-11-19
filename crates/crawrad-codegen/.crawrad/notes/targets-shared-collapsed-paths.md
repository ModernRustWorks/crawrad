---
id: targets-shared-collapsed-paths
kind: doc
target_file: src/targets/shared.rs
line_hint: 572
author: 
created: 
title: "collapsed_paths"
tags: []
signature: "pub fn collapsed_paths (items : & [crawrad_ir :: Item]) -> Vec < Vec < usize > >"
---

Returns hierarchical paths to unexpanded nodes with children.

Paths to nodes with children whose `expanded` is false (e.g. `[0, 2]` = third child of the first item).

# Examples

```rust
use crawrad_codegen::targets::shared::collapsed_paths;

let paths = collapsed_paths(&[]);
assert!(paths.is_empty());
```
