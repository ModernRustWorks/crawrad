---
id: debug-output-tail
kind: doc
target_file: src/debug.rs
line_hint: 53
author: 
created: 
title: "OUTPUT_TAIL"
tags: []
signature: "pub const OUTPUT_TAIL : usize = 50"
---

How many of the program's last output pieces `debug.state` carries.

# Examples

```rust
use crawrad_host::debug::OUTPUT_TAIL;

assert_eq!(OUTPUT_TAIL, 50);
```
