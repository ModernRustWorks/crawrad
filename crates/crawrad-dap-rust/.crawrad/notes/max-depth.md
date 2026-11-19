---
id: max-depth
kind: doc
target_file: src/lib.rs
line_hint: 25
author: 
created: 
title: "MAX_DEPTH"
tags: []
signature: "pub const MAX_DEPTH : usize = 3"
---

How deep a value is expanded inside another (struct fields, enum payloads).

# Examples

```rust
use crawrad_dap_rust::MAX_DEPTH;

assert_eq!(MAX_DEPTH, 3);
```
