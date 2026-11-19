---
id: install-shellextensionhost-uninstall
kind: doc
target_file: src/install.rs
line_hint: 93
author: 
created: 
title: "ShellExtensionHost::uninstall"
tags: []
signature: "pub fn uninstall (& mut self , root : & Path , id : & str) -> Result < () , ExtensionError >"
---

Uninstalls an extension by its identifier.

Uninstalls extension `id`: its process stopped, everything it
contributed taken back, its folder under `root` deleted.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;
use std::path::Path;

let mut host = ShellExtensionHost::default();
assert!(host.uninstall(Path::new("dummy_root"), "unknown").is_err());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the extension is not registered, or an IO error if folder deletion fails.
