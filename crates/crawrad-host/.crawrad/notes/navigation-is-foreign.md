---
id: navigation-is-foreign
kind: doc
target_file: src/navigation.rs
line_hint: 264
author: 
created: 
title: "is_foreign"
tags: []
signature: "pub fn is_foreign (file : & Path , roots : & [PathBuf]) -> bool"
---

Checks whether the given file path lies within any of the foreign roots.

Is `file` under one of `roots`? Compared without case on Windows paths
and with either slash, since a path from the server and one from the
environment rarely agree on spelling.

# Examples

```rust
use crawrad_host::navigation::is_foreign;
use std::path::{Path, PathBuf};

let roots = vec![PathBuf::from("/rustup")];
assert!(is_foreign(Path::new("/rustup/toolchains/lib.rs"), &roots));
assert!(!is_foreign(Path::new("/project/src/lib.rs"), &roots));
```
