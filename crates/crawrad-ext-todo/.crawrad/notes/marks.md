---
id: marks
kind: doc
target_file: src/lib.rs
line_hint: 150
author: 
created: 
title: "marks"
tags: []
signature: "pub fn marks (root : & Path , tagged : & [Todo] , before : & [Todo]) -> Vec < SetLineMarksParams >"
---

Builds editor line mark parameters for tagged comments.

The marks of each file that has (or had) a tagged comment: a file that
lost its last one gets an empty list, which clears it in the editor.

# Examples

```rust
use std::path::Path;
use crawrad_ext_todo::marks;

let params = marks(Path::new("/root"), &[], &[]);
assert!(params.is_empty());
```
