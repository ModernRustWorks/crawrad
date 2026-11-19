---
id: viewcenter-design-tab-label
kind: doc
target_file: src/lib.rs
line_hint: 80
author: 
created: 
title: "ViewCenter::design_tab_label"
tags: []
signature: "pub fn design_tab_label (self) -> & 'static str"
---

The label of the left tab of the centre strip.

# Examples

```rust
use crawrad_host::ViewCenter;

assert_eq!(ViewCenter::Design.design_tab_label(), "Design");
```
