---
id: install-legacy-installed-dir
kind: doc
target_file: src/install.rs
line_hint: 92
author: 
created: 
title: "legacy_installed_dir"
tags: []
signature: "pub fn legacy_installed_dir (root : & Path , id : & str) -> Option < PathBuf >"
---

The folder of extension `id` in an older project's own extensions folder.

That is `.crawrad/extensions/<any name>/`, found by the manifest's id.

# Examples

```rust
use crawrad_extension_host::install::legacy_installed_dir;
use std::path::Path;

assert!(legacy_installed_dir(Path::new("nonexistent"), "ext1").is_none());
```
