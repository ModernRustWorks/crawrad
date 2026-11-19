---
id: explorer-rows-rows-child
kind: doc
target_file: src/explorer_rows/rows.rs
line_hint: 41
author: 
created: 
title: "CHILD"
tags: []
signature: "pub const CHILD : usize = 1"
---

How deep the children of a node sit, in nesting steps.

One number for the whole folder: the forms of a member, the files of the
plan and the pairs of the legend are all *inside something*, and three
separate constants would be three chances for one of them to drift a pixel.

# Examples

```rust
use crawrad_project::explorer_rows::rows::CHILD;

assert_eq!(CHILD, 1);
```
