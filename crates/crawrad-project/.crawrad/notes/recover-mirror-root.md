---
id: recover-mirror-root
kind: doc
target_file: src/recover.rs
line_hint: 54
author: 
created: 
title: "Mirror::root"
tags: []
signature: "pub fn root (& self) -> Option < & Path >"
---

The open project.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mirror = Mirror::default();
assert_eq!(mirror.root(), None);
```
