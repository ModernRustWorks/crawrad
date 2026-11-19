---
id: installation-verify-package
kind: doc
target_file: src/installation.rs
line_hint: 89
author: 
created: 
title: "verify_package"
tags: []
signature: "pub fn verify_package (origin : & Path , already_installed : & Registry ,) -> Result < VerifiedPackage , String >"
---

**Can this package enter?** — through the same gate loading uses.

`already_installed` is the current catalog: it is against it that rejection
for repeated class is measured, which is why it is caught **before** copying.

# Errors

Returns an error message if the package manifest is missing, invalid,
or conflicts with the existing catalog.

# Examples

```rust
use std::path::Path;
use crawrad_registry::Registry;
use crawrad_project::installation::verify_package;

let reg = Registry::builtin();
let res = verify_package(Path::new("missing_pkg_dir"), &reg);
assert!(res.is_err());
```
