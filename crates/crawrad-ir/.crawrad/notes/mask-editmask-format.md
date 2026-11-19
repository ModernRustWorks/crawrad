---
id: mask-editmask-format
kind: doc
target_file: src/mask.rs
line_hint: 217
author: 
created: 
title: "EditMask::format"
tags: []
signature: "pub fn format (& self , typed : & str) -> String"
---

Returns the shown text for `typed` laid into the mask.

The shown text for `typed`: the accepted characters laid into the
mask, case rules applied, blanks where nothing was typed.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
assert_eq!(mask.format("01022020"), "01/02/2020");
```
