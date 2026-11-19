---
id: navigation-history-jumped-from
kind: doc
target_file: src/navigation.rs
line_hint: 192
author: 
created: 
title: "History::jumped_from"
tags: []
signature: "pub fn jumped_from (& mut self , from : Place)"
---

A jump is leaving `from`: remember it, forget the forward side.

# Examples

```rust
use crawrad_host::navigation::{History, Place};

let mut hist = History::default();
hist.jumped_from(Place::default());
assert!(hist.can_go_back());
```
