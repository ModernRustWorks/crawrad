---
id: protocol-completionitem-is-deprecated
kind: doc
target_file: src/protocol.rs
line_hint: 347
author: 
created: 
title: "CompletionItem::is_deprecated"
tags: []
signature: "pub fn is_deprecated (& self) -> bool"
---

The item is `#[deprecated]`: tag 1, or the older `deprecated` field.

# Examples

```rust
use crawrad_lsp::protocol::CompletionItem;

let item = CompletionItem { deprecated: Some(true), ..Default::default() };
assert!(item.is_deprecated());
```
