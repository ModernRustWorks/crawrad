---
id: inlay-end-of
kind: doc
target_file: src/inlay.rs
line_hint: 117
author: 
created: 
title: "end_of"
tags: []
signature: "pub fn end_of (text : & str , enc : crate :: encoding :: PositionEncoding) -> Position"
---

Calculates the ending position of `text` in protocol coordinates.

Where `text` ends, as the LSP counts it: the last line and its length in
UTF-16 units.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::inlay::end_of;

let end = end_of("foo\nbar", PositionEncoding::Utf8);
assert_eq!(end.line, 1);
assert_eq!(end.character, 3);
```
