---
id: protocol-labeldetails
kind: doc
target_file: src/protocol.rs
line_hint: 315
author: 
created: 
title: "LabelDetails"
tags: []
signature: "pub struct LabelDetails"
---

What LSP 3.17 shows next to a label.

# Examples

```rust
use crawrad_lsp::protocol::LabelDetails;

let details = LabelDetails {
    detail: Some("(as Clone)".to_string()),
    description: None,
};
assert!(details.detail.is_some());
```
