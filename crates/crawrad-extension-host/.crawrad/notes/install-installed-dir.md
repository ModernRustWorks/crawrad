---
id: install-installed-dir
kind: doc
target_file: src/install.rs
line_hint: 105
author: 
created: 
title: "installed_dir"
tags: []
signature: "pub fn installed_dir (root : & Path , user_dir : & Path , id : & str) -> Option < PathBuf >"
---

The installed folder of extension `id` for the project at `root`.

Its user-store folder when the project lists it, else an older project's own folder.

# Examples

```rust
use crawrad_extension_host::install::installed_dir;
use std::path::Path;

assert!(installed_dir(Path::new("nonexistent"), Path::new("nostore"), "ext1").is_none());
```
