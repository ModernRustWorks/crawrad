---
id: package-manager-installedkey
kind: doc
target_file: src/package_manager.rs
line_hint: 296
author: 
created: 
title: "InstalledKey"
tags: []
signature: "pub struct InstalledKey"
---

What the installed list depends on.

What the installed list depends on: the member, the package, and every
`Cargo.toml` from the member up to the disk root with its modification time
— a `workspace = true` dependency takes its version from an ancestor.

Equal key, equal list: `cargo metadata` is not run again. A `cargo add`
made by the IDE or outside it rewrites a manifest, and the key changes.

# Examples

```rust
use std::path::Path;
use crawrad_project::package_manager::InstalledKey;

let key = InstalledKey::of(Path::new("."), None);
assert_eq!(key, key.clone());
```
