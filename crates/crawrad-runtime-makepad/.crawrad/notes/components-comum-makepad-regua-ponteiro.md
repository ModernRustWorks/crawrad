---
id: components-comum-makepad-regua-ponteiro
kind: doc
target_file: src/components/comum.rs
line_hint: 112
author: 
created: 
title: "MAKEPAD_REGUA_PONTEIRO"
tags: []
signature: "pub const MAKEPAD_REGUA_PONTEIRO : [f32 ; 4] = MAKEPAD_ACCENT"
---

Where the pointer is, marked on both rulers.

**It does not copy RXP's red, and the reason is about role.** There, `REGUA_PONTEIRO`
is the SAME value as `SELO` (`#C02020`), and the comment explains why: *"it is the color
this canvas already uses for 'look here'"*. The token playing that role in this theme
is [`MAKEPAD_ACCENT`] — from it come the tab order badge,
the marquee, and the handle. Copying the red would introduce a fifth accent color to
a canvas that already has its own.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_REGUA_PONTEIRO;

assert_eq!(MAKEPAD_REGUA_PONTEIRO[3], 1.0);
```
