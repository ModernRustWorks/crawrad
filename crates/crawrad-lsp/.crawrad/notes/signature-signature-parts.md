---
id: signature-signature-parts
kind: doc
target_file: src/signature.rs
line_hint: 73
author: 
created: 
title: "Signature::parts"
tags: []
signature: "pub fn parts (& self) -> (& str , & str , & str)"
---

Splits the label into slices before, at, and after the active parameter.

The label cut around the active parameter: before, the parameter,
after. Without an active parameter all of it is "before".

# Examples

```rust
use crawrad_lsp::signature::Signature;

let sig = Signature {
    label: "fn foo(x: i32)".to_string(),
    params: vec![7..13],
    active: Some(0),
    count: 1,
};
let (before, active, after) = sig.parts();
assert_eq!(before, "fn foo(");
assert_eq!(active, "x: i32");
assert_eq!(after, ")");
```
