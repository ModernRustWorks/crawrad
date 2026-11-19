---
id: components-base-color-props
kind: doc
target_file: src/components/base.rs
line_hint: 81
author: 
created: 
title: "color_props"
tags: []
signature: "pub fn color_props () -> Vec < PropDesc >"
---

Properties defining how a control paints its background.

**How a control paints its background** (#433): `Color` (alpha 0 = the
theme's) and whether it comes from the parent.

# Examples

```rust
use crawrad_registry::color_props;

let props = color_props();
assert_eq!(props.len(), 2);
```
