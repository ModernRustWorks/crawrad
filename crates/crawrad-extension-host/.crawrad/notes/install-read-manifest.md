---
id: install-read-manifest
kind: doc
target_file: src/install.rs
line_hint: 21
author: 
created: 
title: "read_manifest"
tags: []
signature: "pub fn read_manifest (dir : & Path) -> Result < ExtensionManifest , ExtensionError >"
---

The manifest a folder holds (`manifest.json`, else `extension.json`).

# Examples

```rust
use crawrad_extension_host::install::read_manifest;
use std::path::Path;

assert!(read_manifest(Path::new("nonexistent")).is_err());
```

# Errors

Returns [`ExtensionError::InvalidManifest`](crawrad_extension_api::error::ExtensionError::InvalidManifest) if no manifest file exists or it fails to parse.
