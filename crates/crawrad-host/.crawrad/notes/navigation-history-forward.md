---
id: navigation-history-forward
kind: doc
target_file: src/navigation.rs
line_hint: 210
author: 
created: 
title: "History::forward"
tags: []
signature: "pub fn forward (& mut self , here : Place) -> Option < Place >"
---

Where *Forward* goes from `here`, which *Back* will return to.

# Examples

```rust
use crawrad_host::navigation::{History, Place};

let mut hist = History::default();
hist.jumped_from(Place::default());
hist.back(Place::default());
assert!(hist.forward(Place::default()).is_some());
```
