---
id: generator-has-a-target
kind: doc
target_file: src/generator.rs
line_hint: 150
author: 
created: 
title: "has_a_target"
tags: []
signature: "pub fn has_a_target (member : & Path) -> bool"
---

The package name declared in a member's `Cargo.toml`.

`app` when the manifest is missing or says nothing — the generator needs a
name to write the `use`, and refusing to generate because of an unreadable
`Cargo.toml` would trade a crooked name for no code at all.
Whether `member` has a target cargo can build — `src/main.rs` or
`src/lib.rs`. A form's project gets `main.rs` only when it is first
generated (save or run); until then cargo calls the manifest invalid
("no targets specified") and rust-analyzer analyses nothing in it.

# Examples

```rust
use std::path::Path;
use crawrad_project::generator::has_a_target;

assert!(!has_a_target(Path::new("missing_path")));
```
