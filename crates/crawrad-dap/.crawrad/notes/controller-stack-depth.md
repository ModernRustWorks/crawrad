---
id: controller-stack-depth
kind: doc
target_file: src/controller.rs
line_hint: 31
author: 
created: 
title: "STACK_DEPTH"
tags: []
signature: "pub const STACK_DEPTH : i64 = 64"
---

How many frames of the stack a stop reports.

# Examples

```rust
use crawrad_dap::controller::STACK_DEPTH;

assert!(STACK_DEPTH > 0);
```
