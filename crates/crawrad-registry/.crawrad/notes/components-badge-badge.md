---
id: components-badge-badge
kind: doc
target_file: src/components/badge.rs
line_hint: 5
author: 
created: 
title: "badge"
tags: []
signature: "pub fn badge () -> ComponentDesc"
---

A short label in a pill badge for statuses or counters.

`Badge` (#500, wave 2): a short label in a pill: "New", "3", "Beta", "Error".
Graphic control: not a container, no TabOrder.

# Examples

```rust
use crawrad_registry::badge;

let desc = badge();
assert_eq!(desc.class, "Badge");
```
