---
id: format-formatted
kind: doc
target_file: src/format.rs
line_hint: 86
author: 
created: 
title: "formatted"
tags: []
signature: "pub fn formatted (text : & str , edits : & [TextEdit] , enc : crate :: encoding :: PositionEncoding ,) -> Option < String >"
---

Applies formatting edits to `text`, returning `None` if unchanged.

`text` with `edits` applied, or `None` when nothing changes (already
formatted). The edits' columns are in `enc`.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::format::formatted;

let res = formatted("fn main() {}", &[], PositionEncoding::Utf8);
assert_eq!(res, None);
```
