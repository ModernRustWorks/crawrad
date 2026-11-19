---
id: inlay-inlayhint-text
kind: doc
target_file: src/inlay.rs
line_hint: 58
author: 
created: 
title: "InlayHint::text"
tags: []
signature: "pub fn text (& self) -> String"
---

The text drawn, with the spaces the server asked for around it.

# Examples

```rust
use crawrad_lsp::inlay::{InlayHint, InlayHintLabel};
use crawrad_lsp::protocol::Position;

let hint = InlayHint {
    position: Position::new(0, 0),
    label: InlayHintLabel::Text("msg".to_string()),
    kind: None,
    padding_left: Some(true),
    padding_right: None,
};
assert_eq!(hint.text(), " msg");
```
