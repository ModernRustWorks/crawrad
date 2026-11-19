---
id: note-inlinednote
kind: doc
target_file: src/note.rs
line_hint: 170
author: 
created: 
title: "InlinedNote"
tags: []
signature: "pub struct InlinedNote"
---

A note successfully inlined back into the Rust source code.

# Examples

```rust
use crawrad_cli::note::InlinedNote;

let note = InlinedNote {
    id: "intro".to_string(),
    file: "src/lib.rs".to_string(),
    line: 1,
};
assert_eq!(note.line, 1);
```
