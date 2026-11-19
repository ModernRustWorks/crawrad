---
id: snippet-placed
kind: doc
target_file: src/snippet.rs
line_hint: 170
author: 
created: 
title: "placed"
tags: []
signature: "pub fn placed (text : & str , stops : & [(usize , usize)] , (line , byte) : (usize , usize) , lines : & [String] ,) -> Vec < [(usize , usize) ; 2] >"
---

Calculates document line and byte positions for expanded snippet tabstops.

The stops of `text` written at (`line`, `byte`), as (line, byte) on the
document's `lines` as they landed. The editor re-indents the lines after the first of
a multi-line insert (measured: the `match` body and its `}` move), so a
stop on a later line is shifted by that line's change of indentation and
kept inside it — never a byte past its end.

# Examples

```rust
use crawrad_insight::snippet::placed;

let stops = vec![(0, 4)];
let lines = vec!["test".to_string()];
let p = placed("test", &stops, (0, 0), &lines);
assert_eq!(p.len(), 1);
```
