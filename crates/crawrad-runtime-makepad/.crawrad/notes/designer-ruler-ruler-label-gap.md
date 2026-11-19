---
id: designer-ruler-ruler-label-gap
kind: doc
target_file: src/designer/ruler.rs
line_hint: 29
author: 
created: 
title: "RULER_LABEL_GAP"
tags: []
signature: "pub const RULER_LABEL_GAP : f32 = 40.0"
---

The minimum distance, in screen points, between two ruler numbers.

Mirrors the RXP's `FOLGA_DA_REGUA` (`crawrad-runtime-egui/src/lib.rs:3324`),
measured on the worst case the ruler produces: `1000` in a 9pt monospace,
which takes about 22 points, plus the breathing room up to the next tick.

# Examples

```rust,ignore
// Not reachable from outside the crate: shown, not run.
use crate::designer::ruler::RULER_LABEL_GAP;

assert_eq!(RULER_LABEL_GAP, 40.0);
```
