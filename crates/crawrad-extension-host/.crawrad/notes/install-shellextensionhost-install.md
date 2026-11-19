---
id: install-shellextensionhost-install
kind: doc
target_file: src/install.rs
line_hint: 72
author: 
created: 
title: "ShellExtensionHost::install"
tags: []
signature: "pub fn install (& mut self , root : & Path , source : & Path ,) -> Result < ExtensionManifest , ExtensionError >"
---

Installs the extension in `source` into the project at `root`.

Installs the extension in `source` into the project at `root` and
registers it now. Refused when its id is already installed.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;
use std::path::Path;

let mut host = ShellExtensionHost::default();
assert!(host.install(Path::new("dummy_root"), Path::new("dummy_source")).is_err());
```

# Errors

Returns [`ExtensionError`](crawrad_extension_api::error::ExtensionError) if the manifest cannot be read, is duplicate, or copying files fails.
