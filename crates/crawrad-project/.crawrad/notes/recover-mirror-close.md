---
id: recover-mirror-close
kind: doc
target_file: src/recover.rs
line_hint: 104
author: 
created: 
title: "Mirror::close"
tags: []
signature: "pub fn close (& mut self)"
---

Closes the mirror, preserving only the waiting copies.

The IDE closes without losing anything: only the waiting copies stay,
to be offered again on the next start.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mut mirror = Mirror::default();
mirror.close();
```
