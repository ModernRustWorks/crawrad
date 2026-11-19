---
id: dependencies-patch-makepad-platform-at
kind: doc
target_file: src/dependencies.rs
line_hint: 349
author: 
created: 
title: "patch_makepad_platform_at"
tags: []
signature: "pub fn patch_makepad_platform_at (manifest_dir : & Path) -> String"
---

Platform patch with the vendored path relative to the manifest.

# Examples

```rust
use crawrad_codegen::dependencies::patch_makepad_platform_at;

let patch = patch_makepad_platform_at(std::path::Path::new("."));
assert!(patch.is_empty() || patch.contains("[patch.crates-io]"));
```
