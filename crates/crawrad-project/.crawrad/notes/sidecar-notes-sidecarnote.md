---
id: sidecar-notes-sidecarnote
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 15
author: 
created: 
title: "SidecarNote"
tags: []
signature: "pub struct SidecarNote"
---

A decoupled code annotation (Sidecar Note).

# Examples

```rust
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::default();
assert_eq!(note.author, "user");
```
