---
id: protocol-completionresponse
kind: doc
target_file: src/protocol.rs
line_hint: 353
author: 
created: 
title: "CompletionResponse"
tags: []
signature: "pub enum CompletionResponse"
---

Completion response representing either an item array or an item list.

Completion list response that may be either a list of items or an object containing an `items` array.

# Examples

```rust
use crawrad_lsp::protocol::CompletionResponse;

let resp = CompletionResponse::Array(Vec::new());
assert!(resp.into_items().is_empty());
```
