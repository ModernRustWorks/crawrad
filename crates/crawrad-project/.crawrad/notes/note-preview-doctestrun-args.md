---
id: note-preview-doctestrun-args
kind: doc
target_file: src/note_preview.rs
line_hint: 149
author: 
created: 
title: "DoctestRun::args"
tags: []
signature: "pub fn args (& self) -> Vec < String >"
---

The arguments after `cargo`.

# Examples

```rust
use crawrad_project::note_preview::DoctestRun;

let run = DoctestRun::new("my_crate", None, "my_fn");
assert!(run.args().contains(&"--doc".to_string()));
```
