---
id: sidecar-notes-anchormatch
kind: doc
target_file: src/sidecar_notes.rs
line_hint: 121
author: 
created: 
title: "AnchorMatch"
tags: []
signature: "pub struct AnchorMatch"
---

Occurrence of an anchor found in source code.

# Examples

```rust
use crawrad_project::sidecar_notes::AnchorMatch;

let m = AnchorMatch {
    note_id: "n1".into(),
    line_number: 1,
    is_region_begin: false,
    is_region_end: false,
    is_comment: false,
};
assert_eq!(m.line_number, 1);
```
