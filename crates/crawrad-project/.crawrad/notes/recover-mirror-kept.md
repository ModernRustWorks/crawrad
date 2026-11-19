---
id: recover-mirror-kept
kind: doc
target_file: src/recover.rs
line_hint: 59
author: 
created: 
title: "Mirror::kept"
tags: []
signature: "pub fn kept (& self) -> & [PathBuf]"
---

The copies still waiting for an answer.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mirror = Mirror::default();
assert!(mirror.kept().is_empty());
```
