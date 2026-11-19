---
id: debug-phase
kind: doc
target_file: src/debug.rs
line_hint: 16
author: 
created: 
title: "Phase"
tags: []
signature: "pub enum Phase"
---

Where the session is.

# Examples

```rust
use crawrad_host::Phase;

let phase = Phase::Idle;
assert_eq!(phase, Phase::Idle);
```
