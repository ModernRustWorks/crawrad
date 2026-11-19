---
id: protocol-completionitem-text-to-insert
kind: doc
target_file: src/protocol.rs
line_hint: 325
author: 
created: 
title: "CompletionItem::text_to_insert"
tags: []
signature: "pub fn text_to_insert (& self) -> & str"
---

The text that goes into the code: the edit's, else `insertText`,
else the label.

# Examples

```rust
use crawrad_lsp::protocol::CompletionItem;

let item = CompletionItem { label: "clone".to_string(), ..Default::default() };
assert_eq!(item.text_to_insert(), "clone");
```
