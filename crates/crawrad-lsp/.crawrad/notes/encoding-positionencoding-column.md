---
id: encoding-positionencoding-column
kind: doc
target_file: src/encoding.rs
line_hint: 42
author: 
created: 
title: "PositionEncoding::column"
tags: []
signature: "pub fn column (self , line : & str , byte : usize) -> u32"
---

The column of byte `byte` of `line` (clamped to the line).

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;

let col = PositionEncoding::Utf8.column("hello", 3);
assert_eq!(col, 3);
```
