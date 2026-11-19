---
id: signature-signature
kind: doc
target_file: src/signature.rs
line_hint: 61
author: 
created: 
title: "Signature"
tags: []
signature: "pub struct Signature"
---

Signature help information to display for a callable symbol.

The signature to show: its label, where each parameter sits in it (byte
ranges), and which parameter the caret is on.

# Examples

```rust
use crawrad_lsp::signature::Signature;

let sig = Signature {
    label: "fn foo(x: i32)".to_string(),
    params: vec![7..13],
    active: Some(0),
    count: 1,
};
assert_eq!(sig.active, Some(0));
```
