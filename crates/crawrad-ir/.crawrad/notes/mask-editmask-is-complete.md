---
id: mask-editmask-is-complete
kind: doc
target_file: src/mask.rs
line_hint: 248
author: 
created: 
title: "EditMask::is_complete"
tags: []
signature: "pub fn is_complete (& self , shown : & str) -> bool"
---

Every required position of `shown` holds a typed character.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
assert!(mask.is_complete("01/02/2020"));
assert!(!mask.is_complete("01/02/____"));
```
