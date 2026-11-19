---
id: mask-editmask-reshape
kind: doc
target_file: src/mask.rs
line_hint: 243
author: 
created: 
title: "EditMask::reshape"
tags: []
signature: "pub fn reshape (& self , edited : & str) -> String"
---

The edited box text, shaped by the mask again (`format(raw(edited))`).

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
assert_eq!(mask.reshape("0102"), "01/02/____");
```
