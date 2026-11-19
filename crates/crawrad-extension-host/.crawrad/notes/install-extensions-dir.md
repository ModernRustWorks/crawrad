---
id: install-extensions-dir
kind: doc
target_file: src/install.rs
line_hint: 16
author: 
created: 
title: "extensions_dir"
tags: []
signature: "pub fn extensions_dir (root : & Path) -> PathBuf"
---

The folder the project's extensions live in.

# Examples

```rust
use crawrad_extension_host::install::extensions_dir;
use std::path::Path;

let p = extensions_dir(Path::new("/root"));
assert!(p.ends_with(".crawrad/extensions") || p.ends_with(r".crawrad\extensions"));
```
