---
id: notes-panel-index-project
kind: doc
target_file: src/notes_panel.rs
line_hint: 520
author: 
created: 
title: "index_project"
tags: []
signature: "pub fn index_project (root : & Path) -> Vec < IndexedNote >"
---

Indexes every note of the project at `root`, placed at its first anchor.

Every note of the project at `root`, each where its first anchor is: the
`.rs` files under `root` are walked in path order (skipping `target`,
`node_modules` and hidden directories), reading line anchors
(`// [crawrad:note#id]`, `// [crawrad:comment#id]`) and documentation
attributes (`#[note("id")]`). A note no file anchors comes last, unanchored.

# Examples

```rust
use std::path::Path;
use crawrad_project::notes_panel::index_project;

let notes = index_project(Path::new("non_existent_dir_12345"));
assert!(notes.is_empty());
```
