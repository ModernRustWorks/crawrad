---
id: recover-mirror-settle
kind: doc
target_file: src/recover.rs
line_hint: 74
author: 
created: 
title: "Mirror::settle"
tags: []
signature: "pub fn settle (& mut self , files : & [PathBuf])"
---

`files` were recovered or discarded: they no longer wait.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mut mirror = Mirror::default();
mirror.settle(&[]);
assert!(mirror.kept().is_empty());
```
