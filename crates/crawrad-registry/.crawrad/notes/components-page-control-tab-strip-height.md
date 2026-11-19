---
id: components-page-control-tab-strip-height
kind: doc
target_file: src/components/page_control.rs
line_hint: 5
author: 
created: 
title: "TAB_STRIP_HEIGHT"
tags: []
signature: "pub const TAB_STRIP_HEIGHT : f32 = 24.0"
---

Height of the `PageControl` tab strip, in points.

Lives here, not in drawing, because **three** places must agree: the
designer (which hit-tests on the model), the RXP (which paints) and the generator
(which emits). A constant duplicated between them would be the same class of bug that
has appeared in this project whenever two places had to know the
same thing.

# Examples

```rust
use crawrad_registry::TAB_STRIP_HEIGHT;

assert_eq!(TAB_STRIP_HEIGHT, 24.0);
```
