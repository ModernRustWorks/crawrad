---
id: components-comum-makepad-panel
kind: doc
target_file: src/components/comum.rs
line_hint: 79
author: 
created: 
title: "MAKEPAD_PANEL"
tags: []
signature: "pub const MAKEPAD_PANEL : [f32 ; 4] = rgb (0x2F , 0x2F , 0x45)"
---

The surface of a container on the form.

The surface of a CONTAINER: `Panel`, `ScrollBox`, `TabSheet`, the body of
`PageControl`. Mirrors RXP's `PAINEL`.

It exists for the reason RXP wrote in `lib.rs:172-177`: *"the container
must be VISIBLE against the paper"*. And it cannot be the same face as
`Button`, otherwise what the owner saw returns — panel and button indistinguishable.
In RXP the panel is one step DARKER than the light paper; here it is one step
LIGHTER than the dark paper, which is the same step read in the opposite direction.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_PANEL;

assert_eq!(MAKEPAD_PANEL[3], 1.0);
```
