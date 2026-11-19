---
id: navigation-history-ask
kind: doc
target_file: src/navigation.rs
line_hint: 169
author: 
created: 
title: "History::ask"
tags: []
signature: "pub fn ask (& mut self , from : Place) -> u64"
---

Registers a Go to Definition request from the given origin place.

A *Go to Definition* is asked from `from`: the ticket its answer
must carry. Asking again makes the earlier ticket stale.

# Examples

```rust
use crawrad_host::navigation::{History, Place};

let mut hist = History::default();
let ticket = hist.ask(Place::default());
assert_eq!(hist.pending_ticket(), Some(ticket));
```
