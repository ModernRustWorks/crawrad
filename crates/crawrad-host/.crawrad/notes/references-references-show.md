---
id: references-references-show
kind: doc
target_file: src/references.rs
line_hint: 103
author: 
created: 
title: "References::show"
tags: []
signature: "pub fn show (& mut self , symbol : & str , hits : Vec < (PathBuf , Hit) > , roots : & [PathBuf] , generated_suffix : & str ,)"
---

Populates the references panel with search results grouped by file.

The answer for `symbol`: files by path, occurrences by line, every
group open. `roots` and `generated_suffix` decide the marks.

# Examples

```rust
use crawrad_host::references::{Hit, References};
use std::path::PathBuf;

let mut refs = References::default();
let hit = Hit::new(0, 4, 7, "let foo = 1;".into());
refs.show("foo", vec![(PathBuf::from("src/main.rs"), hit)], &[], ".designer.rs");
assert_eq!(refs.total(), 1);
```
