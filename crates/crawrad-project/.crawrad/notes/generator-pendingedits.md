---
id: generator-pendingedits
kind: doc
target_file: src/generator.rs
line_hint: 224
author: 
created: 
title: "PendingEdits"
tags: []
signature: "pub trait PendingEdits"
---

Editor changes that must reach disk before the generation pre-pass.

What is being typed somewhere else and has to reach the disk **before** the
pre-pass reads it.

A trait, and not the `egui` shell's code tab, for the reason this whole
module exists: the Makepad shell has its own source panel, and a module
that named one of them would be reachable by one shell only. Two methods,
both of them the editor's, both of them ordered by [`prepare`] and
[`write`](fn@write) so that neither shell has to remember when to call them.

# Examples

```rust
use crawrad_project::generator::PendingEdits;

let mut edits: () = ();
assert!(edits.flush().is_ok());
```
