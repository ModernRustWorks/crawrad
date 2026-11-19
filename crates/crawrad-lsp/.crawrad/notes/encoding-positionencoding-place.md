---
id: encoding-positionencoding-place
kind: doc
target_file: src/encoding.rs
line_hint: 81
author: 
created: 
title: "PositionEncoding::place"
tags: []
signature: "pub fn place (self , lines : & [String] , at : Position) -> (usize , usize)"
---

Translates protocol position `at` into editor line and byte coordinates.

Where `at` lands in the editor's `lines`: `(line, byte)`. A line past
the end stays as given, at byte 0.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::protocol::Position;

let lines = vec!["hello world".to_string()];
let place = PositionEncoding::Utf8.place(&lines, Position::new(0, 5));
assert_eq!(place, (0, 5));
```
