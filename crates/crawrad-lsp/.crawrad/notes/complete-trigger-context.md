---
id: complete-trigger-context
kind: doc
target_file: src/complete.rs
line_hint: 45
author: 
created: 
title: "Trigger::context"
tags: []
signature: "pub fn context (self) -> serde_json :: Value"
---

The `context` of the `textDocument/completion` request.

# Examples

```rust
use crawrad_lsp::complete::Trigger;

let ctx = Trigger::Invoked.context();
assert_eq!(ctx["triggerKind"], 1);
```
