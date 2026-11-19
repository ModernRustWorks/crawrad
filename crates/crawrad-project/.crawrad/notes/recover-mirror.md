---
id: recover-mirror
kind: doc
target_file: src/recover.rs
line_hint: 34
author: 
created: 
title: "Mirror"
tags: []
signature: "pub struct Mirror"
---

Manages recovery copies of the currently open project.

The recovery copies of the open project, as the IDE runs: which project,
and which copies a crash left that still wait for an answer.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mirror = Mirror::default();
assert_eq!(mirror.root(), None);
```
