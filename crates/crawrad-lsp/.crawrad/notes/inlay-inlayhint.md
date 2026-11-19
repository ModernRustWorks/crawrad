---
id: inlay-inlayhint
kind: doc
target_file: src/inlay.rs
line_hint: 29
author: 
created: 
title: "InlayHint"
tags: []
signature: "pub struct InlayHint"
---

One hint as LSP 3.17 sends it.

# Examples

```rust
use crawrad_lsp::inlay::{InlayHint, InlayHintLabel};
use crawrad_lsp::protocol::Position;

let hint = InlayHint {
    position: Position::new(0, 5),
    label: InlayHintLabel::Text(": usize".to_string()),
    kind: Some(1),
    padding_left: None,
    padding_right: None,
};
assert_eq!(hint.text(), ": usize");
```
