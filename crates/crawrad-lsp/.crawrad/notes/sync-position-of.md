---
id: sync-position-of
kind: doc
target_file: src/sync.rs
line_hint: 98
author: 
created: 
title: "position_of"
tags: []
signature: "pub fn position_of (text : & str , offset : usize , enc : crate :: encoding :: PositionEncoding) -> Position"
---

The LSP position (UTF-16 column) of byte `offset` in `text`.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::sync::position_of;

let pos = position_of("hello\nworld", 6, PositionEncoding::Utf8);
assert_eq!(pos.line, 1);
assert_eq!(pos.character, 0);
```
