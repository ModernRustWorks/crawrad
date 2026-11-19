---
id: open
kind: doc
target_file: src/lib.rs
line_hint: 257
author: 
created: 
title: "open"
tags: []
signature: "pub fn open (progress : fn () -> (usize , usize , usize))"
---

Opens the splash. `progress` reads the shader counter: done, total, compiled.

# Examples

```rust,no_run
// spawns an OS splash window on a background thread
use crawrad_splash::open;

open(|| (0, 0, 0));
```
