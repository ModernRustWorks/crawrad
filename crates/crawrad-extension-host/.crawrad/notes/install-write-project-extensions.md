---
id: install-write-project-extensions
kind: doc
target_file: src/install.rs
line_hint: 47
author: 
created: 
title: "write_project_extensions"
tags: []
signature: "pub fn write_project_extensions (root : & Path , mut exts : ProjectExtensions ,) -> Result < () , ExtensionError >"
---

Writes the project's extension list, sorted by id.

Creates `.crawrad/` when needed.

# Errors

Returns an error if the folder or the file cannot be written.

# Examples

```rust
use crawrad_extension_host::install::{read_project_extensions, write_project_extensions, ExtensionRef, ProjectExtensions};

let root = std::env::temp_dir().join(format!("rrad-doc-extlist-{}", std::process::id()));
let list = ProjectExtensions { extensions: vec![
    ExtensionRef { id: "b".into(), version: "1.0.0".into() },
    ExtensionRef { id: "a".into(), version: "1.0.0".into() },
] };
write_project_extensions(&root, list).unwrap();
assert_eq!(read_project_extensions(&root).unwrap().extensions[0].id, "a");
let _ = std::fs::remove_dir_all(&root);
```
