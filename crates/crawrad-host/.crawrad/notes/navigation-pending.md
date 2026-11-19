---
id: navigation-pending
kind: doc
target_file: src/navigation.rs
line_hint: 69
author: 
created: 
title: "Pending"
tags: []
signature: "pub struct Pending"
---

Tracks a single pending request by ticket for file formatting.

A request that waits for one answer about one file at one version
(Format Document): the newest ticket only.

# Examples

```rust
use crawrad_host::navigation::Pending;

let pending = Pending::default();
assert_eq!(pending.ticket(), None);
```
