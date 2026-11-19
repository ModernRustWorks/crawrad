---
id: workspace-refusal-old-toolchain
kind: doc
target_file: src/workspace.rs
line_hint: 1671
author: 
created: 
title: "refusal_old_toolchain"
tags: []
signature: "pub fn refusal_old_toolchain (found : (u32 , u32) , needed : (u32 , u32)) -> Option < Refusal >"
---

Refusal when the toolchain is older than the project needs.

The refusal when the toolchain is older than the project needs (`needed`, from
`crawrad_codegen::minimum_rustc`). `None` when `found` is new enough.

# Examples

```rust
use crawrad_project::workspace::refusal_old_toolchain;

assert!(refusal_old_toolchain((1, 80), (1, 85)).is_some());
assert!(refusal_old_toolchain((1, 90), (1, 85)).is_none());
```
