---
id: encoding-positionencoding-byte
kind: doc
target_file: src/encoding.rs
line_hint: 57
author: 
created: 
title: "PositionEncoding::byte"
tags: []
signature: "pub fn byte (self , line : & str , column : u32) -> usize"
---

Computes the byte offset in `line` corresponding to `column`.

The byte of `line` that column `column` points at (clamped to the
line's end; inside a character, its start).

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;

let offset = PositionEncoding::Utf8.byte("hello", 3);
assert_eq!(offset, 3);
```
