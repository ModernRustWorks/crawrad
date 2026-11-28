---
id: hostmodel-previous-rect
kind: doc
target_file: src/lib.rs
line_hint: 341
author: 
created: 
title: "HostModel::previous_rect"
tags: []
signature: "pub fn previous_rect (& self) -> Option < crawrad_ir :: Rect >"
---

The front form's previous rectangle, for the anchors and alignment pass.

It is per form, since each is resized on its own.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.previous_rect().is_none(), "no form, no previous rectangle");
```
