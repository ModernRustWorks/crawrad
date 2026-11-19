---
id: mask-editmask-blank
kind: doc
target_file: src/mask.rs
line_hint: 185
author: 
created: 
title: "EditMask::blank"
tags: []
signature: "pub fn blank (& self) -> char"
---

The character shown in a position nothing was typed into.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00;1;_");
assert_eq!(mask.blank(), '_');
```
