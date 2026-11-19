---
id: navigation-history-answered
kind: doc
target_file: src/navigation.rs
line_hint: 181
author: 
created: 
title: "History::answered"
tags: []
signature: "pub fn answered (& mut self , ticket : u64) -> Option < Place >"
---

The answer for `ticket` arrived: where it was asked from, once.

# Examples

```rust
use crawrad_host::navigation::{History, Place};

let mut hist = History::default();
let ticket = hist.ask(Place::default());
assert_eq!(hist.answered(ticket), Some(Place::default()));
```
