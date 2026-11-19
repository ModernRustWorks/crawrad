---
id: components-base-control-props
kind: doc
target_file: src/components/base.rs
line_hint: 23
author: 
created: 
title: "control_props"
tags: []
signature: "pub fn control_props () -> Vec < PropDesc >"
---

Appears on screen — has position, size and visibility.

# Examples

```rust
use crawrad_registry::control_props;

let props = control_props();
assert!(props.iter().any(|p| p.key == "visible"));
```
