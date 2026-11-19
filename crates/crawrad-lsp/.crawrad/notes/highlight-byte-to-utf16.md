---
id: highlight-byte-to-utf16
kind: doc
target_file: src/highlight.rs
line_hint: 81
author: 
created: 
title: "byte_to_utf16"
tags: []
signature: "pub fn byte_to_utf16 (line : & str , byte : usize) -> u32"
---

Computes the UTF-16 column of byte `byte` in `line`.

The UTF-16 column of byte `byte` of `line` — the other way round from
[`utf16_to_byte`](crate::inlay::utf16_to_byte).

# Examples

```rust
use crawrad_lsp::highlight::byte_to_utf16;

assert_eq!(byte_to_utf16("hello", 3), 3);
```
