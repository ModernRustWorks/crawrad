---
id: highlight-placed
kind: doc
target_file: src/highlight.rs
line_hint: 56
author: 
created: 
title: "placed"
tags: []
signature: "pub fn placed (highlights : & [DocumentHighlight] , text : & str , enc : crate :: encoding :: PositionEncoding ,) -> Vec < Occurrence >"
---

Places protocol highlights onto editor text lines as occurrences.

The highlights placed on `text`. A range spanning lines (a name never does)
is dropped rather than drawn wrong.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::highlight::placed;

let occs = placed(&[], "fn main() {}", PositionEncoding::Utf8);
assert!(occs.is_empty());
```
