---
id: navigation-history
kind: doc
target_file: src/navigation.rs
line_hint: 157
author: 
created: 
title: "History"
tags: []
signature: "pub struct History"
---

The back and forward stacks, and the jump still waiting for the server.

# Examples

```rust
use crawrad_host::navigation::History;

let hist = History::default();
assert!(!hist.can_go_back());
```
