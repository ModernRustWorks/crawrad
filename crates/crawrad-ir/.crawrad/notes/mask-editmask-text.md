---
id: mask-editmask-text
kind: doc
target_file: src/mask.rs
line_hint: 258
author: 
created: 
title: "EditMask::text"
tags: []
signature: "pub fn text (& self , shown : & str) -> String"
---

Returns what the `Text` property holds for `shown`.

What the `Text` property holds for `shown`: the shown text with its
literals (`;1`), or only the typed characters (`;0`).

# Examples

```rust
use crawrad_ir::mask::EditMask;

let with_lit = EditMask::parse("00/00/0000;1;_");
assert_eq!(with_lit.text("01/02/2020"), "01/02/2020");
let no_lit = EditMask::parse("00/00/0000;0;_");
assert_eq!(no_lit.text("01/02/2020"), "01022020");
```
