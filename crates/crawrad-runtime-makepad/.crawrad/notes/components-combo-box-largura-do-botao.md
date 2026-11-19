---
id: components-combo-box-largura-do-botao
kind: doc
target_file: src/components/combo_box.rs
line_hint: 14
author: 
created: 
title: "LARGURA_DO_BOTAO"
tags: []
signature: "pub const LARGURA_DO_BOTAO : f32 = 16.0"
---

The width of the arrow button, in design units.

The width of the arrow button, in design units — the rounded Windows
`GetSystemMetrics(SM_CXVSCROLL)`, which is the same
number as the scrollbar.

# Examples

```rust
use crawrad_runtime_makepad::components::combo_box::LARGURA_DO_BOTAO;

assert_eq!(LARGURA_DO_BOTAO, 16.0);
```
