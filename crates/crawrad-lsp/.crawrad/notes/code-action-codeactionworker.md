---
id: code-action-codeactionworker
kind: doc
target_file: src/code_action.rs
line_hint: 121
author: 
created: 
title: "CodeActionWorker"
tags: []
signature: "pub struct CodeActionWorker (crate :: asker :: Asker < CodeActionAnswer >)"
---

The thread that asks for code actions ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::code_action::CodeActionWorker;

let worker = CodeActionWorker::new();
assert!(worker.take(99).is_none());
```
