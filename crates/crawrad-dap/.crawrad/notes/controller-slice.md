---
id: controller-slice
kind: doc
target_file: src/controller.rs
line_hint: 28
author: 
created: 
title: "SLICE"
tags: []
signature: "pub const SLICE : Duration = Duration :: from_millis (100)"
---

How long the thread waits for a stop before it looks at the commands again.

# Examples

```rust
use crawrad_dap::controller::SLICE;

assert!(SLICE.as_millis() > 0);
```
