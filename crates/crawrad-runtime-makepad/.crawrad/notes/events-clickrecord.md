---
id: events-clickrecord
kind: doc
target_file: src/events.rs
line_hint: 50
author: 
created: 
title: "ClickRecord"
tags: []
signature: "pub type ClickRecord = Option < (usize , f64) >"
---

Records the last clicked row and timestamp in seconds.

The row clicked last and when, in seconds since the app started: what a
list needs to tell a double click from two clicks.

# Examples

```rust
use crawrad_runtime_makepad::events::ClickRecord;

let record: ClickRecord = Some((0, 1.25));
assert_eq!(record, Some((0, 1.25)));
```
