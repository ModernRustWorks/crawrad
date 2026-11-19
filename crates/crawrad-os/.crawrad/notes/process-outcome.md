---
id: process-outcome
kind: doc
target_file: src/process.rs
line_hint: 139
author: 
created: 
title: "Outcome"
tags: []
signature: "pub struct Outcome"
---

Everything a finished child left behind — **both streams, whole**.

Whole is the whole point. See the module header, and issue #67.

# Examples

```rust
use crawrad_os::process::Outcome;

let outcome = Outcome::default();
assert!(!outcome.success);
```
