---
id: navigation-aids-rows
kind: doc
target_file: src/navigation.rs
line_hint: 48
author: 
created: 
title: "Aids::rows"
tags: []
signature: "pub fn rows (& self , files : & [PathBuf]) -> Vec < String >"
---

Returns display labels for Quick Open rows based on available choices.

The rows of Quick Open: Link item…'s items, the definitions (one
per file of `files`), or the files' names.

# Examples

```rust
use crawrad_host::navigation::Aids;
use std::path::PathBuf;

let aids = Aids::default();
let files = vec![PathBuf::from("src/lib.rs")];
assert_eq!(aids.rows(&files), vec!["lib.rs".to_string()]);
```
