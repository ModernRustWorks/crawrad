---
id: package-manager-no-packages-hint
kind: doc
target_file: src/package_manager.rs
line_hint: 605
author: 
created: 
title: "NO_PACKAGES_HINT"
tags: []
signature: "pub const NO_PACKAGES_HINT : & str = \"Go to the 'Browse (crates.io)' tab to search and add packages to this project.\""
---

The follow-up of [`NO_PACKAGES`].

# Examples

```rust
use crawrad_project::package_manager::NO_PACKAGES_HINT;

assert!(!NO_PACKAGES_HINT.is_empty());
```
