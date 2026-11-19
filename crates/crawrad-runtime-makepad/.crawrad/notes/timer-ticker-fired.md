---
id: timer-ticker-fired
kind: doc
target_file: src/timer.rs
line_hint: 38
author: 
created: 
title: "Ticker::fired"
tags: []
signature: "pub fn fired (& self , event : & Event) -> bool"
---

Whether `event` is this timer's tick.

# Examples

```rust,no_run
// Makepad Event requires the Makepad application runtime.
use crawrad_runtime_makepad::timer::Ticker;

let ticker = Ticker::default();
# fn demo(ticker: &Ticker, event: &makepad_widgets::Event) {
assert!(!ticker.fired(event));
# }
```
