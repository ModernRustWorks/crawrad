---
id: protocol-completionitem
kind: doc
target_file: src/protocol.rs
line_hint: 260
author: 
created: 
title: "CompletionItem"
tags: []
signature: "pub struct CompletionItem"
---

A completion item representing a text snippet, symbol or keyword suggestion.

# Examples

```rust
use crawrad_lsp::protocol::CompletionItem;

let item = CompletionItem {
    label: "println".to_string(),
    ..Default::default()
};
assert_eq!(item.label, "println");
```
