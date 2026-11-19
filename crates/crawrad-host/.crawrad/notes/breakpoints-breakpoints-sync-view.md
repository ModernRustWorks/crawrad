---
id: breakpoints-breakpoints-sync-view
kind: doc
target_file: src/breakpoints.rs
line_hint: 182
author: 
created: 
title: "Breakpoints::sync_view"
tags: []
signature: "pub fn sync_view (& mut self , file : & Path , view_owner : Option < & Path > , view_lines : & [usize] ,) -> ViewSync"
---

Reconciles the editor's view of `file` with the session, once a frame.

`view_owner` is the file whose breakpoints the view holds and
`view_lines` its **zero-based** lines, as the editor counts them. A view
that holds another file (or none) gets this file's breakpoints; a view
that holds this one and differs was clicked or edited, and wins.

# Examples

```rust
use std::path::Path;
use crawrad_host::breakpoints::{Breakpoints, ViewSync};

let mut bp = Breakpoints::default();
let file = Path::new("src/lib.rs");
let sync = bp.sync_view(file, Some(file), &[]);
assert_eq!(sync, ViewSync::Same);
```
