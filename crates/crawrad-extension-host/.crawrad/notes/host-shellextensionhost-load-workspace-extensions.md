---
id: host-shellextensionhost-load-workspace-extensions
kind: doc
target_file: src/host.rs
line_hint: 172
author: 
created: 
title: "ShellExtensionHost::load_workspace_extensions"
tags: []
signature: "pub fn load_workspace_extensions_from (& mut self , workspace_root : & Path , user_dir : & Path)"
---

Load and register the project's extensions.

Its list resolved in the user store, then legacy folders.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;
use std::path::Path;

let mut host = ShellExtensionHost::default();
host.load_workspace_extensions_from(Path::new("nonexistent"), Path::new("nostore"));
assert!(host.manifests.is_empty());
```
