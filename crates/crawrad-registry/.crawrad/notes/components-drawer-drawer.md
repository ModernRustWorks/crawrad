---
id: components-drawer-drawer
kind: doc
target_file: src/components/drawer.rs
line_hint: 5
author: 
created: 
title: "drawer"
tags: []
signature: "pub fn drawer () -> ComponentDesc"
---

A side panel drawer that slides open over the form.

`Drawer` (#500, wave 2): a side panel that opens over the form with a header,
optional close button, child controls, and a dimmed overlay (scrim).

# Examples

```rust
use crawrad_registry::drawer;

let desc = drawer();
assert_eq!(desc.class, "Drawer");
```
