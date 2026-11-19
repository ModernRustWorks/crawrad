---
id: mask-editmask-parse
kind: doc
target_file: src/mask.rs
line_hint: 134
author: 
created: 
title: "EditMask::parse"
tags: []
signature: "pub fn parse (mask : & str) -> Self"
---

Parses `mask`; any string is a mask, an unknown character a literal.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("!(99) 0000-0000;0;_");
assert!(!mask.keeps_literals());
assert_eq!(mask.blank(), '_');
```
