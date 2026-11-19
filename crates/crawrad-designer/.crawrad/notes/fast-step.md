---
id: fast-step
kind: doc
target_file: src/lib.rs
line_hint: 154
author: 
created: 
title: "FAST_STEP"
tags: []
signature: "pub const FAST_STEP : f32 = 3.0"
---

How many grid steps the fast arrow moves.

Three, and that is the number the owner stated — *"faster I think 3x"*. It remains as
a named constant because it is the answer to *"why 24 and not 8?"* when
someone measures the arrow with `Grid size` at 8.

# Examples

```rust
use crawrad_designer::FAST_STEP;

assert_eq!(FAST_STEP, 3.0);
```
