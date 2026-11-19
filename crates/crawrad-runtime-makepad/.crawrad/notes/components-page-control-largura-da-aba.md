---
id: components-page-control-largura-da-aba
kind: doc
target_file: src/components/page_control.rs
line_hint: 21
author: 
created: 
title: "LARGURA_DA_ABA"
tags: []
signature: "pub const LARGURA_DA_ABA : f32 = 92.0"
---

Fixed width of each tab in design units.

Fixed width of each tab. The designer resolves the click by
`(x - esquerda) / LARGURA_DA_ABA`, so drawing does the reverse calculation.

# Examples

```rust
use crawrad_runtime_makepad::components::page_control::LARGURA_DA_ABA;

assert_eq!(LARGURA_DA_ABA, 92.0);
```
