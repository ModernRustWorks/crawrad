---
id: installed-catalog
kind: doc
target_file: src/installed.rs
line_hint: 140
author: 
created: 
title: "catalog"
tags: []
signature: "pub fn catalog () -> InstalledPackages"
---

The IDE's catalogue.

Builtin, plus every package that allows itself to be loaded.

# Examples

```rust
use crawrad_project::installed::catalog;

let cat = catalog();
assert!(cat.catalog.all().count() > 0);
```
