---
id: inlay-inlayhintlabel
kind: doc
target_file: src/inlay.rs
line_hint: 44
author: 
created: 
title: "InlayHintLabel"
tags: []
signature: "pub enum InlayHintLabel"
---

The label: a string, or parts (rust-analyzer sends both shapes).

# Examples

```rust
use crawrad_lsp::inlay::InlayHintLabel;

let label = InlayHintLabel::Text("foo".to_string());
assert!(matches!(label, InlayHintLabel::Text(_)));
```
