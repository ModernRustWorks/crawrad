---
id: complete-trigger
kind: doc
target_file: src/complete.rs
line_hint: 32
author: 
created: 
title: "Trigger"
tags: []
signature: "pub enum Trigger"
---

Why the list is asked for — LSP 3.17 `CompletionContext.triggerKind`.

# Examples

```rust
use crawrad_lsp::complete::Trigger;

assert_eq!(Trigger::default(), Trigger::Invoked);
```
