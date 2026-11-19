---
id: craterequirement
kind: doc
target_file: src/lib.rs
line_hint: 454
author: 
created: 
title: "CrateRequirement"
tags: []
signature: "pub struct CrateRequirement"
---

What the GENERATED project needs in its `Cargo.toml` because of this
component.

# Why this lives in the descriptor

A component that emits code emits code that **calls something**, and
that thing lives in a crate. The native `Button` needs nothing because the
generator already writes the application's `eframe`; a third-party component
needs to declare where what it calls comes from, otherwise the generated project
does not compile and the fault has no name.

# And FEATURES are the main reason, not a detail

It is the SQLite *driver link* of the classic RAD IDE: there, dropping the *driver link*
on the form is what causes the SQLite driver to be LINKED into the application. In
Rust the exact equivalent is the Cargo *feature* — without
`portadb/rusqlite`, the driver module is not even compiled
(`drivers/mod.rs` is all `#[cfg(feature = …)]`).

That is why one component per provider is not redundancy with the abstraction: the
abstraction component says *"I want to connect"*, and the provider component says *"and it is
THIS driver that must exist in the binary"*.

# Examples

```rust
use crawrad_registry::CrateRequirement;

let req = CrateRequirement {
    crate_name: "rusqlite",
    version: "0.29",
    features: &["bundled"],
    default_features: true,
};
assert_eq!(req.crate_name, "rusqlite");
```
