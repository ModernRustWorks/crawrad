---
id: recover-discard
kind: doc
target_file: src/recover.rs
line_hint: 164
author: 
created: 
title: "discard"
tags: []
signature: "pub fn discard (root : & Path , files : & [PathBuf])"
---

Deletes the copies of `files` (relative to the project).

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::recover::discard;

discard(Path::new("."), &[]);
```
