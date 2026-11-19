---
id: designer-arrow-step
kind: doc
target_file: src/lib.rs
line_hint: 1029
author: 
created: 
title: "Designer::arrow_step"
tags: []
signature: "pub fn arrow_step (& self , mode : ArrowNudge) -> f32"
---

Returns how far a keyboard arrow moves, in design units.

**How far a keyboard arrow moves**, in design units.

Lives here and not in the shells because it is designer arithmetic: the grid belongs
to this crate, and ADR 0027 states that the shell recalculates none of this. Both
IDEs ask the same thing and receive the same number — which is what
keeps them in agreement when the owner changes `Grid size` in the options.

# Examples

```rust
use crawrad_designer::{ArrowNudge, Designer};

let d = Designer::default();
assert_eq!(d.arrow_step(ArrowNudge::Move), 8.0);
```
