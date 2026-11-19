---
id: navigation-history-pending-ticket
kind: doc
target_file: src/navigation.rs
line_hint: 176
author: 
created: 
title: "History::pending_ticket"
tags: []
signature: "pub fn pending_ticket (& self) -> Option < u64 >"
---

The ticket still waiting, if any.

# Examples

```rust
use crawrad_host::navigation::History;

let hist = History::default();
assert_eq!(hist.pending_ticket(), None);
```
