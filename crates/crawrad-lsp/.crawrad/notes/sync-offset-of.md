---
id: sync-offset-of
kind: doc
target_file: src/sync.rs
line_hint: 108
author: 
created: 
title: "offset_of"
tags: []
signature: "pub fn offset_of (text : & str , position : Position , enc : crate :: encoding :: PositionEncoding) -> usize"
---

Converts an LSP position into a byte offset within `text`.

The byte of `position` (UTF-16 column) in `text` — how the server
applies a change; the tests use it to prove a change round-trips.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::protocol::Position;
use crawrad_lsp::sync::offset_of;

let pos = Position { line: 1, character: 0 };
let offset = offset_of("hello\nworld", pos, PositionEncoding::Utf8);
assert_eq!(offset, 6);
```
