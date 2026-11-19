---
id: canvas-brush-probe-char-width
kind: doc
target_file: src/canvas/brush.rs
line_hint: 261
author: 
created: 
title: "PROBE_CHAR_WIDTH"
tags: []
signature: "pub const PROBE_CHAR_WIDTH : f64 = 7.0"
---

The width of one character when the probe measures text.

The width of one character when the probe measures text, in screen points
at zoom 1. A fixed metric on purpose: what the probe answers is *what* was
painted and *where*, and a font would make that depend on the machine.

# Examples

```rust
use crawrad_runtime_makepad::canvas::PROBE_CHAR_WIDTH;

assert_eq!(PROBE_CHAR_WIDTH, 7.0);
```
