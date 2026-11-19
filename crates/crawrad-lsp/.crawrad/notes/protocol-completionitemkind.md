---
id: protocol-completionitemkind
kind: doc
target_file: src/protocol.rs
line_hint: 144
author: 
created: 
title: "CompletionItemKind"
tags: []
signature: "pub enum CompletionItemKind"
---

The kind of a completion item.

# Examples

```rust
use crawrad_lsp::protocol::CompletionItemKind;

assert_eq!(CompletionItemKind::from(2), CompletionItemKind::Method);
```
