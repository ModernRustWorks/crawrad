---
id: signature-trigger-context
kind: doc
target_file: src/signature.rs
line_hint: 46
author: 
created: 
title: "Trigger::context"
tags: []
signature: "pub fn context (self , retrigger : bool) -> Value"
---

Builds the LSP `SignatureHelpContext` object for this trigger.

The request's `context` (LSP `SignatureHelpContext`). `retrigger` is
whether a card was already up or being asked for.

# Examples

```rust
use crawrad_lsp::signature::Trigger;

let ctx = Trigger::Invoked.context(false);
assert_eq!(ctx["triggerKind"], 1);
assert_eq!(ctx["isRetrigger"], false);
```
