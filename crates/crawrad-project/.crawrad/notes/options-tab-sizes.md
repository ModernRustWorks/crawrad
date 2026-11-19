---
id: options-tab-sizes
kind: doc
target_file: src/options.rs
line_hint: 35
author: 
created: 
title: "TAB_SIZES"
tags: []
signature: "pub const TAB_SIZES : & [u8] = & [2 , 4 , 8]"
---

The tab sizes offered by the editor options dropdown.

# Examples

```rust
use crawrad_project::options::TAB_SIZES;

assert!(TAB_SIZES.contains(&4));
```
