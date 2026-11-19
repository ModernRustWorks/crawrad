---
id: timer-ticker-sync
kind: doc
target_file: src/timer.rs
line_hint: 23
author: 
created: 
title: "Ticker::sync"
tags: []
signature: "pub fn sync (& mut self , cx : & mut Cx , enabled : bool , interval_ms : u64)"
---

Synchronizes the interval with form timer properties.

Starts, restarts or stops the interval so it matches the form's
`enabled` and `interval_ms` (a zero interval is stopped, as in a
classic RAD IDE). It only touches the platform when they change: the
sync runs after every event.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::timer::Ticker;

let mut ticker = Ticker::default();
# fn demo(ticker: &mut Ticker, cx: &mut makepad_widgets::Cx) {
ticker.sync(cx, true, 1000);
assert_eq!(ticker.running(), Some(1000));
# }
```
