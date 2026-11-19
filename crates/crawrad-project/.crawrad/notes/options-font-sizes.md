---
id: options-font-sizes
kind: doc
target_file: src/options.rs
line_hint: 32
author: 
created: 
title: "FONT_SIZES"
tags: []
signature: "pub const FONT_SIZES : & [u32] = & [9 , 10 , 11 , 12 , 14 , 16 , 18 , 20]"
---

The font sizes offered by the editor options dropdown.

# Examples

```rust
use crawrad_project::options::FONT_SIZES;

assert!(FONT_SIZES.contains(&12));
```
