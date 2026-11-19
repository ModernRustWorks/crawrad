---
id: explorer-group-root-of
kind: doc
target_file: src/explorer.rs
line_hint: 236
author: 
created: 
title: "group_root_of"
tags: []
signature: "pub fn group_root_of (document : & Path) -> Option < PathBuf >"
---

The group root of a document — the folder the whole tree hangs from.

The egui shell gets it from `Open Folder`; this shell opens a form by path
and has no folder to be told about, so the root is DERIVED: the member is
the grandparent of the form ([`workspace::document_member`]), and the
group is the folder above it when that folder's manifest declares the
member.

When it does not, the member IS the group — which is exactly what
[`workspace::members`] answers for a lone package, and what `cargo` sees
when it is pointed at a folder with `[package]` and no `[workspace]`.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer::group_root_of;

assert!(group_root_of(Path::new("missing")).is_none());
```
