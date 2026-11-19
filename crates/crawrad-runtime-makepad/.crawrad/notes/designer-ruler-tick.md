---
id: designer-ruler-tick
kind: doc
target_file: src/designer/ruler.rs
line_hint: 44
author: 
created: 
title: "Tick"
tags: []
signature: "pub struct Tick"
---

One ruler tick.

# Examples

```rust
use crawrad_runtime_makepad::designer::Tick;

let tick = Tick { value: 50.0, on_screen: 50.0, major: true };
assert!(tick.major);
```
