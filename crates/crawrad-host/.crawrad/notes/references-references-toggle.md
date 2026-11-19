---
id: references-references-toggle
kind: doc
target_file: src/references.rs
line_hint: 149
author: 
created: 
title: "References::toggle"
tags: []
signature: "pub fn toggle (& mut self , group : usize)"
---

A file's group opens or closes.

# Examples

```rust
use crawrad_host::references::{Hit, References};
use std::path::PathBuf;

let mut refs = References::default();
let hit = Hit::new(0, 4, 7, "let foo = 1;".into());
refs.show("foo", vec![(PathBuf::from("src/main.rs"), hit)], &[], ".designer.rs");
refs.toggle(0);
assert!(!refs.groups[0].open);
```
