---
id: sidecar-notes-extract-anchors
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 136
author: 
created: 
title: "extract_anchors"
tags: []
signature: "pub fn extract_anchors (code : & str) -> Vec < AnchorMatch >"
---

Extracts all anchors present in the source code text.

# Examples

```rust
use crawrad_project::sidecar_notes::extract_anchors;

let anchors = extract_anchors("// [crawrad:note#intro]\nfn main() {}");
assert_eq!(anchors.len(), 1);
assert_eq!(anchors[0].note_id, "intro");
```
