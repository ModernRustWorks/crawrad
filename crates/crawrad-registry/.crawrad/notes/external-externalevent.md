---
id: external-externalevent
kind: doc
target_file: src/external.rs
line_hint: 265
author: 
created: 
title: "ExternalEvent"
tags: []
signature: "pub struct ExternalEvent"
---

One event a third party declares.

# Examples

```rust
use crawrad_registry::external::ExternalEvent;

let ev = ExternalEvent {
    name: "on_click".into(),
    display: "OnClick".into(),
    signature: "fn(&mut self)".into(),
};
assert_eq!(ev.name, "on_click");
```
