---
id: events-raw
kind: doc
target_file: src/events.rs
line_hint: 60
author: 
created: 
title: "Raw"
tags: []
signature: "pub enum Raw"
---

The raw pointer events the router reads.

# Examples

```rust
use crawrad_runtime_makepad::events::Raw;

let raw = Raw::Down;
assert_eq!(raw, Raw::Down);
```
