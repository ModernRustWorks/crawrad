---
id: mask-editmask-hold-edit
kind: doc
target_file: src/mask.rs
line_hint: 267
author: 
created: 
title: "EditMask::hold_edit"
tags: []
signature: "pub fn hold_edit (& self , before : & str , edited : & str , cursor : usize) -> (String , usize)"
---

An edit held to the mask with fixed positions on deletion (#71, #21).

When `edited` has fewer characters than `before` and `before` is a full
shaped text (its character count equals the mask's slot count), the edit
was a deletion: placeholders from `cursor .. cursor + missing` are
re-inserted at `cursor` (blank for input positions, the literal itself for
literals), keeping other positions fixed in place and leaving the cursor
at `cursor`.

Otherwise (typing, paste, or a box not yet full), falls back to [`EditMask::hold`].

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
let (text, cursor) = mask.hold_edit("01/02/2020", "01/02/202", 9);
assert_eq!(text, "01/02/202_");
assert_eq!(cursor, 9);
```
