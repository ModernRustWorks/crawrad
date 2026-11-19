---
id: mask-editmask-cursor-after
kind: doc
target_file: src/mask.rs
line_hint: 306
author: 
created: 
title: "EditMask::cursor_after"
tags: []
signature: "pub fn cursor_after (& self , edited : & str , cursor : usize) -> usize"
---

Calculates where the cursor goes after an edit.

Where the cursor goes after an edit: `cursor` is its character index
in the `edited` text, and the answer is a character index in
`reshape(edited)` — the next editable position after the characters
typed before the cursor (the end, when the mask is full).

# Examples

```rust
use crawrad_ir::mask::EditMask;

let mask = EditMask::parse("00/00/0000;1;_");
assert_eq!(mask.cursor_after("01", 2), 3);
```
