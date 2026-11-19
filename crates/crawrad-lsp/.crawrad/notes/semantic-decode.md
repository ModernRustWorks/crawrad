---
id: semantic-decode
kind: doc
target_file: src/semantic.rs
line_hint: 111
author: 
created: 
title: "decode"
tags: []
signature: "pub fn decode (data : & [u32] , legend : & Legend , text : & str , enc : crate :: encoding :: PositionEncoding ,) -> Vec < Span >"
---

Decodes raw semantic token integers into editor spans.

The relative `data` (5 integers a token) as spans of `text`, only the
classes the editor colours and the deprecated names. Columns are in `enc`
and become bytes.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::semantic::{decode, Legend};

let legend = Legend {
    types: vec!["macro".to_string()],
    modifiers: Vec::new(),
};
let data = [0, 0, 4, 0, 0];
let spans = decode(&data, &legend, "test\n", PositionEncoding::Utf8);
assert_eq!(spans.len(), 1);
assert_eq!(spans[0].end, 4);
```
