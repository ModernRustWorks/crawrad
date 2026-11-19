---
id: item-from-lsp
kind: doc
target_file: src/item.rs
line_hint: 294
author: 
created: 
title: "from_lsp"
tags: []
signature: "pub fn from_lsp (mut itens : Vec < crawrad_lsp :: CompletionItem >) -> Vec < ItemSugestao >"
---

Converts LSP completion items into sorted suggestion items.

rust-analyzer's items as the list shows them, in the server's order
(`sortText`): it puts what fits the place first.

# Examples

```rust
use crawrad_insight::item::from_lsp;

let items = from_lsp(Vec::new());
assert!(items.is_empty());
```
