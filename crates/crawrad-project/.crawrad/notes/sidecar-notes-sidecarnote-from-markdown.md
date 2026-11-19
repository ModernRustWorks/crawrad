---
id: sidecar-notes-sidecarnote-from-markdown
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 102
author: 
created: 
title: "SidecarNote::from_markdown"
tags: []
signature: "pub fn from_markdown (text : & str) -> Result < Self , String >"
---

Decodes a note from Markdown text with YAML frontmatter.

# Errors

Returns an error message if the text cannot be parsed as a note file.

# Examples

```rust
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::new("n1".into(), "src/lib.rs", 5);
let md = note.to_markdown();
let decoded = SidecarNote::from_markdown(&md).unwrap();
assert_eq!(decoded.id, "n1");
```
