---
id: canvas-brush-probe
kind: doc
target_file: src/canvas/brush.rs
line_hint: 267
author: 
created: 
title: "Probe"
tags: []
signature: "pub struct Probe"
---

A surface recording brush paint commands without a GPU (#114).

**The shape probe** (#114): a surface that writes down what the brush
paints instead of sending it to the GPU.

A blank canvas used to pass every test, because nothing measured the
painting: geometry, gestures and lines were all tested, the ink was not.
The probe is the other half. It counts what was painted, where and in
which colour, the way the `crawrad-codegen` drawing gate counts the shapes of
the generated app, and it runs anywhere, with no window and no GPU. It does
not judge looks: an SDF that compiles and paints ugly is still the owner's
eye to catch.

# Examples

```rust
use crawrad_runtime_makepad::canvas::Probe;

let probe = Probe::default();
assert!(probe.marks().is_empty());
```
