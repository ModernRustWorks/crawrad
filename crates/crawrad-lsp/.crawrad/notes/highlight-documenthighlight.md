---
id: highlight-documenthighlight
kind: doc
target_file: src/highlight.rs
line_hint: 32
author: 
created: 
title: "DocumentHighlight"
tags: []
signature: "pub struct DocumentHighlight"
---

One highlight as LSP 3.17 sends it. `kind`: 1 text, 2 read, 3 write.

# Examples

```rust
use crawrad_lsp::highlight::DocumentHighlight;
use crawrad_lsp::protocol::{Position, Range};

let hl = DocumentHighlight {
    range: Range::new(Position::new(0, 0), Position::new(0, 5)),
    kind: Some(2),
};
assert_eq!(hl.kind, Some(2));
```
