---
id: source-not-on-disk-yet
kind: doc
target_file: src/source.rs
line_hint: 139
author: 
created: 
title: "NOT_ON_DISK_YET"
tags: []
signature: "pub const NOT_ON_DISK_YET : & str = \"// ---------------------------------------------------------------------\n     // NOT ON DISK YET — this is the file `Generate` will create for you.\n     //\n     // It is planned from the form you are editing right now, so it already\n     // has the header and the marker where your event handlers go. Wire an\n     // event in the Object Inspector, or hit Generate, and it lands on disk\n     // exactly like this.\n     // ---------------------------------------------------------------------\n\n\""
---

The warning that goes ABOVE the skeleton.

It is a Rust comment on purpose: the view is a code editor, and a
loose prose banner at the top would break first-line highlighting.

# Examples

```rust
use crawrad_project::source::NOT_ON_DISK_YET;

assert!(NOT_ON_DISK_YET.contains("NOT ON DISK YET"));
```
