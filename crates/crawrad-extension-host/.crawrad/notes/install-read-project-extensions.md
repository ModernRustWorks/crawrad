---
id: install-read-project-extensions
kind: doc
target_file: src/install.rs
line_hint: 37
author: 
created: 
title: "read_project_extensions"
tags: []
signature: "pub fn read_project_extensions (root : & Path) -> Result < ProjectExtensions , ExtensionError >"
---

The extensions the project at `root` uses; no list file means none.

# Errors

Returns an error if the list file exists but cannot be read or is not valid JSON.

# Examples

```rust
use crawrad_extension_host::install::read_project_extensions;
use std::path::Path;

let list = read_project_extensions(Path::new("nonexistent")).unwrap();
assert!(list.extensions.is_empty());
```
