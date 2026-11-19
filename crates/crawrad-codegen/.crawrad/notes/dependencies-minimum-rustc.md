---
id: dependencies-minimum-rustc
kind: doc
target_file: src/dependencies.rs
line_hint: 169
author: 
created: 
title: "minimum_rustc"
tags: []
signature: "pub fn minimum_rustc (target : AlvoGerador , with_form : bool) -> (u32 , u32)"
---

Returns the minimum `rustc` version `(major, minor)` required for a target.

The oldest `rustc` that builds what [`dependencias_do_projeto`] puts in a
project for `target`, with or without a form, as `(major, minor)`. Measured on 30/10/2026:
- no form: 1.85 — the 2024 edition; nothing of ours is added;
- Makepad: 1.88 — our own crates use let-chains (1.87 refuses them, E0658);
- Egui: 1.95 — `eframe` 0.36 declares `rust-version = "1.95"`.

Raise it in the same change that raises a dependency or adopts a newer
language feature in a crate the generated project depends on.

# Examples

```rust
use crawrad_codegen::{AlvoGerador, minimum_rustc};

let (major, minor) = minimum_rustc(AlvoGerador::Egui, true);
assert_eq!((major, minor), (1, 95));
```
