---
id: generator-pendingedits-reload
kind: doc
target_file: src/generator.rs
line_hint: 229
author: 
created: 
title: "PendingEdits::reload"
tags: []
signature: "fn reload (& mut self) -> Vec < PathBuf >"
---

Reloads files rewritten by generation that have no unsaved edits.

Reload what the generation just rewrote underneath the editor, and name
what could NOT be reloaded because it had unsaved edits in it.

# Examples

```rust
use crawrad_project::generator::PendingEdits;

let mut edits: () = ();
assert!(edits.reload().is_empty());
```
