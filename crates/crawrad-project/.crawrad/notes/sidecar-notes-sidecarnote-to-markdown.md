---
id: sidecar-notes-sidecarnote-to-markdown
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 83
author: 
created: 
title: "SidecarNote::to_markdown"
tags: []
signature: "pub fn to_markdown (& self) -> String"
---

Serializes the note to Markdown with YAML frontmatter.

# Examples

```rust
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::new("n1".into(), "src/lib.rs", 5);
let md = note.to_markdown();
assert!(md.contains("---"));
```
