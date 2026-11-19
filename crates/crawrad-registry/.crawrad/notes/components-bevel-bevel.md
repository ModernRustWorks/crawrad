---
id: components-bevel-bevel
kind: doc
target_file: src/components/bevel.rs
line_hint: 5
author: 
created: 
title: "bevel"
tags: []
signature: "pub fn bevel () -> ComponentDesc"
---

A sunk or raised box, frame or line that groups controls visually.

`Bevel` (#500, wave 1): a sunk or raised box, frame or line that groups
controls visually. A graphic control: no focus, no events.

# Examples

```rust
use crawrad_registry::bevel;

let desc = bevel();
assert_eq!(desc.class, "Bevel");
```
