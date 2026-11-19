---
id: components-comum-makepad-border
kind: doc
target_file: src/components/comum.rs
line_hint: 76
author: 
created: 
title: "MAKEPAD_BORDER"
tags: []
signature: "pub const MAKEPAD_BORDER : [f32 ; 4] = crate :: shared_theme :: ROLE_BORDER"
---

The border of a control. Mirrors RXP's `CTL_BORDA`.

**It is the single token that solves half of what the owner rejected.** It was
`#45475a` — **1.64:1** against the paper, which means invisible —, and without it nothing
separated control from background. The floor is the same WCAG threshold that RXP adopts for
a border that identifies a control: 3:1 against the paper, against the face, and
against the field.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_BORDER;

assert_eq!(MAKEPAD_BORDER[3], 1.0);
```
