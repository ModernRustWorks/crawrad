---
id: explorer-no-folder
kind: doc
target_file: src/explorer.rs
line_hint: 90
author: 
created: 
title: "NO_FOLDER"
tags: []
signature: "pub const NO_FOLDER : & str = \"No folder open.\""
---

What the panel says when there is no folder to talk about.

The egui panel writes the same sentence in the same situation
(`explorer.rs`, the `No folder open.` arm). It is reachable HERE, which it
is not there: this shell opens a form by path and never asks for a folder,
so a form outside any `<member>/src/` really does leave the panel with
nothing to root the tree on.

# Examples

```rust
use crawrad_project::explorer::NO_FOLDER;

assert_eq!(NO_FOLDER, "No folder open.");
```
