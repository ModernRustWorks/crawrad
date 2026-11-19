---
id: designer-ruler-ticks
kind: doc
target_file: src/designer/ruler.rs
line_hint: 64
author: 
created: 
title: "ticks"
tags: []
signature: "pub fn ticks (t : & Transformacao , vertical : bool , from : f64 , to : f64) -> Vec < Tick >"
---

The ticks between `from` and `to` (screen points along the axis).

Counting starts at **zero on the form origin**, not at the window edge: in
a classic RAD IDE the coordinate the Object Inspector shows is relative to the form,
and the ruler has to say the same number. The origin comes in through the
[`Transformacao`], which is the canvas's only one (ADR 0019).

It only ticks **non-negative** coordinates, like the RXP: whatever sits
before the form has no number worth anything.

# Examples

```rust
use crawrad_runtime_makepad::Transformacao;
use crawrad_runtime_makepad::designer::ticks;

let t = Transformacao::new(0.0, 0.0, 1.0);
let r_ticks = ticks(&t, false, 0.0, 100.0);
assert!(!r_ticks.is_empty());
```
