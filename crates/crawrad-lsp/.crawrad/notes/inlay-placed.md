---
id: inlay-placed
kind: doc
target_file: src/inlay.rs
line_hint: 78
author: 
created: 
title: "placed"
tags: []
signature: "pub fn placed (hints : & [InlayHint] , text : & str , enc : crate :: encoding :: PositionEncoding ,) -> Vec < (usize , usize , String) >"
---

Places inlay hints onto `text`, returning line, byte offset, and label.

`(zero-based line, byte in the line, text)` for every hint, placed on
`text` — the LSP counts columns in UTF-16 code units, the editor in bytes.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::inlay::placed;

let hints = placed(&[], "fn main() {}", PositionEncoding::Utf8);
assert!(hints.is_empty());
```
