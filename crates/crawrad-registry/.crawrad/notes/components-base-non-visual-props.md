---
id: components-base-non-visual-props
kind: doc
target_file: src/components/base.rs
line_hint: 16
author: 
created: 
title: "non_visual_props"
tags: []
signature: "pub fn non_visual_props () -> Vec < PropDesc >"
---

**Does not appear on screen** — the non-visual tier.

# Examples

```rust
use crawrad_registry::non_visual_props;

let props = non_visual_props();
assert!(props.iter().any(|p| p.key == "rect"));
```
