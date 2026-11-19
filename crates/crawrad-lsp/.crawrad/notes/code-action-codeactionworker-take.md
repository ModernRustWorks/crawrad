---
id: code-action-codeactionworker-take
kind: doc
target_file: src/code_action.rs
line_hint: 181
author: 
created: 
title: "CodeActionWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < CodeActionAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::code_action::CodeActionWorker;

let worker = CodeActionWorker::new();
assert!(worker.take(1).is_none());
```
