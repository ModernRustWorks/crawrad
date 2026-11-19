---
id: pull-pull-on-push
kind: doc
target_file: src/pull.rs
line_hint: 154
author: 
created: 
title: "Pull::on_push"
tags: []
signature: "pub fn on_push (& self , payload : & str) -> bool"
---

Handles server-pushed diagnostics, merging them with pulled entries.

A `publishDiagnostics` from the server, sent on merged with what was
pulled; `false` when `payload` is something else.

# Examples

```rust,no_run
// Pull cannot be constructed directly because its transport fields are crate-private.
use crawrad_lsp::pull::Pull;

fn demo(pull: &Pull) {
    assert!(!pull.on_push("{}"));
}
```
