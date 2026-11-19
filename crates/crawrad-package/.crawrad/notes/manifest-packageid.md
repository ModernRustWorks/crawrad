---
id: manifest-packageid
kind: doc
target_file: src/manifest.rs
line_hint: 175
author: 
created: 
title: "PackageId"
tags: []
signature: "pub struct PackageId"
---

Who the package is — and only that.

# Examples

```rust
use crawrad_package::PackageId;

let pkg = PackageId {
    name: "buttons".into(),
    version: "0.1.0".into(),
    description: "Standard buttons".into(),
};
assert_eq!(pkg.name, "buttons");
```
