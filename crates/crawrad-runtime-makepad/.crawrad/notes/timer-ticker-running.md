---
id: timer-ticker-running
kind: doc
target_file: src/timer.rs
line_hint: 43
author: 
created: 
title: "Ticker::running"
tags: []
signature: "pub fn running (& self) -> Option < u64 >"
---

The interval running now, in ms (`None`: stopped).

# Examples

```rust
use crawrad_runtime_makepad::timer::Ticker;

let ticker = Ticker::default();
assert_eq!(ticker.running(), None);
```
