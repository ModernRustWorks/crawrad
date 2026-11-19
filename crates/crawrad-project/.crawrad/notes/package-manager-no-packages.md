---
id: package-manager-no-packages
kind: doc
target_file: src/package_manager.rs
line_hint: 602
author: 
created: 
title: "NO_PACKAGES"
tags: []
signature: "pub const NO_PACKAGES : & str = \"No packages installed in this project.\""
---

What the `Installed` tab says when the manifest declares no dependency.

# Examples

```rust
use crawrad_project::package_manager::NO_PACKAGES;

assert_eq!(NO_PACKAGES, "No packages installed in this project.");
```
