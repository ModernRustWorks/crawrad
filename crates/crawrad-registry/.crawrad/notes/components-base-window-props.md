---
id: components-base-window-props
kind: doc
target_file: src/components/base.rs
line_hint: 54
author: 
created: 
title: "window_props"
tags: []
signature: "pub fn window_props () -> Vec < PropDesc >"
---

Receives focus, and therefore enters the tab order sequence.

# Examples

```rust
use crawrad_registry::window_props;

let props = window_props();
assert!(props.iter().any(|p| p.key == "tab_stop"));
```
