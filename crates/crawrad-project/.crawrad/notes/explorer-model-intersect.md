---
id: explorer-model-intersect
kind: doc
target_file: src/explorer_model.rs
line_hint: 69
author: 
created: 
title: "intersect"
tags: []
signature: "pub fn intersect (plan : & [Arquivo] , member : & Path) -> Vec < Line >"
---

Crosses the generator plan with what is on disk for a member.

Crosses the generator plan with what is on disk for a **member**.

The disk scan is **deliberately narrow**: `src/*.rs` non-recursive, plus
the `Cargo.toml` in the member folder. Scanning the entire root would call
`Cargo.lock` and `target/` stale, and a panel that reports dirt where there
is none is a panel people learn to ignore.

`Cargo.toml` enters **via disk**, not via plan: since [ADR 0018] the generator
no longer writes it — `cargo new` creates it —, and it disappeared from screen
along with it. But it is the most important file of the project, and the
information the panel gives about it became the best possible: it belongs to
the user **from the very first second**, not "ours the first time".

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_model::intersect;

let lines = intersect(&[], Path::new("."));
assert!(lines.iter().all(|l| !l.path.as_os_str().is_empty()));
```
