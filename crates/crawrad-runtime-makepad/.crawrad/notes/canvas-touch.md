---
id: canvas-touch
kind: doc
target_file: src/canvas.rs
line_hint: 660
author: 
created: 
title: "Touch"
tags: []
signature: "pub enum Touch"
---

The touch that arrived, already with nothing from Makepad inside.

# Examples

```rust
use crawrad_runtime_makepad::canvas::Touch;

let t = Touch::Down { ctrl: false, double: false };
assert!(matches!(t, Touch::Down { ctrl: false, .. }));
```

It exists so that the TRANSLATION — the only thing `handle_event` does — can
be measured without a window: building a real `Hit::FingerDown` requires
`WindowId`/`DigitId`, and a test that cannot construct the input does not
measure the output.
