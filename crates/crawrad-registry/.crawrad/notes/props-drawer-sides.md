---
id: props-drawer-sides
kind: doc
target_file: src/props.rs
line_hint: 85
author: 
created: 
title: "DRAWER_SIDES"
tags: []
signature: "pub const DRAWER_SIDES : & [& str] = & [\"dsLeft\" , \"dsRight\"]"
---

The edge from which a `Drawer` slides out (`Side`, #500).

# Examples

```rust
use crawrad_registry::DRAWER_SIDES;

assert!(DRAWER_SIDES.contains(&"dsLeft"));
```
