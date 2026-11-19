---
id: events-pointer
kind: doc
target_file: src/events.rs
line_hint: 31
author: 
created: 
title: "Pointer"
tags: []
signature: "pub enum Pointer"
---

Which pointer event a control gets.

# Examples

```rust
use crawrad_runtime_makepad::events::Pointer;

let event = Pointer::Down;
assert_eq!(event, Pointer::Down);
```
