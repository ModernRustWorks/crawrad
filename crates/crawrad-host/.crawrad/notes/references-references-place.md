---
id: references-references-place
kind: doc
target_file: src/references.rs
line_hint: 190
author: 
created: 
title: "References::place"
tags: []
signature: "pub fn place (& self , group : usize , hit : usize) -> Option < Place >"
---

Where an occurrence is: the caret goes on the name.

# Examples

```rust
use crawrad_host::references::{Hit, References};
use std::path::PathBuf;

let mut refs = References::default();
let hit = Hit::new(0, 4, 7, "let foo = 1;".into());
refs.show("foo", vec![(PathBuf::from("src/main.rs"), hit)], &[], ".designer.rs");
assert!(refs.place(0, 0).is_some());
```
