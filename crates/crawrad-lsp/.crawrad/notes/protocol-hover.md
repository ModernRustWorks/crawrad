---
id: protocol-hover
kind: doc
target_file: src/protocol.rs
line_hint: 387
author: 
created: 
title: "Hover"
tags: []
signature: "pub struct Hover"
---

Hover response containing the hover contents and optional range.

# Examples

```rust
use crawrad_lsp::protocol::{Documentation, Hover};

let hover = Hover {
    contents: Documentation::String("tip".to_string()),
    range: None,
};
assert_eq!(hover.contents.as_str(), "tip");
```
