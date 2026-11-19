---
id: sync-apply-change
kind: doc
target_file: src/sync.rs
line_hint: 123
author: 
created: 
title: "apply_change"
tags: []
signature: "pub fn apply_change (text : & str , change : & ContentChange , enc : crate :: encoding :: PositionEncoding ,) -> String"
---

`text` with `change` applied, as the server does.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::protocol::{Position, Range};
use crawrad_lsp::sync::{apply_change, ContentChange};

let change = ContentChange {
    range: Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 0, character: 3 },
    },
    text: "bye".to_string(),
};
let res = apply_change("foo world", &change, PositionEncoding::Utf8);
assert_eq!(res, "bye world");
```
