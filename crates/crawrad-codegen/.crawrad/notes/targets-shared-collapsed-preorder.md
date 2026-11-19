---
id: targets-shared-collapsed-preorder
kind: doc
target_file: src/targets/shared.rs
line_hint: 547
author: 
created: 
title: "collapsed_preorder"
tags: []
signature: "pub fn collapsed_preorder (items : & [crawrad_ir :: Item]) -> Vec < usize >"
---

Returns pre-order indexes of unexpanded nodes with children.

Pre-order indexes of nodes with children whose `expanded` is false.
Uses the same numbering as `crawrad_runtime::tree::visible_rows`, capped at depth 32.

# Examples

```rust
use crawrad_codegen::targets::shared::collapsed_preorder;

let idxs = collapsed_preorder(&[]);
assert!(idxs.is_empty());
```
