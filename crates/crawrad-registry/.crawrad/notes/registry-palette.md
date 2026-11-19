---
id: registry-palette
kind: doc
target_file: src/lib.rs
line_hint: 695
author: 
created: 
title: "Registry::palette"
tags: []
signature: "pub fn palette (& self , target : & str) -> Vec < (& 'static str , Vec < & ComponentDesc >) >"
---

Palette items grouped by category, in display order.

# Examples

```rust
use crawrad_registry::Registry;

let reg = Registry::builtin();
let groups = reg.palette("win32");
assert!(!groups.is_empty());
```
