---
id: host-shellextensionhost-scan-extensions
kind: doc
target_file: src/host.rs
line_hint: 144
author: 
created: 
title: "ShellExtensionHost::scan_extensions"
tags: []
signature: "pub fn scan_extensions (workspace_root : & Path , user_dir : & Path) -> Vec < ExtensionManifest >"
---

Scan a workspace root directory for declarative extension manifests.

The project's list resolved in the user store (`user_dir/<id>@<version>`), then an older
project's own `.crawrad/extensions/` folders; one manifest per id, the list first.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;
use std::path::Path;

let manifests = ShellExtensionHost::scan_extensions(Path::new("nonexistent"), Path::new("nostore"));
assert!(manifests.is_empty());
```
