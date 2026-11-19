---
id: signature-trigger
kind: doc
target_file: src/signature.rs
line_hint: 34
author: 
created: 
title: "Trigger"
tags: []
signature: "pub enum Trigger"
---

Why the editor asks (LSP `SignatureHelpTriggerKind`).

# Examples

```rust
use crawrad_lsp::signature::Trigger;

let trigger = Trigger::Invoked;
assert_eq!(trigger, Trigger::Invoked);
```
