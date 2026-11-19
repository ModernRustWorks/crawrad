---
id: mask-editmask
kind: doc
target_file: src/mask.rs
line_hint: 99
author: 
created: 
title: "EditMask"
tags: []
signature: "pub struct EditMask"
---

A parsed `EditMask`.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
assert_eq!(mask.blank(), '_');
assert!(mask.keeps_literals());
```
