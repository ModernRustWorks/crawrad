---
id: canvas-translate
kind: doc
target_file: src/canvas.rs
line_hint: 675
author: 
created: 
title: "translate"
tags: []
signature: "pub fn translate (toque : Touch , t : & Transformacao , abs : DVec2) -> Vec < RuntimeEvent >"
---

The `Hit` -> [`RuntimeEvent`] translation, **in design units**.

It is the contract of ADR 0027: the adapter translates, `crawrad-designer` decides. Note
that the coordinate passes through [`Transformacao`] in here — the caller
passes the SCREEN point, and that is what prevents a second coordinate calculation
from appearing in `handle_event`.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime::RuntimeEvent;
use crawrad_runtime_makepad::canvas::{translate, Touch};
use crawrad_runtime_makepad::Transformacao;

let t = Transformacao::new(0.0, 0.0, 1.0);
let evs = translate(Touch::Up, &t, dvec2(10.0, 20.0));
assert_eq!(evs, vec![RuntimeEvent::PointerUp { x: 10.0, y: 20.0 }]);
```
