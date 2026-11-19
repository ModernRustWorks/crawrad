---
id: workspace-has-binary
kind: doc
target_file: src/workspace.rs
line_hint: 1857
author: 
created: 
title: "has_binary"
tags: []
signature: "pub fn has_binary (root : & std :: path :: Path , package : & str) -> bool"
---

Checks if the compiled package has an executable binary target.

Does the package that `Run` just compiled have an executable target?

**Open when unknown**, and the choice is the same that `crawrad_actions::Projeto::default`
writes out in full: when unable to ask `cargo` — it is not on `PATH`, manifest is
broken —, the IDE does not invent a verdict. Refusing launch due to a failed
query would exchange a readable error for a silent dead button.

Free function and not method because it **never looks at the screen**:
receives folder and name, asks `cargo`, and answers. That also keeps it within
reach for testing against real projects.

The query costs 85 ms measured ([`group_executables`]), and is made ONCE
per `Run` — after a compilation taking seconds or minutes. Intentionally without
cache: cache is state to invalidate, and here there is no frame loop to amortize it.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::workspace::has_binary;

// Avoid spawning cargo processes during doctests.
assert!(has_binary(Path::new("."), "non_existent"));
```
