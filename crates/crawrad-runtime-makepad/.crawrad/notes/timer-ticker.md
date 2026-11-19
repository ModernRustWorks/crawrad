---
id: timer-ticker
kind: doc
target_file: src/timer.rs
line_hint: 14
author: 
created: 
title: "Ticker"
tags: []
signature: "pub struct Ticker"
---

The platform interval behind one form `Timer`.

# Examples

```rust
use crawrad_runtime_makepad::timer::Ticker;

let ticker = Ticker::default();
assert_eq!(ticker.running(), None);
```
