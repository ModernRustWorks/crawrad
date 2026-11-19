---
id: components-base-appearance-props
kind: doc
target_file: src/components/base.rs
line_hint: 94
author: 
created: 
title: "appearance_props"
tags: []
signature: "pub fn appearance_props () -> Vec < PropDesc >"
---

Both: a control that paints text and background.

# Examples

```rust
use crawrad_registry::appearance_props;

let props = appearance_props();
assert_eq!(props.len(), 4);
```
