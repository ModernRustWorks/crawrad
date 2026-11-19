---
id: dependencies-manifest-path
kind: doc
target_file: src/dependencies.rs
line_hint: 379
author: 
created: 
title: "manifest_path"
tags: []
signature: "pub fn manifest_path (manifest_dir : & Path , target : & Path) -> PathBuf"
---

Relative path from a manifest folder; absolute across drives.

# Examples

```rust
use crawrad_codegen::dependencies::manifest_path;
use std::path::Path;

let p = manifest_path(Path::new("/w/examples/app"), Path::new("/w/crates/crawrad-runtime"));
assert_eq!(p, Path::new("../../crates/crawrad-runtime"));
```
