---
id: workspace-has-package
kind: doc
target_file: src/workspace.rs
line_hint: 120
author: 
created: 
title: "has_package"
tags: []
signature: "pub fn has_package (cargo_toml : & str) -> bool"
---

Checks whether this `Cargo.toml` text declares a package.

Does this `Cargo.toml` text declare a package?

Serves to distinguish the **group** (virtual root) from a standalone package
that someone opened directly, and is the only difference between the two
that matters here.

# Examples

```rust
use crawrad_project::workspace::has_package;

assert!(has_package("[package]\nname = \"demo\"\n"));
assert!(!has_package("[workspace]\n"));
```
