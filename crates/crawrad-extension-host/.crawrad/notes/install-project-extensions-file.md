---
id: install-project-extensions-file
kind: doc
target_file: src/install.rs
line_hint: 32
author: 
created: 
title: "project_extensions_file"
tags: []
signature: "pub fn project_extensions_file (root : & Path) -> PathBuf"
---

The project's list of the extensions it uses: `<root>/.crawrad/extensions.json`.

# Examples

```rust
use crawrad_extension_host::install::project_extensions_file;
use std::path::Path;

let file = project_extensions_file(Path::new("app"));
assert!(file.ends_with(".crawrad/extensions.json"));
```
