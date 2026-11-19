---
id: sidecar-notes-sidecarnote-current-timestamp
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 78
author: 
created: 
title: "SidecarNote::current_timestamp"
tags: []
signature: "pub fn current_timestamp () -> String"
---

Generates an approximate ISO-8601 timestamp string.

# Examples

```rust
use crawrad_project::sidecar_notes::SidecarNote;

let ts = SidecarNote::current_timestamp();
assert!(!ts.is_empty());
```
