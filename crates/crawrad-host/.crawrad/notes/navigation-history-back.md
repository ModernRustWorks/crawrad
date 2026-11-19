---
id: navigation-history-back
kind: doc
target_file: src/navigation.rs
line_hint: 203
author: 
created: 
title: "History::back"
tags: []
signature: "pub fn back (& mut self , here : Place) -> Option < Place >"
---

Where *Back* goes from `here`, which *Forward* will return to.

# Examples

```rust
use crawrad_host::navigation::{History, Place};

let mut hist = History::default();
let origin = Place::default();
hist.jumped_from(origin.clone());
assert_eq!(hist.back(Place::default()), Some(origin));
```
