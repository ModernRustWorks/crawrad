---
id: sync-contentchange
kind: doc
target_file: src/sync.rs
line_hint: 42
author: 
created: 
title: "ContentChange"
tags: []
signature: "pub struct ContentChange"
---

One `TextDocumentContentChangeEvent` with a range.

# Examples

```rust
use crawrad_lsp::protocol::{Position, Range};
use crawrad_lsp::sync::ContentChange;

let change = ContentChange {
    range: Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 0, character: 3 },
    },
    text: "let".to_string(),
};
assert_eq!(change.text, "let");
```
