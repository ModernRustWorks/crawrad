---
id: canvas-brush-probe-marks
kind: doc
target_file: src/canvas/brush.rs
line_hint: 275
author: 
created: 
title: "Probe::marks"
tags: []
signature: "pub fn marks (& self) -> & [Mark]"
---

What has been painted so far, in painting order.

# Examples

```rust
use crawrad_runtime_makepad::canvas::Probe;

let probe = Probe::default();
assert_eq!(probe.marks().len(), 0);
```
