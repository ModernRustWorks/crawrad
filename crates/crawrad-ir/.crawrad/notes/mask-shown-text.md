---
id: mask-shown-text
kind: doc
target_file: src/mask.rs
line_hint: 322
author: 
created: 
title: "shown_text"
tags: []
signature: "pub fn shown_text (mask : & str , text : & str) -> String"
---

Returns the text a `MaskEdit` shows for `text` under `mask`.

The text a `MaskEdit` shows for `text` under `mask`: `text` reshaped, or
the blank mask when it is empty. An empty `mask` hands `text` back.

# Examples

```rust
use crawrad_ir::mask::shown_text;

assert_eq!(shown_text("00/00/0000;1;_", "01022020"), "01/02/2020");
assert_eq!(shown_text("", "hello"), "hello");
```
