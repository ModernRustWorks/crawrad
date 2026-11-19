---
id: mask-editmask-raw
kind: doc
target_file: src/mask.rs
line_hint: 232
author: 
created: 
title: "EditMask::raw"
tags: []
signature: "pub fn raw (& self , shown : & str) -> String"
---

Recovers the typed characters from a shown or edited text.

The characters the user typed, recovered from a shown (or edited)
text: blanks and the mask's literals are left out.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
assert_eq!(mask.raw("01/02/____"), "0102");
```
