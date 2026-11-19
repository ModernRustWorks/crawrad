---
id: installed-installedpackages
kind: doc
target_file: src/installed.rs
line_hint: 47
author: 
created: 
title: "InstalledPackages"
tags: []
signature: "pub struct InstalledPackages"
---

The ready catalogue, plus what was refused along the way.

# Examples

```rust
use crawrad_project::installed::InstalledPackages;
use crawrad_registry::Registry;
use std::collections::BTreeMap;

let ip = InstalledPackages {
    catalog: Registry::builtin(),
    refusals: Vec::new(),
    origin: BTreeMap::new(),
    packages: Vec::new(),
    scope: BTreeMap::new(),
};
assert!(ip.refusals.is_empty());
```
