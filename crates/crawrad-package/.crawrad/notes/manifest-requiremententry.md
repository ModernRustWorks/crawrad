---
id: manifest-requiremententry
kind: doc
target_file: src/manifest.rs
line_hint: 652
author: 
created: 
title: "RequirementEntry"
tags: []
signature: "pub struct RequirementEntry"
---

A dependency that the generated project gains because of a component.

# It is the classic RAD *driver link*, and the FEATURES are the point

In the classic RAD IDE, dropping a SQLite driver link onto the form is what makes the
SQLite driver LINKED into the application. In Rust the exact equivalent is the
Cargo *feature*: without `portadb/rusqlite`, the driver module is not even
compiled — `drivers/mod.rs` is entirely `#[cfg(feature = …)]`.

That is why one component per provider is not redundancy with the abstraction: that of the
abstraction says *"I want to connect"*, and that of the provider says *"and THIS driver
is the one that must exist in the binary"*.

The union of requirements of all form components is what goes
into the generated `Cargo.toml`.

# Examples

```rust
use crawrad_package::manifest::RequirementEntry;

let req = RequirementEntry {
    crate_name: "serde".into(),
    version: "1.0".into(),
    features: vec!["derive".into()],
    default_features: true,
};
assert_eq!(req.crate_name, "serde");
```
