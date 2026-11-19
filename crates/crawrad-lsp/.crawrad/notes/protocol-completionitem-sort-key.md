---
id: protocol-completionitem-sort-key
kind: doc
target_file: src/protocol.rs
line_hint: 342
author: 
created: 
title: "CompletionItem::sort_key"
tags: []
signature: "pub fn sort_key (& self) -> & str"
---

The server's order: `sortText`, else the label.

# Examples

```rust
use crawrad_lsp::protocol::CompletionItem;

let item = CompletionItem { label: "alpha".to_string(), ..Default::default() };
assert_eq!(item.sort_key(), "alpha");
```
