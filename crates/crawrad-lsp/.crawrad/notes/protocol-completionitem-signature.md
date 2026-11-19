---
id: protocol-completionitem-signature
kind: doc
target_file: src/protocol.rs
line_hint: 334
author: 
created: 
title: "CompletionItem::signature"
tags: []
signature: "pub fn signature (& self) -> Option < String >"
---

Returns the signature text to display beside the completion item's label.

The signature beside the label: `labelDetails.description`, else
`detail` — `labelDetails.detail` is the label's origin, apart (#384).

# Examples

```rust
use crawrad_lsp::protocol::CompletionItem;

let item = CompletionItem { detail: Some("fn()".to_string()), ..Default::default() };
assert_eq!(item.signature().as_deref(), Some("fn()"));
```
