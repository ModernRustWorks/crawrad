---
id: inlay-utf16-to-byte
kind: doc
target_file: src/inlay.rs
line_hint: 95
author: 
created: 
title: "utf16_to_byte"
tags: []
signature: "pub fn utf16_to_byte (line : & str , column : u32) -> usize"
---

Translates a UTF-16 column to its byte index in `line`.

The byte of `line` that UTF-16 column `column` points at (clamped to the
line's end).

# Examples

```rust
use crawrad_lsp::inlay::utf16_to_byte;

assert_eq!(utf16_to_byte("hello", 3), 3);
```
