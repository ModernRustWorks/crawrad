---
id: breadcrumb-path-items
kind: doc
target_file: src/breadcrumb.rs
line_hint: 43
author: 
created: 
title: "path_items"
tags: []
signature: "pub fn path_items (file : & Path , root : Option < & Path >) -> Vec < String >"
---

Returns the path segments of `file` for the editor's breadcrumb.

The path segments of `file` for the editor's breadcrumb — ONE rule (#604;
two updaters each had their own, one joined with `>` and the other with
`›`): relative to `root` when the file is inside it, else its last four
parts; never the drive. The first segment carries the file's mark (🦀 for
Rust, 📄 otherwise).

# Examples

```rust
use std::path::Path;
use crawrad_host::breadcrumb::path_items;

let items = path_items(Path::new("src/lib.rs"), None);
assert_eq!(items, vec!["🦀 src", "lib.rs"]);
```
