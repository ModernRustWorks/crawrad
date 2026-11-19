---
id: mask-editmask-hold
kind: doc
target_file: src/mask.rs
line_hint: 298
author: 
created: 
title: "EditMask::hold"
tags: []
signature: "pub fn hold (& self , edited : & str , cursor : usize) -> (String , usize)"
---

Reshapes an edit and places the cursor on the next editable position.

An edit held to the mask: `edited` reshaped and the cursor (a
character index) on the next editable position; an empty mask hands
both back untouched.

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
let (text, cursor) = mask.hold("01", 2);
assert_eq!(text, "01/__/____");
assert_eq!(cursor, 3);
```
