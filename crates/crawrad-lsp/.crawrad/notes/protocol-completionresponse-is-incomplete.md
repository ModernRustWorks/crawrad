---
id: protocol-completionresponse-is-incomplete
kind: doc
target_file: src/protocol.rs
line_hint: 366
author: 
created: 
title: "CompletionResponse::is_incomplete"
tags: []
signature: "pub fn is_incomplete (& self) -> bool"
---

LSP: the list is partial and the client asks again as the user types.

# Examples

```rust
use crawrad_lsp::protocol::CompletionResponse;

let resp = CompletionResponse::Array(Vec::new());
assert!(!resp.is_incomplete());
```
