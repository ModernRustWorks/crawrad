---
id: events-second-click
kind: doc
target_file: src/events.rs
line_hint: 53
author: 
created: 
title: "second_click"
tags: []
signature: "pub fn second_click (last : & mut ClickRecord , row : usize , now : f64) -> bool"
---

Checks whether a click on `row` at `now` is the second of a double click.

Whether a click on `row` at `now` is the second of a double click: the
same row again within [`DOUBLE_CLICK_SECONDS`]. A double click starts the
count over, so a third quick click is a first one again.

One clock for every list (#439): the `ListView` kept its own 0.4 s and
the `ListBox` had none, so the same gesture meant two different things.

# Examples

```rust
use crawrad_runtime_makepad::events::{second_click, ClickRecord};

let mut last: ClickRecord = None;
assert!(!second_click(&mut last, 1, 1.0));
assert!(second_click(&mut last, 1, 1.2));
```
