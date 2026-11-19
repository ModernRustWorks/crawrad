---
id: canvas-atlas-pictureatlas-get-or-insert
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 140
author: 
created: 
title: "PictureAtlas::get_or_insert"
tags: []
signature: "pub fn get_or_insert (& mut self , bytes : & [u8]) -> Option < AtlasEntry >"
---

Returns the entry for `bytes`, decoding and packing if not present.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

let mut atlas = PictureAtlas::new();
assert_eq!(atlas.get_or_insert(&[]), None);
```
