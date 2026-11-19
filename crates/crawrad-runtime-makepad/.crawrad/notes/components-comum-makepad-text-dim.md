---
id: components-comum-makepad-text-dim
kind: doc
target_file: src/components/comum.rs
line_hint: 91
author: 
created: 
title: "MAKEPAD_TEXT_DIM"
tags: []
signature: "pub const MAKEPAD_TEXT_DIM : [f32 ; 4] = crate :: shared_theme :: ROLE_TEXT_DIM"
---

The text of a disabled control and the secondary label.

The text of a disabled control, and the secondary label. Mirrors RXP's
`CTL_TEXTO_FRACO`.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_TEXT_DIM;

assert_eq!(MAKEPAD_TEXT_DIM[3], 1.0);
```
