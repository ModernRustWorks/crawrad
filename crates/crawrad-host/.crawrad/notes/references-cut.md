---
id: references-cut
kind: doc
target_file: src/references.rs
line_hint: 260
author: 
created: 
title: "cut"
tags: []
signature: "pub fn cut (h : & Hit) -> (String , String , String)"
---

Cuts a line into prefix, matched word, and suffix snippets.

The line in three: before the name (indent dropped, at most [`BEFORE`]
characters, cut from the left), the name, after it (at most [`AFTER`]).

# Examples

```rust
use crawrad_host::references::{cut, Hit};

let hit = Hit::new(0, 4, 7, "let foo = 1;".into());
let (before, word, after) = cut(&hit);
assert_eq!(word, "foo");
```
